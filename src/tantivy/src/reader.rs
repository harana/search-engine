use std::borrow::Cow;
use std::cmp::Reverse;
use std::sync::Arc;

use harana_common::anyhow::*;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::tantivy::collector::{Count, TopDocs};
use harana_common::tantivy::query::{Query, TermQuery};
use harana_common::tantivy::schema::{Field, FieldType, IndexRecordOption, OwnedValue, Schema, Value};
use harana_common::tantivy::{DateTime, DocAddress, DocId, Document, IndexReader, Order, ReloadPolicy, Score, Searcher, SegmentReader, TantivyDocument, Term};
use harana_common::hashbrown::HashMap;
use harana_common::thread_pools::{execute_operation, SEARCH_POOL};

use harana_common::log::info;

use crate::helpers::{AsScore, Validate};
use crate::query::{DocumentId, QueryBuilder, QuerySelector};
use crate::schema::SchemaContext;
use crate::structures::{DocumentHit, IndexContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ReaderContext {}

impl Validate for ReaderContext {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    fn validate_with_schema(&self, _schema: &Schema) -> Result<()> {
        Ok(())
    }
}

/// A given query payload that describes how the reader should
/// search the index.
#[derive(Debug, Deserialize)]
#[serde(crate = "self::serde")]
pub struct QueryPayload {
    /// The query(s) itself.
    pub query: QuerySelector,

    /// The amount of results to limit by.
    #[serde(default = "QueryPayload::default_limit")]
    pub limit: usize,

    /// The amount of documents to skip before getting the results.
    #[serde(default)]
    pub offset: usize,

    /// A specified field to order results by, this defaults to the
    /// score of the indexed documents (relevancy).
    pub order_by: Option<String>,

    /// How to order the data (asc/desc).
    pub order: Order,

    /// How to sort the data (asc/desc).
    #[serde(default)]
    pub sort: Sort,
}

impl QueryPayload {
    fn default_limit() -> usize {
        20
    }
}

/// What order to sort the returned data.
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    /// Sort the data in ascending order.
    Asc,

    /// Sort the data in descending order. (Default)
    Desc,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Desc
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "self::serde")]
pub struct QueryResults {
    /// The retrieved documents.
    pub hits: Vec<DocumentHit>,

    /// The total amount of documents matching the search
    pub count: usize,

    /// The amount of time taken to search in seconds.
    pub time_taken: f32,
}

impl QueryResults {
    #[inline]
    pub fn len(&self) -> usize {
        self.hits.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.hits.len() == 0
    }
}

/// Attaches an order by clause to the collector.
///
/// This collected the values with be returned in the order according to the
/// given field value.
fn order_and_search<R: AsScore + harana_common::tantivy::fastfield::FastValue>(
    searcher: &Searcher,
    field: Field,
    query: Box<dyn Query>,
    order: Order,
    limit: usize,
    offset: usize
) -> Result<(Vec<(R, DocAddress)>, usize)> {
    let field_name = searcher.schema().get_field_name(field);
    let collector = TopDocs::with_limit(limit + offset);
    let collector = collector.order_by_fast_field(field_name, order);
    let collector = (collector, Count);

    let (result_addresses, count) = searcher
        .search(&query, &collector)
        .map_err(Error::from)?;

    let results = result_addresses
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();

    Ok((results, count))
}

macro_rules! execute_staged_search {
    ($query:expr, $searcher:expr, $schema:expr, $collector:expr, $limit:expr, $offset:expr) => {{
        let collector = ($collector, Count);

        let (results, count) = $searcher
            .search(&$query, &collector)
            .map_err(Error::from)?;

        let results = results
            .into_iter()
            .skip($offset)
            .take($limit)
            .collect::<Vec<_>>();

        Ok((results, count))
    }};
}

/// Performs the search operation and processes the returned results.
fn process_search<S: AsScore>(
    ctx: &SchemaContext,
    searcher: &Searcher,
    top_docs: Vec<(S, DocAddress)>,
) -> Result<Vec<DocumentHit>> {
    let mut hits = Vec::with_capacity(top_docs.len());
    for (ratio, ref_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(ref_address)?;
        let mut doc = retrieved_doc.to_named_doc(searcher.schema());
        let id = doc.0
            .remove("_id")
            .ok_or_else(|| Error::msg("document has been missed labeled (missing primary key '_id'), the dataset is invalid"))?;

        if let OwnedValue::U64(doc_id) = id[0] {
            hits.push(DocumentHit::from_tantivy_document(
                ctx,
                doc_id,
                doc,
                ratio.as_score(),
            ));
        } else {
            return Err(Error::msg("document has been missed labeled (missing identifier tag), the dataset is invalid"));
        }
    }

    Ok(hits)
}

