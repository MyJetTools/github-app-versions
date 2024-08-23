use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/Releases/SetToReleaseVersion",
    controller: "Releases",
    description: "Get release yaml",
    summary: "Get release yaml",
    input_data: SetVersionToReleaseInputModel,
 
    result:[
        {status_code: 204, description: "Ok Result"},
        {status_code: 403, description: "Validation error"},
    ]
)]
pub struct SetVersionsToReleaseAction {
    app: Arc<AppContext>,
}

impl SetVersionsToReleaseAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SetVersionsToReleaseAction,
    input_data: SetVersionToReleaseInputModel,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let env_id = action.app.resolve_env_id(ctx).await?;
    crate::flows::set_version_to_release(&action.app, env_id, &input_data.id, input_data.version).await.unwrap();
   HttpOutput::Empty.into_ok_result(false)
}




#[derive(Debug, MyHttpInput)]
pub struct SetVersionToReleaseInputModel{
    #[http_form_data(description = "Id of service")]
    pub id: String,

    #[http_form_data(description = "Versions to release")]
    pub version: String,
}