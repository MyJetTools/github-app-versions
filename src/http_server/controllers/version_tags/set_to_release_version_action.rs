use std::sync::Arc;

use my_http_server::{
    macros::*,
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/Releases/ToReleaseVersion",
    controller: "VersionTags",
    description: "Set version to release",
    summary: "Set version to release",
    input_data: SetVersionInputData,
 
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct SetToReleaseVersionHttpAction {
    app: Arc<AppContext>,
}

impl SetToReleaseVersionHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SetToReleaseVersionHttpAction,
    input_data: SetVersionInputData,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {

    let env = action.app.resolve_env_id(ctx).await?;

    action.app.tags_version_maps_repo.insert_or_update(&env, input_data.tag, input_data.version, input_data.git_hub_name).await;

    HttpOutput::Empty.into_ok_result(true).into()
}



#[derive(Debug, MyHttpInput)]
pub struct SetVersionInputData {
    #[http_form_data(description = "Tag name")]
    pub tag: String,
    #[http_form_data(description = "Version")]
    pub version: String,

    #[http_form_data(description = "GitHub name")]
    pub git_hub_name: Option<String>,
}

