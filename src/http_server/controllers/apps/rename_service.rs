use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/App/RenameAppId",
    controller: "Repos",
    description: "Rename App Id",
    summary: "Rename App Id",
    input_data: RenameAppInputModel,
 
    result:[
        {status_code: 204, description: "Ok Result"},
        {status_code: 403, description: "Validation error"},
    ]
)]
pub struct RenameAppIdHttpAction {
    app: Arc<AppContext>,
}

impl RenameAppIdHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &RenameAppIdHttpAction,
    input_data: RenameAppInputModel,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let env_id = action.app.resolve_env_id(ctx).await?;

    
    let result = action.app.app_information_repo.rename_app(env_id.as_str(), &input_data.app_id, &input_data.new_app_id).await;

    if !result{
        return HttpFailResult::as_not_found("App not found".to_string(), false).into_err();
    }
    
   HttpOutput::Empty.into_ok_result(false)
}


#[derive(Debug, MyHttpInput)]
pub struct RenameAppInputModel {
    #[http_body(description = "Old App Id")]
    pub app_id: String,
    #[http_body(description = "New App Id")]
    pub new_app_id: String,
}

