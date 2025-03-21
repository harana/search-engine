use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use harana_common::anyhow::{anyhow, Context, Error, Result};
use harana_common::serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use harana_common::serde::de::{MapAccess, SeqAccess, Visitor};
use harana_common::serde::{self, Deserialize, Deserializer, Serialize};
use harana_common::tantivy::schema::{Facet as SchemaFacet, OwnedValue};
use harana_common::tantivy::schema::{FacetParseError, Field, FieldType, Schema, Value};
use harana_common::tantivy::{DateTime, Document as InternalDocument, Document, Index, Score, TantivyDocument};
use harana_common::tantivy::time::format_description::well_known::Rfc3339;
use harana_common::tantivy::time::OffsetDateTime;
use harana_common::hashbrown::HashMap;
use harana_common::serde_json;
use harana_common::tantivy::schema::document::OwnedValue::*;
use crate::corrections::{SymSpellCorrectionManager, SymSpellManager};
use crate::helpers::{Calculated, Validate};
use crate::query::QueryContext;
use crate::reader::ReaderContext;
use crate::schema::{SchemaContext, ID_FIELD, FieldDeclaration};
use crate::stop_words::StopWordManager;
use crate::storage::{OpenType, SledBackedDirectory, StorageBackend};
use crate::synonyms::SynonymsManager;
use crate::tokenizer::SimpleUnicodeTokenizer;
use crate::writer::WriterContext;

/// The possible index storage backends.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
    /// Store the index in a temporary directory.
    TempDir,

    /// Store the index in a file system store.
    FileSystem,
}

fn add_boost_fields(
    schema: &Schema,
    boost_fields: &HashMap<String, Score>,
    fields: &[Field],
    fields_with_boost: &mut Vec<(Field, Score)>,
) {
    for field in fields {
        let name = schema.get_field_name(*field);
        let boost = if let Some(data) = boost_fields.get(name) {
            *data
        } else {
            0f32
        };

        fields_with_boost.push((*field, boost));
    }
}

/// A given index declaration that describes the behaviour of a new index.
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct IndexDeclaration {
    /// The name of the index.
    pub name: String,

    /// The storage type used to store index data.
    pub storage_type: StorageType,

    #[serde(flatten)]
    schema_ctx: SchemaContext,

    #[serde(flatten)]
    reader_ctx: ReaderContext,

    #[serde(flatten)]
    writer_ctx: WriterContext,

    /// If set to true, this switches Tantivy's default query parser
    /// behaviour to use AND instead of OR.
    #[serde(default)]
    pub set_conjunction_by_default: bool,

    /// Whether or not to use the fast fuzzy system or not.
    ///
    /// The fast fuzzy system must be enabled on the server overall
    /// for this feature.
    #[serde(default)]
    pub use_fast_fuzzy: bool,

    /// Whether or not to strip out stop words in fuzzy queries.
    ///
    /// This only applies to the fast-fuzzy query system.
    #[serde(default)]
    pub strip_stop_words: bool,
}

impl Validate for IndexDeclaration {
    fn validate(&self) -> Result<()> {
        self.writer_ctx.validate()?;
        self.reader_ctx.validate()?;
        self.schema_ctx.validate()?;

        Ok(())
    }
}

impl IndexDeclaration {
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_fields(&mut self, fields: HashMap<String, FieldDeclaration>) {
        self.schema_ctx.fields.extend(fields);
    }

