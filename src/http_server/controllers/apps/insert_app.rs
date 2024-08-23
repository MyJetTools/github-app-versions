use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};


use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/App/Insert",
    controller: "Repos",
    description: "Get release yaml",
    summary: "Get release yaml",
    input_data: InsertAppsInputModel,
 
    result:[
        {status_code: 204, description: "Ok Result"},
        {status_code: 403, description: "Validation error"},
    ]
)]
pub struct InsertAppHttpAction {
    app: Arc<AppContext>,
}

impl InsertAppHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &InsertAppHttpAction,
    input_data: InsertAppsInputModel,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let env_id = action.app.resolve_env_id(ctx).await?;



    action.app.app_information_repo.insert_or_update(&env_id, &input_data.app_id,  &input_data.group, &input_data.release_version_tag).await;

    
   HttpOutput::Empty.into_ok_result(false)
}


#[derive(Debug, MyHttpInput)]
pub struct InsertAppsInputModel {
    #[http_body(description = "App Id")]
    pub app_id: String,

    #[http_body(description = "App Group")]
    pub group: String,

    #[http_body(description = "Version Tag")]
    pub release_version_tag: String,
}