/// Orders the search results by the given field with a given sort (ASC, DESC)
///
/// This function is super messy just because of all the type inference
/// so any contributions to clean this up would be very appreciated.
#[allow(clippy::too_many_arguments)]
fn order_and_sort(
    sort: Sort,
    field: Field,
    query: Box<dyn Query>,
    ctx: &SchemaContext,
    searcher: &'static Searcher,
    order: Order,
    limit: usize,
    offset: usize
) -> Result<(Vec<DocumentHit>, usize)> {
    let field_name = searcher.schema().get_field_name(field);

    let is_multi_value = ctx
        .multi_value_fields()
        .contains(field_name);

    if is_multi_value {
        return Err(anyhow!(
            "multi-value fields cannot be used to sort results see issue #70"
        ));
    }

    let field_type = searcher.schema().get_field_entry(field).field_type();
    if let Sort::Desc = sort {
        return match field_type {
            FieldType::I64(_) => {
                let out: (Vec<(i64, DocAddress)>, usize) = order_and_search(&searcher, field, query, order, limit, offset)?;
                Ok((process_search(ctx, &searcher, out.0)?, out.1))
            },
            FieldType::U64(_) => {
                let out: (Vec<(u64, DocAddress)>, usize) = order_and_search(&searcher, field, query, order, limit, offset)?;
                Ok((process_search(ctx, &searcher, out.0)?, out.1))
            },
            FieldType::F64(_) => {
                let out: (Vec<(f64, DocAddress)>, usize) = order_and_search(&searcher, field, query, order, limit, offset)?;
                Ok((process_search(ctx, &searcher, out.0)?, out.1))
            },
            FieldType::Date(_) => {
                let out: (Vec<(DateTime, DocAddress)>, usize) = order_and_search(&searcher, field, query, order, limit, offset)?;
                Ok((process_search(ctx, &searcher, out.0)?, out.1))
            },
            _ => Err(Error::msg("field is not a fast field")),
        };
    }

    let collector = TopDocs::with_limit(limit + offset);

    let out = match field_type {
        FieldType::I64(_) => {
            let collector =
                collector.custom_score(move |segment_reader: &SegmentReader| {
                    let reader = segment_reader
                        .fast_fields()
                        .i64(field_name)
                        .expect("field exists");

                    move |doc: DocId| {
                        let value: i64 = reader.values_for_doc(doc).last().unwrap();
                        Reverse(value)
                    }
                });

            let out: (Vec<(Reverse<i64>, DocAddress)>, usize) = execute_staged_search!(
                query, searcher, schema, collector, limit, offset
            )?;
            (process_search(ctx, &searcher, out.0)?, out.1)
        },
        FieldType::U64(_) => {
            let collector =
                collector.custom_score(move |segment_reader: &SegmentReader| {
                    let reader = segment_reader
                        .fast_fields()
                        .u64(field_name)
                        .expect("field exists");

                    move |doc: DocId| {
                        let value: u64 = reader.values_for_doc(doc).last().unwrap();
                        Reverse(value)
                    }
                });

            let out: (Vec<(Reverse<u64>, DocAddress)>, usize) = execute_staged_search!(
                query, searcher, schema, collector, limit, offset
            )?;
            (process_search(ctx, &searcher, out.0)?, out.1)
        },
        FieldType::F64(_) => {
            let collector =
                collector.custom_score(move |segment_reader: &SegmentReader| {
                    let reader = segment_reader
                        .fast_fields()
                        .f64(field_name)
                        .expect("field exists");

                    move |doc: DocId| {
                        let value: f64 = reader.values_for_doc(doc).last().unwrap();
                        Reverse(value)
                    }
                });

            let out: (Vec<(Reverse<f64>, DocAddress)>, usize) = execute_staged_search!(
                query, searcher, schema, collector, limit, offset
            )?;
            (process_search(ctx, &searcher, out.0)?, out.1)
        },
        FieldType::Date(_) => {
            let collector =
                collector.custom_score(move |segment_reader: &SegmentReader| {
                    let reader  = segment_reader
                        .fast_fields()
                        .date(field_name)
                        .expect("field exists");

                    move |doc: DocId| {
                        let value: DateTime = reader.values_for_doc(doc).last().unwrap();
                        Reverse(value)
                    }
                });

            let out: (Vec<(Reverse<DateTime>, DocAddress)>, usize) = execute_staged_search!(
                query, searcher, schema, collector, limit, offset
            )?;
            (process_search(ctx, &searcher, out.0)?, out.1)
        },
        _ => return Err(Error::msg("field is not a fast field")),
    };

    Ok(out)
}

/// The reader of the given index.
///
/// This manages all searches on the index which encompasses the concurrency
/// limiters and thread pool execution.
///
/// Each index should only have one `Reader` instance.
#[derive(Clone)]
pub struct Reader {
    index_reader: Cow<'static, IndexReader>,

    index_name: Cow<'static, str>,

    schema_ctx: Cow<'static, SchemaContext>,

    /// The query factory system.
    query_handler: Arc<QueryBuilder>,
}

