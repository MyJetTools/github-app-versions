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
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {

    match crate::flows::set_version_to_release(&action.app, &input_data.id, input_data.version).await{
        Ok(_) => {
            HttpOutput::Empty.into_ok_result(true).into()
        },
        Err(err) => {
            HttpFailResult::as_validation_error(err).into()
        },
    }
    

    
}




#[derive(Debug, MyHttpInput)]
pub struct SetVersionToReleaseInputModel{
    #[http_body(description = "Id of service")]
    pub id: String,

    #[http_body(description = "Versions to release")]
    pub version: String,
}