use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/Releases/Github",
    controller: "Releases",
    description: "Get list of github releases",
    summary: "Get list of github releases",
    result:[
        {status_code: 200, description: "Git list of releases"},
    ]
)]
pub struct GetGitHubReleasesAction {
    app: Arc<AppContext>,
}

impl GetGitHubReleasesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetGitHubReleasesAction,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = action.app.cache.lock().await.get_github_repos();
    HttpOutput::as_json(result).into_ok_result(true).into()
}
