use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/Releases/GithubVersion",
    controller: "Releases",
    description: "Get list of github version",
    summary: "Get list of github version",
    input_data: GetGitHubVersionInputData,
    result:[
        {status_code: 200, description: "Git list of releases"},
    ]
)]
pub struct GetGitHubVersionAction {
    app: Arc<AppContext>,
}

impl GetGitHubVersionAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetGitHubVersionAction,
    input_data: GetGitHubVersionInputData,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let git_hub_api_key = action.app.settings_reader.get_git_hub_api_key().await;
    let http_clients_cache = Arc::new(flurl::HttpClientsCache::new());
    let result = crate::github::get_last_release(
        git_hub_api_key.as_str(),
        &input_data.repo_id,
        http_clients_cache,
    )
    .await;
    HttpOutput::as_json(result).into_ok_result(true).into()
}

#[derive(Debug, MyHttpInput)]
pub struct GetGitHubVersionInputData {
    #[http_query(name:"repoId", description = "Repository id")]
    pub repo_id: String,
}
