use harana_common::tantivy::Order;

use harana_common::futures::future::join_all;
use harana_common::hashbrown::HashMap;
use harana_document::document::Document;
use harana_tantivy::QueryPayload;
use harana_tantivy::query::{FieldSelector, Occur, QueryData, QueryKind, QuerySelector};
use harana_tantivy::structures::DocumentValue;
use crate::manager::IndexManager;

pub struct IndexSearcher {
    index_manager: &'static IndexManager
}

impl IndexSearcher {

    pub async fn new(index_manager: &'static IndexManager) -> Self {
        Self {
            index_manager
        }
    }

    pub async fn search(&self, indexes: Vec<String>, query: String) -> HashMap<String, Vec<Document>> {
        let results: Vec<_> = indexes.iter().map(|index_name| async {
            let index = self.index_manager.get_index(index_name);

            let results = index.search(
                QueryPayload {
                    query: QuerySelector::Single(
                        QueryData {
                            kind: QueryKind::Fuzzy {
                                ctx: DocumentValue::Text(query.to_string()),
                                cfg: Default::default(),
                                fields: FieldSelector::MultiWithBoost(HashMap::from([
                                    ("title".to_string(), 100.0),
                                    ("primary_tokens".to_string(), 100.0),
                                    ("secondary_tokens".to_string(), 10.0),
                                ])),
                            },
                            occur: Occur::Must,
                        }
                    ),
                    limit: 100,
                    offset: 0,
                    order: Order::Asc,
                    order_by: None,
                    sort: Default::default(),
                }
            ).await.unwrap();

            let docs: Vec<Document> = results.hits.into_iter().map(|hit| Document::from_hit(hit)).collect();
            (index_name.to_string(), docs)
        }).collect();

        join_all(results).await.into_iter().collect()
    }
}