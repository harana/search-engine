use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::sync::Arc;
use triple_accel::levenshtein_exp;

use harana_common::arc_swap::ArcSwap;
use harana_common::itertools::Itertools;
use harana_common::serde::{Deserialize, Serialize};
use harana_common::serde;

use crate::mac::{applications, generate_icons};

pub struct ApplicationManager {
    icons_path: &'static PathBuf,
    applications: ArcSwap<Vec<Application>>
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Application {
    pub name: String,
    pub path: String,
    pub source_icon_path: Option<String>,
    pub target_icon_path: Option<String>
}

impl ApplicationManager {

    pub async fn new(icons_path: &'static PathBuf) -> Self {
        let apps = applications(icons_path);
        generate_icons(apps.clone());

        Self {
            icons_path,
            applications: ArcSwap::from_pointee(apps.clone())
        }
    }

    pub async fn refresh(&self) {
        let apps = applications(self.icons_path);
        generate_icons(apps.clone());
        self.applications.store(Arc::from(apps.clone()));
    }

    pub async fn search(&self, prefix: &str) -> Option<Application> {
        let applications = self.applications.load()
            .iter()
            .map(|a| Application {
                name: a.clone().name,
                path: a.clone().path,
                source_icon_path: a.clone().source_icon_path,
                target_icon_path: a.clone().target_icon_path
            })
            .collect_vec();

        for a in &applications {
            if a.name.to_lowercase() == prefix.to_lowercase() {
                return Some(a.clone())
            }
        }

        for a in &applications {
            if a.name.to_lowercase().split(" ").any(|w| w.starts_with(&prefix.to_lowercase()) || w == prefix.to_lowercase()) {
                return Some(a.clone())
            }
        }

        for a in &applications {
            if levenshtein_exp(a.name.to_lowercase().as_bytes(), prefix.to_lowercase().as_bytes()) <= 2 {
                return Some(a.clone())
            }
        }

        return None
    }
}