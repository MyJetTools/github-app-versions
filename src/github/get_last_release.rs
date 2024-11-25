use std::sync::Arc;

use flurl::{FlUrl, HttpClientsCache};

use serde::*;

pub async fn get_last_release(
    api_key: &str,
    repo_id: &str,
    http_clients_cache: Arc<HttpClientsCache>,
) -> Result<String, String> {
    let mut repo_id_encoded = String::new();

    let repo_id = if repo_id.starts_with("/") {
        &repo_id[1..]
    } else {
        repo_id
    };

    for segment in repo_id.split("/") {
        repo_id_encoded.push('/');
        let encoded_url = url_utils::url_encoder::encode_string(segment);
        repo_id_encoded.push_str(encoded_url.as_str());
    }

    let url = if repo_id_encoded.starts_with('/') {
        format!(
            "https://api.github.com/repos{}/releases/latest",
            repo_id_encoded.as_str()
        )
    } else {
        format!(
            "https://api.github.com/repos/{}/releases/latest",
            repo_id_encoded.as_str()
        )
    };

    //println!("Doing request: {}", url);
    let mut result = FlUrl::new(url)
        .with_clients_cache(http_clients_cache)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", "RustClient")
        .with_header("Authorization", format!("Bearer {}", api_key))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .get()
        .await
        .map_err(|err| format!("{:?}", err))?;

    let result = result
        .get_body_as_slice()
        .await
        .map_err(|err| format!("{:?}", err))?;

    /*
       if repo_id.contains(".") {
           println!("Result: {}", std::str::from_utf8(result).unwrap());
       }
    */
    let result: GetVersionItem =
        serde_json::from_slice(result).map_err(|err| format!("{:?}", err))?;

    Ok(result.tag_name)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionItem {
    pub tag_name: String,
}
