use std::sync::Arc;

use my_http_server::{
    macros::*,
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[http_route(
    method: "DELETE",
    route: "/api/Releases/Delete",
    controller: "VersionTags",
    description: "Delete version to release",
    summary: "Delete version to release",
    input_data: DeleteVersionInputData,
 
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct DeleteToReleaseVersionHttpAction {
    app: Arc<AppContext>,
}

impl DeleteToReleaseVersionHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteToReleaseVersionHttpAction,
    input_data: DeleteVersionInputData,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {

    let env = action.app.resolve_env_id(ctx).await?;

    action.app.tags_version_maps_repo.delete_if_exists(&env, &input_data.tag).await;

    HttpOutput::Empty.into_ok_result(true).into()
}



#[derive(Debug, MyHttpInput)]
pub struct DeleteVersionInputData {
    #[http_query(description = "Tag name")]
    pub tag: String,
}