    /// Builds IndexContext from the declaration, applying any validation in
    /// the process.
    // TODO add-back #[instrument(name = "index-setup", skip(self), fields(index = %self.name))]
    pub fn create_context(&self, index_path: &Path, index_passphrase: String) -> Result<IndexContext> {
        self.validate()?;

        let mut schema_ctx = self.schema_ctx.clone();
        schema_ctx.calculate_once()?;

        let open = match self.storage_type {
            StorageType::TempDir => OpenType::TempFile,
            StorageType::FileSystem => OpenType::Dir(index_path.join(self.name.as_str()), index_passphrase.clone())
        };

        let dir = SledBackedDirectory::new_with_root(&open)?;
        let does_exist = Index::exists(&dir).with_context(|| {
            format!("failed to check for existing index {:?}", &open)
        })?;

        let index = if does_exist {
            Index::open(dir.clone())
        } else {
            Index::open_or_create(dir.clone(), schema_ctx.as_tantivy_schema())
        }?;

        let schema = index.schema();
        schema_ctx.validate_with_schema(&schema)?;

        let query_context = {
            let default_fields = schema_ctx.get_search_fields(&schema);
            let fuzzy_fields = schema_ctx.get_fuzzy_search_fields(&schema);

            let mut default_fields_with_boost = Vec::with_capacity(default_fields.len());
            add_boost_fields(
                &schema,
                schema_ctx.boost_fields(),
                &default_fields,
                &mut default_fields_with_boost,
            );

            let mut fuzzy_fields_with_boost = Vec::with_capacity(fuzzy_fields.len());
            add_boost_fields(
                &schema,
                schema_ctx.boost_fields(),
                &fuzzy_fields,
                &mut fuzzy_fields_with_boost,
            );

            QueryContext {
                id_field: schema.get_field(ID_FIELD).expect("get pk"),
                set_conjunction_by_default: self.set_conjunction_by_default,
                use_fast_fuzzy: self.use_fast_fuzzy,
                strip_stop_words: self.strip_stop_words,
                default_search_fields: default_fields_with_boost,
                fuzzy_search_fields: fuzzy_fields_with_boost,
            }
        };

        let corrections = Arc::new(SymSpellManager::new());
        let storage = StorageBackend::using_conn(dir);

        index
            .tokenizers()
            .register("default", SimpleUnicodeTokenizer::default());

        Ok(IndexContext {
            name: self.name.clone(),
            path: index_path.to_path_buf(),
            passphrase: index_passphrase,
            storage,
            correction_manager: corrections,
            index,
            schema_ctx: schema_ctx.clone(),
            reader_ctx: self.reader_ctx.clone(),
            writer_ctx: self.writer_ctx,
            query_ctx: query_context,
            fuzzy_search_fields: schema_ctx.get_fuzzy_search_fields(&schema),
            synonyms: SynonymsManager::init(),
            stop_words: StopWordManager::init()?,
        })
    }
}

#[derive(Debug)]
pub struct IndexContext {
    /// The name of the index.
    pub name: String,

    // The path of the index.
    pub path: PathBuf,
    
    // The passphrase of ihe index.
    pub passphrase: String,
    
    /// An SQLite DB instance used for storing engine state.
    pub storage: StorageBackend,

    /// The index's custom stop words.
    pub stop_words: StopWordManager,

    /// The index's custom synonym relations.
    pub synonyms: SynonymsManager,

    /// The index's fast-fuzzy pre-processor.
    pub correction_manager: SymSpellCorrectionManager,

    /// The tantivy Index.
    pub index: Index,

    /// The context for the readers.
    pub schema_ctx: SchemaContext,

    /// The context for the readers.
    pub reader_ctx: ReaderContext,

    /// The context for the writer actor.
    pub writer_ctx: WriterContext,

    /// The context for the query handler.
    pub query_ctx: QueryContext,

    /// All search fields used for fuzzy searching.
    ///
    /// This is only TEXT / STRING fields.
    pub fuzzy_search_fields: Vec<Field>,
}

impl IndexContext {
    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    #[inline]
    pub fn passphrase(&self) -> String {
        self.passphrase.clone()
    }

    /// Get the schema of the index.
    #[inline]
    pub fn schema(&self) -> Schema {
        self.index.schema()
    }

    /// Gets the fuzzy-query fields.
    #[inline]
    pub fn fuzzy_search_fields(&self) -> &Vec<Field> {
        self.fuzzy_search_fields.as_ref()
    }
}

/// A document value that can be processed by tantivy.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "self::serde")]
pub enum DocumentValue {
    /// A signed 64 bit integer.
    I64(i64),

