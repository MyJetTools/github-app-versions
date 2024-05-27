use std::collections::HashMap;

use flurl::FlUrl;
use serde::*;

use crate::app::AppContext;

pub async fn get_released_versions(app: &AppContext) -> HashMap<String, String> {
    let url = app.settings_reader.get_released_versions_yaml_url().await;
    let result = FlUrl::new(url)
        .get()
        .await
        .unwrap()
        .receive_body()
        .await
        .unwrap();

    let result: VersionsYamlModel = serde_yaml::from_slice(&result).unwrap();

    result.vars
}

#[derive(Debug, Deserialize)]
pub struct VersionsYamlModel {
    pub vars: HashMap<String, String>,
}
