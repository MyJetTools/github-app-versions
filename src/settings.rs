use rust_extensions::ShortString;
use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub git_hub_api_key: String,
    //pub repos: BTreeMap<String, Vec<GitHubRepoSettingsModel>>,
    //pub versions_yaml_file_path: String,
    pub db_path: String,
    pub envs: Vec<EnvInfoSettingsModel>,
}

impl SettingsReader {
    /*
    pub async fn get_versions_yaml_file_path(&self) -> String {
        let read_access = self.settings.read().await;

        rust_extensions::file_utils::format_path(read_access.versions_yaml_file_path.as_str())
            .to_string()
    }
     */

    pub async fn get_envs(&self) -> Vec<String> {
        let read_access = self.settings.read().await;
        let mut result = Vec::with_capacity(read_access.envs.len());

        for env in read_access.envs.iter() {
            result.push(env.id.clone());
        }

        result
    }
    pub async fn get_app_versions_db_path(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.db_path.clone()
    }

    pub async fn find_env_id(&self, domain: &str) -> Option<String> {
        let read_access = self.settings.read().await;

        for env in read_access.envs.iter() {
            if rust_extensions::str_utils::compare_strings_case_insensitive(&env.domain, domain) {
                return Some(env.id.to_lowercase());
            }
        }

        None
    }

    /*
    pub async fn get_repos(&self) -> BTreeMap<String, Vec<GitHubRepoSettingsModel>> {
        let read_access = self.settings.read().await;
        read_access.repos.clone()
    }


    pub async fn get_service_version_tag(&self, service_id: &str) -> Option<String> {
        let read_access = self.settings.read().await;

        for services in read_access.repos.values() {
            for service in services {
                if service.id == service_id {
                    return Some(service.release_version_tag.clone());
                }
            }
        }

        None
    }
    */

    pub async fn get_git_hub_api_key(&self) -> ShortString {
        let read_access = self.settings.read().await;
        ShortString::from_str(read_access.git_hub_api_key.as_str()).unwrap()
    }
}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct GitHubRepoSettingsModel {
//    pub id: String,
//   pub release_version_tag: String,
//}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCertInfo {
    pub file_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvInfoSettingsModel {
    pub domain: String,
    pub id: String,
}
