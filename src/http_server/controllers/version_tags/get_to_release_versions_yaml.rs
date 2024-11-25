use std::{collections::BTreeMap, sync::Arc};

use my_http_server::{
    macros::*,
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::Serialize;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/Releases/ToReleaseVersions",
    controller: "VersionTags",
    description: "Get release yaml",
    summary: "Get release yaml",
 
    result:[
        {status_code: 200, description: "List of release versions in yaml"},
    ]
)]
pub struct GetToReleaseVersionsYamlAction {
    app: Arc<AppContext>,
}

impl GetToReleaseVersionsYamlAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetToReleaseVersionsYamlAction,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {

    let env_id = action.app.resolve_env_id(ctx).await?;

    let var = action.app.tags_version_maps_repo.get_all(env_id.as_str()).await;
    let result = GetToReleaseVersionsHttpResponse{
        vars: var.into_iter().map(|(k, v)| (k, v.ver)).collect()
    };

    HttpOutput::as_yaml(result).into_ok_result(true).into()
}


#[derive(Debug, Serialize, MyHttpObjectStructure)]
pub struct GetToReleaseVersionsHttpResponse{
    pub vars: BTreeMap<String, String>
}