impl Reader {
    /// Creates a new reader from the given index context.
    // TODO add-back #[instrument(name = "index-reader", skip_all)]
    pub async fn create(ctx: &IndexContext) -> Result<Self> {
        let reader: IndexReader = ctx
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let query_ctx = ctx.query_ctx.clone();
        let query_handler = QueryBuilder::new(
            query_ctx,
            ctx.stop_words.clone(),
            ctx.correction_manager.clone(),
            ctx.synonyms.clone(),
            &ctx.index
        );
        info!(
            "query builder constructed with config: fast-fuzzy={} strip-stop-words={}.",
            ctx.query_ctx.use_fast_fuzzy, ctx.query_ctx.strip_stop_words,
        );

        Ok(Self {
            index_reader: Cow::Owned(reader.clone()),
            index_name: Cow::Owned(ctx.name()),
            schema_ctx: Cow::Owned(ctx.schema_ctx.clone()),
            query_handler: Arc::new(query_handler),
        })
    }

    /// Gets a list of suggested corrections based off of the index corpus.
    pub fn get_corrected_query_hint(&self, query: &str) -> String {
        self.query_handler.get_corrected_query_hint(query)
    }

    /// Gets a singular document from the given id.
    ///
    /// If no document is found an error is raised without context.
    // TODO add-back #[instrument(name = "document-fetcher", skip(self), fields(index = %self.index_name))]
    pub async fn get_document(&self, id: DocumentId) -> Result<DocumentHit> {
        let id_field = self.query_handler.id_field();
        let searcher = self.index_reader.searcher();

        let document =
            execute_operation(&SEARCH_POOL, move |_| {
                let qry = TermQuery::new(
                    Term::from_field_u64(id_field, id),
                    IndexRecordOption::Basic,
                );

                let mut results = searcher.search(
                    &qry,
                    &TopDocs::with_limit(1)
                )?;
                if results.is_empty() {
                    return Err(Error::msg(format!(
                        "no document exists with id: '{}'",
                        id
                    )));
                }

                let (_, addr) = results.remove(0);
                let doc: TantivyDocument = searcher.doc(addr)?;
                let schema = searcher.schema();
                Ok(doc.to_named_doc(&schema))
            }).await?;

        Ok(DocumentHit::from_tantivy_document(
            self.schema_ctx.as_ref(),
            id,
            document,
            Some(1.0),
        ))
    }

    /// Searches the index reader with the given query payload.
    ///
    /// The payload determines the behaviour of the query results.
    /// The actual behaviour of how a query is built is upto the query handler
    /// which will parse and interpret the given data.
    // TODO add-back #[instrument(name = "document-self.searcher", skip_all, fields(index = %self.index_name))]
    pub async fn search(&self, qry: QueryPayload) -> Result<QueryResults> {
        let start = std::time::Instant::now();
        let limit = qry.limit;
        let sort = qry.sort;
        let order_by = qry.order_by;
        let order = qry.order;
        let offset = qry.offset;
        let queries = self.query_handler.build_query(&self.index_reader, qry.query).await?;
        let ctx = self.schema_ctx.clone();
        let searcher = self.index_reader.searcher();

        let (hits, count) = {
            execute_operation(&SEARCH_POOL, move |_| {
                let order_by = order_by.map(|v| searcher.schema().get_field(&v));

                let (hits, count) = if let Some(Result::Ok(field)) = order_by {
                    order_and_sort(
                        sort,
                        field,
                        queries,
                        ctx.as_ref(),
                        unsafe { std::mem::transmute(searcher.clone()) },
                        order,
                        limit,
                        offset
                    )?
                } else {
                    let collector = TopDocs::with_limit(limit + offset);
                    let out: (Vec<(Score, DocAddress)>, usize) = execute_staged_search!(
                        queries, &searcher, schema, collector, limit, offset
                    )?;

                    (
                        process_search(ctx.as_ref(), &searcher, out.0)?,
                        out.1,
                    )
                };

                Ok((hits, count))
            }).await?
        };

        let elapsed = start.elapsed();
        Ok(QueryResults {
            time_taken: elapsed.as_secs_f32(), // filled in by handler later
            hits,
            count,
        })
    }

    pub fn get_synonyms(&self) -> HashMap<String, Box<[String]>> {
        self.query_handler.synonyms()
    }

    pub fn get_stop_words(&self) -> Vec<String> {
        self.query_handler.stop_words()
    }

    /// This forces the reader to reload after a commit.
    pub fn force_reload(&self) -> Result<()> {
        self.index_reader.reload().map_err(|_error| {
            Error::msg(format!("Failed to reload index"))
        })
    }

    /// This should not be used for general things.
    ///
    /// This is an internal export to allow the writer
    /// to have access to the segment reader information.
    pub fn get_searcher(&self) -> Searcher {
        self.index_reader.searcher()
    }
}