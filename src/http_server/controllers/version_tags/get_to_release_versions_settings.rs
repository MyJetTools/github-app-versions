use std::{collections::BTreeMap, sync::Arc};

use my_http_server::{
    macros::*,
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::Serialize;

use crate::{app::AppContext};

#[http_route(
    method: "GET",
    route: "/api/Releases/ToReleaseVersions",
    controller: "VersionTags",
    description: "Get release yaml",
    summary: "Get release yaml",
 
    result:[
        {status_code: 200, description: "List of release settings", model:"GetToReleaseVersionsSettingsHttpResponse"},
    ]
)]
pub struct GetToReleaseVersionsSettingsAction {
    app: Arc<AppContext>,
}

impl GetToReleaseVersionsSettingsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetToReleaseVersionsSettingsAction,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {

    let env_id = action.app.resolve_env_id(ctx).await?;

    let resp = action.app.tags_version_maps_repo.get_all(env_id.as_str()).await;
    let result = GetToReleaseVersionsSettingsHttpResponse{
        vars: resp.into_iter().map(|(k, v)| (k, TagVersionHttpModel{
            ver: v.ver,
            git_hub_repo_id: v.git_hub_repo_id,
        })).collect()
    };

    HttpOutput::as_yaml(result).into_ok_result(true).into()
}


#[derive(Debug, Serialize, MyHttpObjectStructure)]
pub struct GetToReleaseVersionsSettingsHttpResponse{
    pub vars: BTreeMap<String, TagVersionHttpModel>
}

#[derive(Debug, Serialize, MyHttpObjectStructure)]
pub struct TagVersionHttpModel{
    pub ver: String,
    pub git_hub_repo_id: Option<String>
}