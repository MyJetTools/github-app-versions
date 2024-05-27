use std::collections::BTreeMap;

use rust_extensions::ShortString;
use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub git_hub_api_key: String,
    pub repos: BTreeMap<String, Vec<GitHubRepoSettingsModel>>,
    pub released_versions_yaml_url: String,
}

impl SettingsReader {
    pub async fn get_released_versions_yaml_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.released_versions_yaml_url.clone()
    }

    pub async fn get_repos(&self) -> BTreeMap<String, Vec<GitHubRepoSettingsModel>> {
        let read_access = self.settings.read().await;
        read_access.repos.clone()
    }

    pub async fn get_git_hub_api_key(&self) -> ShortString {
        let read_access = self.settings.read().await;
        ShortString::from_str(read_access.git_hub_api_key.as_str()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRepoSettingsModel {
    pub id: String,
    pub release_version_tag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCertInfo {
    pub file_name: String,
    pub password: String,
}
