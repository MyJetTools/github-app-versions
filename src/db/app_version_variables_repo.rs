use std::collections::BTreeMap;

use serde::*;
use tokio::sync::Mutex;

use super::db_inner::DbInner;
pub const TABLE_NAME: &str = "apps";

pub struct AppInformationRepo {
    inner: Mutex<DbInner<AppConfigDto>>,
}

impl AppInformationRepo {
    pub fn new(path: String) -> Self {
        Self {
            inner: Mutex::new(DbInner::new(path)),
        }
    }

    pub async fn get_all(&self, env_id: &str) -> BTreeMap<String, Vec<RepoInfoDto>> {
        let mut inner = self.inner.lock().await;
        let result = inner.load(env_id, TABLE_NAME).await;
        result.repos
    }

    pub async fn get(&self, env_id: &str, app_id: &str) -> Option<RepoInfoDto> {
        let mut inner = self.inner.lock().await;
        let result = inner.load(env_id, TABLE_NAME).await;
        for (_, items) in result.repos {
            for itm in items {
                if itm.id == app_id {
                    return Some(itm);
                }
            }
        }

        None
    }

    pub async fn rename_app(&self, env_id: &str, old_app_id: &str, new_app_id: &str) -> bool {
        let mut inner = self.inner.lock().await;
        let mut model = inner.load(env_id, TABLE_NAME).await;

        let mut has_to_save = true;
        for (_, repos) in model.repos.iter_mut() {
            if has_to_save {
                break;
            }
            for repo in repos.iter_mut() {
                if repo.id == old_app_id {
                    repo.id = new_app_id.to_string();
                    has_to_save = true;
                    break;
                }
            }
        }

        if has_to_save {
            inner.save(env_id, TABLE_NAME, model).await;
        }

        has_to_save
    }

    pub async fn insert_or_update(
        &self,
        env_id: &str,
        app_id: &str,
        group_id: &str,
        release_version_tag: &str,
    ) {
        let mut inner = self.inner.lock().await;
        let mut model = inner.load(env_id, TABLE_NAME).await;

        if !model.repos.contains_key(group_id) {
            model.repos.insert(group_id.to_string(), vec![]);
        }

        let items = model.repos.get_mut(group_id).unwrap();

        items.retain(|itm| itm.id != app_id);

        items.push(RepoInfoDto {
            id: app_id.to_string(),
            release_version_tag: release_version_tag.to_string(),
        });

        inner.save(env_id, TABLE_NAME, model).await;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AppConfigDto {
    pub repos: BTreeMap<String, Vec<RepoInfoDto>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RepoInfoDto {
    pub id: String,
    pub release_version_tag: String,
}
