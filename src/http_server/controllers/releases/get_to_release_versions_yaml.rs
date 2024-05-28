use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use crate::{app::AppContext, scripts::VersionsYamlModel};

#[http_route(
    method: "GET",
    route: "/api/Releases/ToReleaseVersions",
    controller: "Releases",
    description: "Get release yaml",
    summary: "Get release yaml",
 
    result:[
        {status_code: 200, description: "Git list of releases"},
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
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
 
    let result = {
        let read_access = action.app.cache.lock().await;

        match read_access.to_release_versions.as_ref(){
            Some(to_release_versions) => {
                VersionsYamlModel{
                    vars: to_release_versions.clone()
                }
            }
            None => {
                VersionsYamlModel{
                    vars: Default::default()
                }
            }
        }
    };

    HttpOutput::as_yaml(result).into_ok_result(true).into()
}

#[derive(Debug, MyHttpInput)]
pub struct GetGitHubVersionInputData {
    #[http_query(name:"repoId", description = "Repository id")]
    pub repo_id: String,
}