    /// A 64 bit floating point number.
    F64(f64),

    /// A unsigned 64 bit integer.
    U64(u64),

    /// A datetime field serialized as string.
    Datetime(String),

    /// A boolean field
    Boolean(bool),

    /// A text field.
    Text(String),
}

impl DocumentValue {

    #[inline]
    pub fn from(value: OwnedValue) -> Self {
        match value {
            Str(v) => DocumentValue::Text(v.to_string()),
            PreTokStr(v) => DocumentValue::Text(v.clone().text),
            U64(v) => DocumentValue::U64(v),
            I64(v) => DocumentValue::I64(v),
            F64(v) => DocumentValue::F64(v),
            Bool(v) => DocumentValue::Boolean(v),
            Date(v) => {
                let utc = v.into_utc();
                let date = utc.format(&Rfc3339).unwrap_or_else(|_| utc.to_string());
                DocumentValue::Datetime(date)
            },
            Facet(v) => DocumentValue::Text(v.to_string()),
            Bytes(v) => DocumentValue::Text(format!("{:?}", v)),
            IpAddr(v) => DocumentValue::Text(v.to_string()),
            Null => DocumentValue::Text("".to_string()),
            Array(_) => todo!(),
            Object(_) => todo!(),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::I64(v) => v.to_string(),
            Self::F64(v) => v.to_string(),
            Self::U64(v) => v.to_string(),
            Self::Boolean(v) => v.to_string(),
            Self::Datetime(v) => v.to_string(),
            Self::Text(v) => v.to_string(),
        }
    }
}

impl TryInto<String> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `String`.
    fn try_into(self) -> Result<String> {
        Ok(self.as_string())
    }
}

impl TryInto<DateTime> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `DateTime`.
    fn try_into(self) -> Result<DateTime> {
        let v = match self.clone() {
            Self::I64(v) => {
                let offset = OffsetDateTime::from_unix_timestamp(v)
                    .map_err(|_| Error::msg("invalid i64 timestamp given"))?;
                DateTime::from_utc(offset)
            },
            Self::U64(v) => {
                let offset = OffsetDateTime::from_unix_timestamp(v as i64)
                    .map_err(|_| Error::msg("invalid u64 timestamp given"))?;
                DateTime::from_utc(offset)
            },
            Self::F64(_) => {
                return Err(Error::msg(
                    "value cannot be interpreted as a datetime value",
                ))
            },
            Self::Datetime(v) => {
                let offset = OffsetDateTime::parse(&v, &Rfc3339).map_err(|_| {
                    Error::msg(
                        "cannot convert value into a datetime value, \
                            datetime should be formatted in RFC 3339, a u64 \
                            timestamp or a i64 timestamp",
                    )
                })?;

                DateTime::from_utc(offset)
            },
            Self::Boolean(_v) => {
                return Err(Error::msg(
                    "value cannot be interpreted as a datetime value",
                ))
            },
            Self::Text(v) => {
                if let Ok(ts) = self.clone().try_into() {
                    let dt: DateTime = Self::I64(ts).try_into()?;
                    return Ok(dt);
                }

                if let Ok(ts) = self.try_into() {
                    let dt: DateTime = Self::U64(ts).try_into()?;
                    return Ok(dt);
                }

                let offset =
                    OffsetDateTime::parse(&v, &Rfc3339).map_err(|_| {
                        Error::msg(
                            "cannot convert value into a datetime value, \
                            datetime should be formatted in RFC 3339, a u64 \
                            timestamp or a i64 timestamp",
                        )
                    })?;

                DateTime::from_utc(offset)
            },
        };

        Ok(v)
    }
}

