use std::cmp::Reverse;
use std::hash::{Hash, Hasher};

use harana_common::anyhow::Result;
use harana_common::tantivy::schema::Schema;
use harana_common::tantivy::DateTime;
use crate::structures::{DocumentHit, DocumentValueOptions};

pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    fn validate_with_schema(&self, _schema: &Schema) -> Result<()> {
        Ok(())
    }
}

pub trait Calculated {
    fn calculate_once(&mut self) -> Result<()>;
}

pub trait AsScore {
    fn as_score(&self) -> Option<f32> {
        None
    }
}

impl AsScore for f64 {}
impl AsScore for u64 {}
impl AsScore for i64 {}
impl AsScore for DateTime {}
impl AsScore for Reverse<f64> {}
impl AsScore for Reverse<u64> {}
impl AsScore for Reverse<i64> {}
impl AsScore for Reverse<DateTime> {}

impl AsScore for f32 {
    fn as_score(&self) -> Option<f32> {
        Some(*self)
    }
}

pub fn cr32_hash(v: impl Hash) -> u64 {
    let mut hasher = crc32fast::Hasher::default();

    v.hash(&mut hasher);

    hasher.finish()
}

pub fn hit_bool(hit: &DocumentHit, field: &str) -> Option<bool> {
    hit.doc.get(field).and_then(|odvo| {
        odvo.clone().and_then(|dvo| {
            match dvo {
                DocumentValueOptions::Single(v) => v.as_string().parse::<bool>().ok(),
                DocumentValueOptions::Multi(_) => None
            }
        })
    })
}

pub fn hit_string(hit: &DocumentHit, field: &str) -> Option<String> {
    hit.doc.get(field).and_then(|odvo| {
        odvo.clone().and_then(|dvo| {
            match dvo {
                DocumentValueOptions::Single(v) => Some(v.as_string()),
                DocumentValueOptions::Multi(_) => None
            }
        })
    })
}

pub fn hit_u64(hit: &DocumentHit, field: &str) -> Option<u64> {
    hit.doc.get(field).and_then(|odvo| {
        odvo.clone().and_then(|dvo| {
            match dvo {
                DocumentValueOptions::Single(v) => v.as_string().parse::<u64>().ok(),
                DocumentValueOptions::Multi(_) => None
            }
        })
    })
}
