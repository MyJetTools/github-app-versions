use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use my_http_server::{macros::*, *};
use serde::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/Releases",
    controller: "Releases",
    description: "Get list of releases",
    summary: "Get list of releases",
    result:[
        {status_code: 200, description: "Git list of released apps"},
    ]
)]
pub struct GetReleasesAction {
    app: Arc<AppContext>,
}

impl GetReleasesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetReleasesAction,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let env_id = action.app.resolve_env_id(ctx).await?;

    let mut to_release_version =
        crate::scripts::get_to_release_versions(&action.app, env_id.as_str()).await;

    let apps = action
        .app
        .app_information_repo
        .get_all(env_id.as_str())
        .await;

    let mut github_versions = action.app.git_hub_versions_repo.get_all().await;

    let mut result: BTreeMap<String, Vec<ReleaseInfoHttpModel>> = BTreeMap::new();

    for (group, app_infos) in apps {
        let mut infos = Vec::with_capacity(app_infos.len());
        for app_info in app_infos {
            let git_hub_version = github_versions.remove(app_info.id.as_str());
            let model: ReleaseInfoHttpModel = ReleaseInfoHttpModel {
                id: app_info.id,
                released_version: to_release_version.remove(app_info.release_version_tag.as_str()),
                git_hub_version,
                envs: HashMap::new(),
            };

            infos.push(model);
        }

        result.insert(group, infos);
    }

    HttpOutput::as_json(result).into_ok_result(true).into()
}

#[derive(Debug, Serialize, MyHttpObjectStructure)]
pub struct ReleaseInfoHttpModel {
    pub id: String,
    pub released_version: Option<String>,
    pub git_hub_version: Option<String>,
    pub envs: HashMap<String, String>,
}
