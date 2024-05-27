use flurl::FlUrl;
use serde::*;

pub async fn list_of_repos(api_key: &str) -> Vec<GitRepoItem> {
    let mut result = FlUrl::new("https://api.github.com/orgs/my-cfd-platform/repos")
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", "RustClient")
        .with_header("Authorization", format!("Bearer {}", api_key))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .get()
        .await
        .unwrap();

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
