use std::collections::BTreeMap;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use harana_common::anyhow::Result;
use harana_common::tantivy::schema::Schema;
use harana_common::tantivy::{Order, Searcher};
use harana_common::hashbrown::HashMap;
use harana_common::log::info;
use harana_common::tokio;
use crate::query::{DocumentId, Occur, QueryData, QuerySelector};
use crate::reader::{QueryPayload, QueryResults};
use crate::structures::{DocumentHit, DocumentPayload, DocumentValue, DocumentValueOptions, IndexContext};
use crate::writer::WriterOp;
use crate::{reader, writer};

#[derive(Clone)]
pub struct Index(Arc<InternalIndex>);

impl Index {

    /// Creates a new index handler from the given index context.
    pub async fn create(ctx: IndexContext, remove_lock: bool) -> Result<Self> {
        if remove_lock {
            let name = ctx.name.to_string();
            let path = ctx.path
                .join(name.as_str())
                .join("data")
                .join(".tantivy-writer.lock");

            if path.exists() {
                remove_file(path).unwrap();
            }
        }

        let index = InternalIndex::create(ctx).await?;
        Ok(Self(Arc::new(index)))
    }

    pub fn name(&self) -> String {
        self.0._ctx.name.to_string()
    }

    /// Commits any changes to the index since the last commit.
    pub async fn commit(&self) -> Result<()> {
        info!("Committing index ..");
        self.0.commit().await
    }

    /// Discards any changes to the index since the last commit.
    pub async fn rollback(&self) -> Result<()> {
        self.0.rollback().await
    }

    /// Gets a list of suggested corrections based off of the index corpus.
    pub fn get_corrected_query_hint(&self, query: &str) -> String {
        self.0.get_corrected_query_hint(query)
    }

    /// Search the index for the given query.
    ///
    /// This returns a set of results ordered by their relevance according to
    /// the order or the score.
    pub async fn search(&self, qry: QueryPayload) -> Result<QueryResults> {
        self.0.search(qry).await
    }

    /// Get a single document via it's given id.
    pub async fn get_document(&self, doc_id: DocumentId) -> Result<DocumentHit> {
        self.0.get_document(doc_id).await
    }

    pub async fn get_documents_count(&self) -> Result<usize> {
        self.0.get_documents_count().await
    }

