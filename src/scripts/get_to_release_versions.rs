use std::collections::BTreeMap;

use serde::*;

use crate::app::AppContext;

pub async fn get_to_release_versions(app: &AppContext) -> BTreeMap<String, String> {
    let mut cache_access = app.cache.lock().await;

    if let Some(to_release_versions) = &cache_access.to_release_versions {
        return to_release_versions.clone();
    }

    let file_path = app.settings_reader.get_versions_yaml_file_path().await;

    let result = tokio::fs::read_to_string(file_path).await.unwrap();

    let result: VersionsYamlModel = serde_yaml::from_str(&result).unwrap();

    cache_access.to_release_versions = Some(result.vars.clone());

    result.vars
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionsYamlModel {
    pub vars: BTreeMap<String, String>,
}
