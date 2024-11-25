use std::sync::Arc;

use my_http_server::{macros::*, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/GitHub",
    controller: "GitHub",
    description: "Get list of github versions",
    summary: "Get list of github versions",
    result:[
        {status_code: 200, description: "List github versions"},
    ]
)]
pub struct GeVersionsAction {
    app: Arc<AppContext>,
}

impl GeVersionsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GeVersionsAction,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let all_versions = action.app.git_hub_versions_repo.get_all().await;
    HttpOutput::as_json(all_versions)
        .into_ok_result(true)
        .into()
}