impl TryInto<u64> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `u64`.
    fn try_into(self) -> Result<u64> {
        let v = match self {
            Self::I64(v) => v as u64,
            Self::F64(v) => v as u64,
            Self::U64(v) => v,
            Self::Boolean(_v) => {
                return Err(Error::msg(
                    "value cannot be interpreted as a u64 value",
                ))
            },
            Self::Datetime(v) => v
                .parse::<u64>()
                .map_err(|_| Error::msg("cannot convert value into u64 value"))?
            ,
            Self::Text(v) => v
                .parse::<u64>()
                .map_err(|_| Error::msg("cannot convert value into u64 value"))?,
        };

        Ok(v)
    }
}

impl TryInto<i64> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `i64`.
    fn try_into(self) -> Result<i64> {
        let v = match self {
            Self::I64(v) => v,
            Self::F64(v) => v as i64,
            Self::U64(v) => v as i64,
            Self::Boolean(_v) => {
                return Err(Error::msg("value cannot be interpreted as a i64 value"))
            },
            Self::Datetime(_) => {
                return Err(Error::msg("value cannot be interpreted as a i64 value"))
            },
            Self::Text(v) => v
                .parse::<i64>()
                .map_err(|_| Error::msg("cannot convert value into i64 value"))?,
        };

        Ok(v)
    }
}

impl TryInto<f64> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `f64`.
    fn try_into(self) -> Result<f64> {
        let v = match self {
            Self::I64(_) => {
                return Err(Error::msg("value cannot be interpreted as a f64 value"))
            },
            Self::F64(v) => v,
            Self::U64(_) => {
                return Err(Error::msg("value cannot be interpreted as a f64 value"))
            },
            Self::Boolean(_) => {
                return Err(Error::msg("value cannot be interpreted as a f64 value"))
            },
            Self::Datetime(_) => {
                return Err(Error::msg("value cannot be interpreted as a f64 value"))
            },
            Self::Text(v) => v
                .parse::<f64>()
                .map_err(|_| Error::msg("cannot convert value into f64 value"))?,
        };

        Ok(v)
    }
}

impl TryInto<bool> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `bool`.
    fn try_into(self) -> Result<bool> {
        let v = match self {
            Self::I64(_v) => {
                return Err(Error::msg("value cannot be interpreted as a bool value"))
            },
            Self::F64(_v) => {
                return Err(Error::msg("value cannot be interpreted as a bool value"))
            },
            Self::U64(_v) => {
                return Err(Error::msg("value cannot be interpreted as a bool value"))
            },
            Self::Boolean(v) => v,
            Self::Datetime(_dt) => {
                return Err(Error::msg("value cannot be interpreted as a bool value"))
            },
            Self::Text(v) => return match v.to_lowercase().as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(Error::msg("value cannot be interpreted as a bool value"))
            }
        };

        Ok(v)
    }
}


impl TryInto<SchemaFacet> for DocumentValue {
    type Error = Error;

    /// Attempts to convert the value into a `facet`.
    fn try_into(self) -> Result<SchemaFacet> {
        let facet: String = self.try_into()?;

        let facet = SchemaFacet::from_text(&facet).map_err(|e| {
            let FacetParseError::FacetParseError(e) = e;
            Error::msg(e)
        })?;

        Ok(facet)
    }
}

