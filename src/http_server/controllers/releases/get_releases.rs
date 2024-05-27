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
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let repos = action.app.settings_reader.get_repos().await;

    let mut released_version = action.app.cache.lock().await.get_released_versions();

    let mut github_versions = action.app.cache.lock().await.git_hub_versions.clone();

    let mut result: BTreeMap<String, Vec<ReleaseInfoHttpModel>> = BTreeMap::new();

    for (group, repos) in repos {
        result.insert(group.clone(), vec![]);

        for repo in repos {
            let envs = HashMap::new();
            let git_hub_version = github_versions.remove(&repo.id);
            result
                .get_mut(group.as_str())
                .unwrap()
                .push(ReleaseInfoHttpModel {
                    id: repo.id,
                    released_version: released_version.remove(repo.release_version_tag.as_str()),
                    git_hub_version,
                    envs,
                });
        }
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