    /// Adds one or more documents to the index.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    pub async fn add_document(&self, document_id: DocumentId, payload: DocumentPayload) -> Result<()> {
        self.0.add_document(document_id, payload).await
    }

    /// Adds one or more documents to the index.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    pub async fn add_documents(&self, documents: Vec<(DocumentId, DocumentPayload)>) -> Result<()> {
        self.0.add_documents(documents).await
    }

    /// Deletes all documents from the index matching a given term(s).
    pub async fn delete_documents_where(
        &self,
        fields: BTreeMap<String, DocumentValueOptions>,
    ) -> Result<usize> {
        self.0.delete_documents_where(fields).await
    }

    /// Deletes a specific document
    pub async fn delete_document(&self, document_id: DocumentId) -> Result<()> {
        self.0.delete_document(document_id).await
    }

    /// Deletes all documents from the index matching a given term(s).
    pub async fn delete_documents_by_query(&self, qry: QueryPayload) -> Result<usize> {
        self.0.delete_by_query(qry).await
    }

    /// Updates document
    pub async fn update_document(&self, document_id: DocumentId, replacement_values: HashMap<String, DocumentValueOptions>) -> Result<()> {
        self.0.update_document(document_id, replacement_values).await
    }

    /// Updates document
    pub async fn update_document_multivalue(&self, document_id: DocumentId, field: String, value: DocumentValue, add: bool) -> Result<()> {
        self.0.update_document_multivalue(document_id, field, value, add).await
    }

    /// Deletes all documents from the index.
    pub async fn clear_documents(&self) -> Result<()> {
        self.0.clear_documents().await
    }

    /// Adds a set of stop words to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    pub async fn add_stop_words(&self, words: Vec<String>) -> Result<()> {
        self.0.add_stop_words(words).await
    }

    /// Get the current index stop words.
    pub fn get_stop_words(&self) -> Vec<String> {
        self.0.get_stop_words()
    }

    /// Adds a set of stop words to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    pub async fn remove_stop_words(&self, words: Vec<String>) -> Result<()> {
        self.0.remove_stop_words(words).await
    }

    /// Removes all stop words from the index.
    pub async fn clear_stop_words(&self) -> Result<()> {
        self.0.clear_stop_words().await
    }

    /// Adds a set of synonyms to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    pub async fn add_synonyms(&self, relations: Vec<String>) -> Result<()> {
        self.0.add_synonyms(relations).await
    }

    /// Get the current index synonyms.
    pub fn get_synonyms(&self) -> HashMap<String, Box<[String]>> {
        self.0.get_synonyms()
    }

    /// Adds a set of synonyms to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    pub async fn remove_synonyms(&self, words: Vec<String>) -> Result<()> {
        self.0.remove_synonyms(words).await
    }

    /// Removes all synonyms from the index.
    pub async fn clear_synonyms(&self) -> Result<()> {
        self.0.clear_synonyms().await
    }

    /// Shuts the index down waiting for all writer threads to finish.
    pub async fn shutdown(&self) -> Result<()> {
        self.0.shutdown().await
    }

    /// Shuts the index down removing any persistent data along with it.
    pub async fn destroy(&self) -> Result<()> {
        self.0.destroy().await
    }

    pub fn get_schema(&self) -> Schema {
        self.0._ctx.schema()
    }

    pub fn get_searcher(&self) -> Searcher {
        self.0.reader.get_searcher()
    }
}

struct InternalIndex {
    /// The name of the index.
    _ctx: IndexContext,

    /// The index reader handler
    reader: reader::Reader,

    /// A writer actor to handle the index writer.
    writer: writer::Writer,
}

impl InternalIndex {
    /// Creates a new index handler from the given index context.
    // TODO add-back #[instrument(name = "index-controller", skip(ctx), fields(index = %ctx.name))]
    async fn create(ctx: IndexContext) -> Result<Self> {
        info!("creating reader...");
        let reader = reader::Reader::create(&ctx).await?;

        info!("creating writer...");
        let writer = writer::Writer::create(&ctx, reader.clone())?;

        Ok(Self {
            _ctx: ctx,
            reader,
            writer,
        })
    }

    /// Commits any changes to the index since the last commit.
    async fn commit(&self) -> Result<()> {
        self.writer.send_op(WriterOp::Commit).await
    }

    /// Discards any changes to the index since the last commit.
    async fn rollback(&self) -> Result<()> {
        self.writer.send_op(WriterOp::Rollback).await
    }

    /// Gets a list of suggested corrections based off of the index corpus.
    pub fn get_corrected_query_hint(&self, query: &str) -> String {
        self.reader.get_corrected_query_hint(query)
    }

    /// Gets count of searchable documents in the index.
    pub async fn get_documents_count(&self) -> Result<usize> {
        if let Ok(metas) = self._ctx.index.searchable_segment_metas() {
            let mut count = 0usize;
            for m in metas {
                count += m.num_docs() as usize;
            }
            return Ok(count);
        }
        Ok(0)
    }

    /// Search the index for the given query.
    ///
    /// This returns a set of results ordered by their relevance according to
    /// the order or the score.
    async fn search(&self, qry: QueryPayload) -> Result<QueryResults> {
        self.reader.search(qry).await
    }

    /// Get a single document via it's given id.
    async fn get_document(&self, doc_id: DocumentId) -> Result<DocumentHit> {
        self.reader.get_document(doc_id).await
    }

