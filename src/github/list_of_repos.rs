use std::sync::Arc;

use flurl::{FlUrl, HttpClientsCache};
use serde::*;

pub async fn list_of_repos(
    api_key: &str,
    http_clients_cache: Arc<HttpClientsCache>,
) -> Vec<GitRepoItem> {
    let mut result = FlUrl::new("https://api.github.com/orgs/my-cfd-platform/repos")
        .with_clients_cache(http_clients_cache)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", "RustClient")
        .with_header("Authorization", format!("Bearer {}", api_key))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .get()
        .await
        .unwrap();

    if result.get_status_code() != 200 {
        println!("Status: {}", result.get_status_code());
        let result = result.get_body_as_slice().await.unwrap();

        panic!("{}", String::from_utf8_lossy(result));
    }

    let result = result.get_body_as_slice().await.unwrap();

    serde_json::from_slice(result).unwrap()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitRepoItem {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub url: String,
}
