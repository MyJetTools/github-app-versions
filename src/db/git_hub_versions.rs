use std::collections::BTreeMap;

use serde::*;
use tokio::sync::Mutex;

use super::db_inner::DbInner;

const TABLE_NAME: &str = "github_versions";
const SHARED_ENV: &str = "shared";
pub struct GitHubVersionsRepo {
    inner: Mutex<DbInner<GitHubVersionsModel>>,
}

impl GitHubVersionsRepo {
    pub fn new(path: String) -> Self {
        Self {
            inner: Mutex::new(DbInner::new(path)),
        }
    }

    pub async fn get_all(&self) -> BTreeMap<String, String> {
        let mut inner = self.inner.lock().await;
        let model = inner.load(SHARED_ENV, TABLE_NAME).await;
        model.versions
    }

    pub async fn save(&self, repo: String, version: String) {
        let mut inner = self.inner.lock().await;
        let mut model = inner.load(SHARED_ENV, TABLE_NAME).await;
        model.versions.insert(repo, version);
        inner.save(SHARED_ENV, TABLE_NAME, model).await;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GitHubVersionsModel {
    pub versions: BTreeMap<String, String>,
}