    /// Adds one or more documents to the index.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    async fn add_document(&self, id: DocumentId, payload: DocumentPayload) -> Result<()> {
        self.writer.send_op(WriterOp::AddDocument(id, payload)).await
    }

    /// Adds one or more documents to the index.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    async fn add_documents(&self, documents: Vec<(DocumentId, DocumentPayload)>) -> Result<()> {
        self.writer
            .send_op(WriterOp::AddManyDocuments(documents))
            .await
    }

    /// Deletes all documents from the index.
    async fn clear_documents(&self) -> Result<()> {
        self.writer.send_op(WriterOp::DeleteAll).await
    }

    /// Deletes a specific document
    pub async fn delete_document(&self, document_id: DocumentId) -> Result<()> {
        self.writer
            .send_op(WriterOp::DeleteManyDocuments(vec![document_id]))
            .await
    }

    /// Deletes all documents from the index matching a given term(s).
    async fn delete_documents_where(
        &self,
        fields: BTreeMap<String, DocumentValueOptions>,
    ) -> Result<usize> {
        let mut query_payload = vec![];
        for (field, opts) in fields {
            match opts {
                DocumentValueOptions::Single(value) => {
                    query_payload.push(QueryData::make_term_query(
                        field,
                        value,
                        Occur::Should,
                    ));
                },
                DocumentValueOptions::Multi(values) => {
                    for value in values {
                        query_payload.push(QueryData::make_term_query(
                            field.clone(),
                            value,
                            Occur::Should,
                        ));
                    }
                },
            }
        }

        let limit = 10_000;
        let mut total_deleted = 0;
        let mut offset = 0;
        loop {
            let query = QueryPayload {
                query: QuerySelector::Multi(query_payload.clone()),
                limit,
                offset,
                order_by: None,
                order: Order::Asc,
                sort: Default::default(),
            };

            let results = self.search(query).await?;
            let docs: Vec<DocumentId> =
                results.hits.into_iter().map(|v| v.document_id).collect();

            let should_break = docs.len() < limit;
            total_deleted += docs.len();

            self.writer
                .send_op(WriterOp::DeleteManyDocuments(docs))
                .await?;

            if should_break {
                break;
            }

            offset += limit;
        }

        Ok(total_deleted)
    }

    /// Deletes all returned documents matching the given query.
    async fn delete_by_query(&self, qry: QueryPayload) -> Result<usize> {
        let results = self.search(qry).await?;
        let docs: Vec<DocumentId> =
            results.hits.into_iter().map(|v| v.document_id).collect();

        let total_deleted = docs.len();

        self.writer
            .send_op(WriterOp::DeleteManyDocuments(docs))
            .await
            .unwrap();

        Ok(total_deleted)
    }

    /// Updates document
    pub async fn update_document(&self, document_id: DocumentId, replacement_values: HashMap<String, DocumentValueOptions>) -> Result<()> {
        let doc = self
            .reader
            .get_document(document_id)
            .await?;

        let doc_values = doc.doc.iter();

        let mut new_values: BTreeMap<String, DocumentValueOptions> = BTreeMap::new();
        for (k,v) in doc_values {
            if replacement_values.contains_key(k) {
                let replacement = replacement_values.get(k).unwrap().clone();
                new_values.insert(k.to_string(), replacement);
            }else{
                if v.is_some() {
                    new_values.insert(k.to_string(), v.clone().unwrap());
                }
            }
        }

        let payload = DocumentPayload(new_values);
        self.writer.send_op(WriterOp::DeleteManyDocuments(vec![document_id])).await?;
        self.writer.send_op(WriterOp::AddDocument(document_id, payload)).await
    }

    /// Add value to multivalue field
    pub async fn update_document_multivalue(&self,
                                            document_id: DocumentId,
                                            field: String,
                                            value: DocumentValue,
                                            add: bool) -> Result<()> {
        let doc_values = self.reader
            .get_document(document_id)
            .await?
            .doc;

        let mut new_doc_values: BTreeMap<String, DocumentValueOptions> = BTreeMap::new();
        for (k,v) in doc_values {
            if k == field {
                match v {
                    Some(DocumentValueOptions::Multi(fv)) => {
                        let new_field_values = match add {
                            true => {
                                let mut nfv = fv.clone();
                                nfv.push(value.clone());
                                nfv
                            }
                            false => {
                                let mut nfv = fv.clone();
                                let index = fv.iter().position(|x| *x == value).unwrap();
                                nfv.remove(index);
                                nfv
                            }
                        };

                        new_doc_values.insert(field.to_string(), DocumentValueOptions::Multi(new_field_values));
                    }
                    _ => {}
                }
            }else{
                if v.is_some() {
                    new_doc_values.insert(k, v.unwrap());
                }
            }
        }
        let payload = DocumentPayload(new_doc_values);
        self.writer.send_op(WriterOp::DeleteManyDocuments(vec![document_id])).await?;
        self.writer.send_op(WriterOp::AddDocument(document_id, payload)).await
    }

    /// Adds a set of stop words to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    async fn add_stop_words(&self, words: Vec<String>) -> Result<()> {
        self.writer.send_op(WriterOp::AddStopWords(words)).await
    }

    /// Get the current index stop words.
    pub fn get_stop_words(&self) -> Vec<String> {
        self.reader.get_stop_words()
    }

    /// Adds a set of stop words to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    async fn remove_stop_words(&self, words: Vec<String>) -> Result<()> {
        self.writer.send_op(WriterOp::RemoveStopWords(words)).await
    }

    /// Removes all stop words from the index.
    async fn clear_stop_words(&self) -> Result<()> {
        self.writer.send_op(WriterOp::ClearStopWords).await
    }

    /// Adds a set of synonyms to the indexes' synonym manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    async fn add_synonyms(&self, relations: Vec<String>) -> Result<()> {
        self.writer.send_op(WriterOp::AddSynonyms(relations)).await
    }

    /// Get the current index stop words.
    pub fn get_synonyms(&self) -> HashMap<String, Box<[String]>> {
        self.reader.get_synonyms()
    }

    /// Adds a set of synonyms to the indexes' stop word manager.
    ///
    /// This function is semi-asynchronous in the sense that there is a buffer of
    /// 20 tasks that can be submitted to the writer before the extra pending tasks
    /// must wait in order to then submit their operation to the queue.
    async fn remove_synonyms(&self, words: Vec<String>) -> Result<()> {
        self.writer.send_op(WriterOp::RemoveSynonyms(words)).await
    }

    /// Removes all synonyms from the index.
    async fn clear_synonyms(&self) -> Result<()> {
        self.writer.send_op(WriterOp::ClearSynonyms).await
    }

    /// Shuts the index down waiting for all writer threads to finish.
    async fn shutdown(&self) -> Result<()> {
        self.writer.shutdown().await
    }

    /// Shuts the index down removing any persistent data along with it.
    async fn destroy(&self) -> Result<()> {
        let dir = self._ctx.path.join(self._ctx.name.as_str());

        if dir.exists() {
            tokio::fs::remove_dir_all(dir).await?;
        }

        self.writer.destroy().await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;
    use harana_common::{serde_json, tokio};
    use harana_common::uuid::Uuid;
    use super::*;
    use crate::structures::{DocumentOptions, DocumentValue, IndexDeclaration};

    fn init_state() {
        let _ = std::env::set_var("RUST_LOG", "debug,sled=info");
        let _ = harana_common::pretty_env_logger::try_init_timed();
    }

    fn create_temp_path() -> PathBuf {
        let uuid = Uuid::new_v4();
        PathBuf::from("/tmp").join(uuid.to_string())
    }

    async fn get_index_with(value: serde_json::Value) -> Result<Index> {
        let index_path = PathBuf::from(create_temp_path());
        let dec: IndexDeclaration = serde_json::from_value(value)?;
        let res = dec.create_context(&index_path, "passphrase".to_string())?;
        let index = Index::create(res, false).await?;
        Ok(index)
    }

    #[tokio::test]
    async fn memory_lifecycle_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with(serde_json::json!({
            "name": "test_index_memory_lifecycle_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            })?;

        index.destroy().await?;

        Ok(())
    }

    #[tokio::test]
    async fn tempdir_lifecycle_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with(serde_json::json!({
            "name": "test_index_tempdir_lifecycle_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            })?;

        index.destroy().await?;

        Ok(())
    }

    #[tokio::test]
    async fn filesystem_lifecycle_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with(serde_json::json!({
            "name": "test_index_filesystem_lifecycle_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

            "storage_type": "filesystem",
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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            })?;

        index.destroy().await?;

        Ok(())
    }

    #[tokio::test]
    async fn multi_threaded_reader_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with(serde_json::json!({
            "name": "test_index_multi_threaded_reader_expect_ok",

            // Reader context
            "reader_threads": 12,
            "max_concurrency": 1,

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            })?;

        index.destroy().await?;

        Ok(())
    }

    #[tokio::test]
    async fn single_threaded_reader_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with(serde_json::json!({
            "name": "test_index_single_threaded_reader_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            })?;

        index.destroy().await?;

        Ok(())
    }

    #[tokio::test]
    async fn zero_concurrency_expect_err() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_0_concurrency_expect_err",

            // Reader context
            "reader_threads": 1,
            "max_concurrency": 0,

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
                   "fast": "single"
                },
            },
            "search_fields": [
                "title",
                "description",
            ],
        }))
            .await;

        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn concurrency_expect_ok() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_0_concurrency_expect_err",

            // Reader context
            "reader_threads": 1,
            "max_concurrency": 12,

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            });

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn zero_buffer_expect_defaulting() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_0_buffer_expect_err",

            "writer_buffer": 0,
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
        }))
            .await;

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn default_writer_settings_expect_ok() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_0_buffer_expect_err",

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
        })).await;

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn buffer_expect_ok() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_buffer_expect_ok",

            "writer_buffer": 36_000_000,
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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            });

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn zero_writer_threads_expect_err() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_0_writer_threads_expect_err",

            "writer_buffer": 3500000,
            "writer_threads": 0,

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
                   "fast": "single"
                },
            },
            "search_fields": [
                "title",
                "description",
            ],
        }))
            .await;

        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn writer_threads_expect_ok() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_buffer_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            });

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn no_search_fields_expect_err() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_no_search_fields_expect_err",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
                   "fast": "single"
                },
            },
            "search_fields": [
            ],
        }))
            .await;

        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn search_fields_expect_ok() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_search_fields_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            });

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn no_fields_expect_err() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_no_fields_expect_err",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

            "storage_type": "tempdir",
            "fields": {
            },
            "search_fields": [
            ],
        }))
            .await;

        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn fields_expect_ok() -> Result<()> {
        init_state();

        let res = get_index_with(serde_json::json!({
            "name": "test_index_fields_expect_ok",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

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
                "description"
            ],
        }))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                e
            });

        assert!(res.is_ok());

        Ok(())
    }

    async fn get_basic_index(fast_fuzzy: bool) -> Result<Index> {
        get_index_with(serde_json::json!({
            "name": "basic_test_index",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

            "use_fast_fuzzy": fast_fuzzy,

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
                "category": {
                   "type": "facet",
                   "stored": true,
                   "indexed": true
                },
            },
            "search_fields": [
                "title",
                "description",
            ],
        }))
            .await
    }

    async fn get_index_with_required_title(fast_fuzzy: bool) -> Result<Index> {
        get_index_with(serde_json::json!({
            "name": "basic_test_index",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

            "use_fast_fuzzy": fast_fuzzy,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true,
                    "required": true
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
                "category": {
                   "type": "facet",
                   "stored": true,
                   "indexed": true
                },
            },
            "search_fields": [
                "title",
                "description",
            ],
        }))
            .await
    }

    async fn get_index_with_required_multi_fields(
        fast_fuzzy: bool,
        multi_title: bool,
        multi_description: bool,
    ) -> Result<Index> {
        get_index_with(serde_json::json!({
            "name": "basic_test_index",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

            "use_fast_fuzzy": fast_fuzzy,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true,
                    "required": true,
                    "multi": multi_title
                },
                "description": {
                    "type": "string",
                    "stored": false,
                    "multi": multi_description,
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": true,
                   "fast": true
                },
                "category": {
                   "type": "facet",
                   "stored": true,
                   "indexed": true
                },
            },
            "search_fields": [
                "title",
                "description",
            ],
        }))
            .await
    }

    #[tokio::test]
    async fn add_stop_words_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let words = vec![
            "The".into(),
            "quick".into(),
            "brown".into(),
            "fox".into(),
            "jumped".into(),
        ];

        let res = index.add_stop_words(words).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_extreme_stop_words_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let words = vec![
            "".into(),
            "quick".into(),
            "🚀".into(),
            "fox".into(),
            "jumped".into(),
        ];

        let res = index.add_stop_words(words).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_docs_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!({
            "title": "The Old Man and the Sea",
            "description": "He was an old man who fished alone in a skiff in \
            the Gulf Stream and he had gone eighty-four days \
            now without taking a fish.",
        }))?;

        let result = index.add_documents(document).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn add_docs_with_valid_non_required_field_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!({
            "title": "hello",
        }))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_docs_with_required_field_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with_required_title(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!({
            "title": "hello",
        }))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_docs_valid_doc_missing_required_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with_required_title(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!({
            "description": "hello",
        }))?;

        let res = index.add_documents(document).await;
        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn add_bulk_docs_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_bulk_docs_multi_value_expect_ok() -> Result<()> {
        init_state();

        let index = get_index_with_required_multi_fields(false, true, false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": ["The Old Man and the Sea", "Hello world"],
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_bulk_docs_empty_multi_field_expect_err() -> Result<()> {
        init_state();

        let index = get_index_with_required_multi_fields(false, true, false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": [],
                    "description": ["He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.", "Hello"],
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn add_bulk_docs_with_non_required_missing_fields_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title-title": "The Old Man and the Sea 2",
                    "descriptio": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "titled": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn add_bulk_docs_with_required_field_expect_err() -> Result<()> {
        init_state();

        let index = get_index_with(serde_json::json!({
            "name": "basic_test_index",

            "writer_buffer": 3_000_000,
            "writer_threads": 1,

            "use_fast_fuzzy": false,

            "storage_type": "tempdir",
            "fields": {
                "title": {
                    "type": "text",
                    "stored": true,
                    "required": true
                },
                "description": {
                    "type": "string",
                    "stored": false,
                    "required": true
                },
                "count": {
                   "type": "u64",
                   "stored": true,
                   "indexed": true,
                   "fast": true
                }
            },
            "search_fields": [
                "title",
                "description",
            ],
        }))
            .await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title-title": "The Old Man and the Sea 2",
                    "descriptio": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "titled": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn remove_docs_single_field_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        let mut mapping = BTreeMap::new();
        mapping.insert(
            "count".to_string(),
            DocumentValueOptions::Single(DocumentValue::U64(3)),
        );

        let res = index.delete_documents_where(mapping).await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn remove_docs_many_fields_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        let mut mapping = BTreeMap::new();
        mapping.insert(
            "count".to_string(),
            DocumentValueOptions::Single(DocumentValue::U64(3)),
        );

        // This should get converted to "3".
        mapping.insert(
            "title".to_string(),
            DocumentValueOptions::Single(DocumentValue::I64(3)),
        );

        let res = index.delete_documents_where(mapping).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn remove_docs_single_field_many_values_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        let values = vec![
            DocumentValue::U64(3),
            DocumentValue::U64(4),
            DocumentValue::U64(6),
        ];
        let mut mapping = BTreeMap::new();
        mapping.insert("count".to_string(), DocumentValueOptions::Multi(values));

        let res = index.delete_documents_where(mapping).await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn remove_docs_many_fields_many_values_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        let values = vec![
            DocumentValue::U64(3),
            DocumentValue::U64(4),
            DocumentValue::U64(6),
        ];
        let mut mapping = BTreeMap::new();
        mapping.insert("count".to_string(), DocumentValueOptions::Multi(values));

        let values = vec![
            DocumentValue::Text("the".into()),
            DocumentValue::Text("quick".into()),
            DocumentValue::Text("brown".into()),
            DocumentValue::Text("fox".into()),
        ];
        mapping.insert("title".to_string(), DocumentValueOptions::Multi(values));

        let res = index.delete_documents_where(mapping).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn remove_docs_with_normal_query_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "normal": {"ctx": "*"},
            },
        }))?;

        let results = index.delete_documents_by_query(query).await.map_err(|e| {
            eprintln!("{:?}", e);
            e
        });
        assert!(results.is_ok());
        assert_eq!(results.unwrap(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn remove_docs_wrong_type_expect_err() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;

        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                },
                {
                    "title": "The Old Man and the Sea 2",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                },
            ]
        ))?;

        let res = index.add_documents(document).await;
        assert!(res.is_ok());

        let mut mapping = BTreeMap::new();

        let values = vec![
            DocumentValue::Text("1".into()),
            DocumentValue::Text("3".into()),
            DocumentValue::Text("brown".into()),
            DocumentValue::Text("45".into()),
        ];
        mapping.insert("count".to_string(), DocumentValueOptions::Multi(values));

        let res = index.delete_documents_where(mapping).await;
        assert!(res.is_err());

        Ok(())
    }

    const NUM_DOCS: usize = 3;

    async fn add_documents(index: &Index) -> Result<()> {
        let document: DocumentOptions = serde_json::from_value(serde_json::json!(
            [
                {
                    "title": "The Old Man and the Sea extra word",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "category": "/tools/hammers",
                },
                {
                    "title": "The Old Man and the Sea 2 extra word",
                    "description": "He was an old man who fished alone in a skiff in \
                    the Gulf Stream and he had gone eighty-four days \
                    now without taking a fish.",
                    "category": "/tools/hammers",
                    "count": 3
                },
                {
                    "title": "The Old Man and the Sea 3",
                    "category": "/tools/fish",
                },
            ]
        ))?;

        index.add_documents(document).await?;
        index.commit().await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;

        Ok(())
    }

    #[tokio::test]
    async fn search_fuzzy_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "fuzzy": {"ctx": "ol man"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": "Man",
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_fast_fuzzy_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(true).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "fuzzy": {"ctx": "ol man"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_normal_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "normal": {"ctx": "*"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_more_like_this_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "normal": {"ctx": "man"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        let doc_id = results.hits[0].document_id;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "more-like-this": {"ctx": doc_id},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_term_with_single_field_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "term": {"ctx": "man", "fields": "title"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_term_with_multi_fields_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "term": {"ctx": "man", "fields": ["title", "description"]},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_term_with_multi_fields_and_boost_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "term": {"ctx": "man", "fields": {"title": 2.0, "description": 1.0}},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_term_with_default_fields_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "term": {"ctx": "man"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        Ok(())
    }

    #[tokio::test]
    async fn search_facet_term_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "term": {"ctx": "/tools", "fields": "category"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), NUM_DOCS);

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": {
                "term": {"ctx": "/tools/hammers", "fields": "category"},
            },
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn search_combination_query_expect_ok() -> Result<()> {
        init_state();

        let index = get_basic_index(false).await?;
        add_documents(&index).await?;

        let query: QueryPayload = serde_json::from_value(serde_json::json!({
            "query": [
                {
                    "normal": {"ctx": "extra"},
                    "occur": "must",
                },
                {
                    "term": {"ctx": "3", "fields": "count"},
                    "occur": "must",
                },
            ],
        }))?;

        let results = index.search(query).await?;
        assert_eq!(results.hits.len(), 1);

        Ok(())
    }
}