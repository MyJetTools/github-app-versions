use std::collections::BTreeMap;

use serde::*;
use tokio::sync::Mutex;

use super::db_inner::DbInner;

pub const TABLE_NAME: &str = "versions";

pub struct TagsVersionMapsRepo {
    inner: Mutex<DbInner<TagsVersionsDbModel>>,
}

impl TagsVersionMapsRepo {
    pub fn new(path: String) -> Self {
        Self {
            inner: Mutex::new(DbInner::new(path)),
        }
    }

    pub async fn insert_or_update(
        &self,
        env: &str,
        tag: String,
        ver: String,
        mut git_hub_name: Option<String>,
    ) {
        let mut inner = self.inner.lock().await;

        let mut model: TagsVersionsDbModel = inner.load(env, TABLE_NAME).await;

        let removed_value = model.vars.remove(&tag);

        if git_hub_name.is_none() {
            if let Some(removed_value) = removed_value {
                git_hub_name = removed_value.git_hub_name;
            }
        }

        model.vars.insert(tag, TagVersion { ver, git_hub_name });

        inner.save(env, TABLE_NAME, model).await;
    }

    pub async fn delete_if_exists(&self, env: &str, tag: &str) {
        let mut inner = self.inner.lock().await;

        let mut model: TagsVersionsDbModel = inner.load(env, TABLE_NAME).await;

        model.vars.remove(tag);

        inner.save(env, TABLE_NAME, model).await;
    }

    /*
       pub async fn bulk_insert_or_update(&self, env: &str, items: BTreeMap<String, TagVersion>) {
           let mut inner = self.inner.lock().await;

           let mut model = inner.load(env, TABLE_NAME).await;

           for (tag, version) in items {
               model.vars.insert(tag, version);
           }

           inner.save(env, TABLE_NAME, model).await;
       }
    */
    pub async fn get_all(&self, env_id: &str) -> BTreeMap<String, TagVersion> {
        let mut inner = self.inner.lock().await;

        let model = inner.load(env_id, TABLE_NAME).await;

        model.vars
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TagsVersionsDbModel {
    pub vars: BTreeMap<String, TagVersion>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TagVersion {
    pub ver: String,
    pub git_hub_name: Option<String>,
}