impl<'de> Deserialize<'de> for DocumentValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = DocumentValue;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string, int or float")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(DocumentValue::I64(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(DocumentValue::U64(v))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(DocumentValue::F64(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                if let Ok(_dt) = OffsetDateTime::parse(v, &Rfc3339) {
                    return Ok(DocumentValue::Datetime(v.to_string()));
                }

                Ok(DocumentValue::Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                if let Ok(_dt) = OffsetDateTime::parse(&v, &Rfc3339) {
                    return Ok(DocumentValue::Datetime(v.to_string()));
                }

                Ok(DocumentValue::Text(v))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

/// The possible formats for adding document values.
#[derive(Clone, Debug, Serialize)]
#[serde(crate = "self::serde")]
pub enum DocumentValueOptions {
    /// A singular document value.
    Single(DocumentValue),

    /// An array of document values.
    Multi(Vec<DocumentValue>),
}

impl DocumentValueOptions {

    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multi(v) => v.len(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'de> Deserialize<'de> for DocumentValueOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct DocumentValueOptionsVisitor;

        impl<'de> Visitor<'de> for DocumentValueOptionsVisitor {
            type Value = DocumentValueOptions;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(
                    "a string, int, float or a list of strings, ints or floats",
                )
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(DocumentValueOptions::Single(DocumentValue::I64(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(DocumentValueOptions::Single(DocumentValue::U64(v)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(DocumentValueOptions::Single(DocumentValue::F64(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                if let Ok(_dt) = OffsetDateTime::parse(v, &Rfc3339) {
                    return Ok(DocumentValueOptions::Single(DocumentValue::Datetime(v.to_string())));
                }

                Ok(DocumentValueOptions::Single(DocumentValue::Text(
                    v.to_owned(),
                )))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                if let Ok(_dt) = OffsetDateTime::parse(&v, &Rfc3339) {
                    return Ok(DocumentValueOptions::Single(DocumentValue::Datetime(v.to_string())));
                }

                Ok(DocumentValueOptions::Single(DocumentValue::Text(v)))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
            {
                Vec::deserialize(SeqAccessDeserializer::new(seq))
                    .map(DocumentValueOptions::Multi)
            }
        }

        deserializer.deserialize_any(DocumentValueOptionsVisitor)
    }
}

/// A key-value map matching the target index's schema.
#[derive(Debug)]
pub struct DocumentPayload(pub BTreeMap<String, DocumentValueOptions>);

impl DocumentPayload {
    pub fn parse_into_document(
        mut self,
        id: u64,
        schema: &Schema,
        ctx: &SchemaContext,
    ) -> Result<TantivyDocument> {

        let mut doc = TantivyDocument::new();
        let field = schema.get_field(ID_FIELD).unwrap();
        doc.add_u64(field, id);

        for (field_name, info) in ctx.fields() {
            let data = match self.0.remove(field_name) {
                Some(data) => {
                    if info.is_required() & data.is_empty() {
                        return Err(anyhow!(
                            "a required field ({:?}) must contain at least one value",
                            field_name
                        ));
                    }

                    data
                },
                None => {
                    if info.is_required() {
                        return Err(anyhow!(
                            "missing a required field {:?}",
                            field_name
                        ));
                    } else {
                        continue;
                    }
                },
            };

            // should never panic as `ctx.fields` is inline with schema.
            let field = schema.get_field(field_name).expect("get field");

            let entry = schema.get_field_entry(field);
            let field_type = entry.field_type();

            match data {
                DocumentValueOptions::Single(value) => {
                    Self::add_value(field_name, field, field_type, value, &mut doc)?
                },
                DocumentValueOptions::Multi(mut values) => {
                    if ctx.multi_value_fields().contains(field_name) {
                        for value in values {
                            Self::add_value(
                                field_name, field, field_type, value, &mut doc,
                            )?;
                        }
                    } else if let Some(value) = values.pop() {
                        Self::add_value(field_name, field, field_type, value, &mut doc)?;
                    }
                },
            };
        }

        Ok(doc)
    }

    fn add_value(
        key: &str,
        field: Field,
        field_type: &FieldType,
        value: DocumentValue,
        doc: &mut TantivyDocument,
    ) -> Result<()> {
        let value = match field_type {
            FieldType::U64(_) => U64(value.try_into()?),
            FieldType::I64(_) => I64(value.try_into()?),
            FieldType::F64(_) => F64(value.try_into()?),
            FieldType::Date(_) => {
                let value: DateTime = value.try_into()?;
                Date(value)
            },
            FieldType::Str(_) => {
                let value: String = value.try_into()?;
                Str(value)
            },
            FieldType::Facet(_) => {
                let facet: SchemaFacet = value.try_into()?;
                Facet(facet)
            },
            _ => {
                return Err(anyhow!(
                    "byte fields (field: {}) are not supported for document insertion",
                    key,
                ))
            },
        };

        doc.add_field_value(field, value);

        Ok(())
    }
}

impl<'de> Deserialize<'de> for DocumentPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct DocumentOptionsVisitor;

        impl<'de> Visitor<'de> for DocumentOptionsVisitor {
            type Value = DocumentPayload;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("A map of key-value pairs or a map of key-values.")
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
            {
                let mapping = BTreeMap::deserialize(MapAccessDeserializer::new(map))?;
                Ok(DocumentPayload(mapping))
            }
        }

        deserializer.deserialize_any(DocumentOptionsVisitor)
    }
}

/// The possible formats for uploading documents.
pub enum DocumentOptions {
    /// A singular document payload.
    Single(DocumentPayload),

    /// An array of documents acting as a bulk insertion.
    Many(Vec<DocumentPayload>),
}

impl<'de> Deserialize<'de> for DocumentOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct DocumentOptionsVisitor;

        impl<'de> Visitor<'de> for DocumentOptionsVisitor {
            type Value = DocumentOptions;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(
                    "A single `DocumentPayload` or a list of `DocumentPayload`s",
                )
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
            {
                Vec::deserialize(SeqAccessDeserializer::new(seq))
                    .map(DocumentOptions::Many)
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
            {
                DocumentPayload::deserialize(MapAccessDeserializer::new(map))
                    .map(DocumentOptions::Single)
            }
        }

        deserializer.deserialize_any(DocumentOptionsVisitor)
    }
}

/// A individual document returned from the index.
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "self::serde")]
pub struct DocumentHit {
    /// The document data itself.
    pub doc: BTreeMap<String, Option<DocumentValueOptions>>,

    /// The document id.
    ///
    /// This is a unique 64 bit integer that can be used
    /// to select other similar docs or the document itself.
    ///
    /// This is serialized to a string for language support.
    #[serde(with = "document_id_serializer")]
    pub document_id: u64,

    /// The computed score of the documents.
    pub score: Option<Score>,
}

impl DocumentHit {
    /// Converts a tantivy document into a document matching
    /// the given schema.
    pub fn from_tantivy_document(
        ctx: &SchemaContext,
        doc_id: u64,
        mut doc: harana_common::tantivy::schema::NamedFieldDocument,
        score: Option<Score>,
    ) -> Self {
        let mut compliant = BTreeMap::new();
        for (name, info) in ctx.fields() {
            let val = match doc.0.remove(name) {
                Some(mut val) => {
                    if info.is_multi() {
                        let dv = val.iter().map(|v| { DocumentValue::from(v.clone()) }).collect();
                        Some(DocumentValueOptions::Multi(dv))
                    } else {
                        val.pop().map(|v| {
                            DocumentValueOptions::Single(DocumentValue::from(v))
                        })
                    }
                },
                None => {
                    if info.is_multi() {
                        Some(DocumentValueOptions::Multi(vec![]))
                    } else {
                        None
                    }
                }
            };

            compliant.insert(name.clone(), val);
        }

        Self {
            doc: compliant,
            document_id: doc_id,
            score,
        }
    }
}

mod document_id_serializer {
    use harana_common::serde::Serializer;

    pub fn serialize<S>(document_id: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = document_id.to_string();
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod test_doc_value {
    use harana_common::anyhow::Result;

    use super::*;

    #[test]
    fn test_into_raw_values_from_string() -> Result<()> {
        let sample = DocumentValue::Text("124314".into());
        let res: Result<String> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<DateTime> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<u64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<i64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<f64> = sample.try_into();
        assert!(res.is_ok());

        let ts = OffsetDateTime::now_utc();
        let sample = DocumentValue::Text(ts.unix_timestamp().to_string());
        let res: Result<String> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<DateTime> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<u64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<i64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<f64> = sample.try_into();
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn test_into_raw_values_from_datetime() -> Result<()> {
        let ts = OffsetDateTime::now_utc();
        let datetime_str = ts.format(Format::Rfc3339).unwrap();
        let sample = DocumentValue::Datetime(datetime_str);
        let res: Result<String> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<DateTime> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<u64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<i64> = sample.clone().try_into();
        assert!(res.is_err());

        let res: Result<f64> = sample.try_into();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_into_raw_values_from_u64() -> Result<()> {
        let sample = DocumentValue::U64(45674);
        let res: Result<String> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<DateTime> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<u64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<i64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<f64> = sample.try_into();
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn test_into_raw_values_from_i64() -> Result<()> {
        let sample = DocumentValue::I64(-2354);
        let res: Result<String> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<DateTime> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<u64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<i64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<f64> = sample.try_into();
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn test_into_raw_values_from_f64() -> Result<()> {
        let sample = DocumentValue::F64(234234.234);
        let res: Result<String> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<DateTime> = sample.clone().try_into();
        assert!(res.is_err());

        let res: Result<u64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<i64> = sample.clone().try_into();
        assert!(res.is_ok());

        let res: Result<f64> = sample.try_into();
        assert!(res.is_ok());

        Ok(())
    }
}

#[cfg(test)]
mod test_context_builder {
    use harana_common::anyhow::Result;

    use super::*;

    #[test]
    fn test_build_context_expect_ok() -> Result<()> {
        let dec = serde_json::from_value::<IndexDeclaration>(serde_json::json!({
            "name": "test",

            "writer_buffer": 64000000,
            "writer_threads": 12,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true
                },
                "description": {
                    "type": "string",
                    "stored": false
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": false,
                   "fast": true
                },
            },
            "search_fields": [
                "title",
                "description"
            ],
        }))?;

        let _ctx = dec.create_context()?;

        Ok(())
    }

    #[test]
    fn test_non_string_search_fields_expect_err() -> Result<()> {
        let dec = serde_json::from_value::<IndexDeclaration>(serde_json::json!({
            "name": "test",

            "writer_buffer": 64000000,
            "writer_threads": 12,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true
                },
                "description": {
                    "type": "string",
                    "stored": false
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": false,
                   "fast": true
                },
            },
            "search_fields": [
                "title",
                "description",
                "count"
            ],
        }))?;

        let r = dec.create_context();
        assert!(r.is_err());

        Ok(())
    }

    #[test]
    fn test_unknown_search_fields_expect_err() -> Result<()> {
        let dec = serde_json::from_value::<IndexDeclaration>(serde_json::json!({
            "name": "test",

            "writer_buffer": 64000000,
            "writer_threads": 12,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true
                },
                "description": {
                    "type": "string",
                    "stored": false
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": false,
                   "fast": true
                },
            },
            "search_fields": [
                "title",
                "description",
                "ahhhhh"
            ],
        }))?;

        let r = dec.create_context();
        assert!(r.is_err());

        Ok(())
    }

    #[test]
    fn test_unknown_boost_fields_expect_err() -> Result<()> {
        let dec = serde_json::from_value::<IndexDeclaration>(serde_json::json!({
            "name": "test",

            "writer_buffer": 64000000,
            "writer_threads": 12,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true
                },
                "description": {
                    "type": "string",
                    "stored": false
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": false,
                   "fast": true
                },
            },
            "search_fields": [
                "title",
                "description",
                "count"
            ],

            "boost_fields": {
                "ahh": 0.1
            }
        }))?;

        let r = dec.create_context();
        assert!(r.is_err());

        Ok(())
    }

    #[test]
    fn test_fields() -> Result<()> {
        let dec: IndexDeclaration = serde_json::from_value(serde_json::json!({
            "name": "test",

            "writer_buffer": 64000000,
            "writer_threads": 12,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true
                },
                "description": {
                    "type": "string",
                    "stored": false
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": true,
                   "fast": true
                },
            },
            "search_fields": [
                "title",
                "description",
            ],
            "use_fast_fuzzy": true
        }))?;

        let res = dec.create_context();
        assert!(res.is_ok());

        let ctx = res.unwrap();
        assert_eq!(ctx.fuzzy_search_fields().len(), 2);

        Ok(())
    }
}