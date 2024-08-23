use std::{collections::BTreeMap, sync::Arc};

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::Deserialize;

use crate::{app::AppContext, db::AppVersionTagNameDto};

#[http_route(
    method: "POST",
    route: "/api/App/BulkInsert",
    controller: "Repos",
    description: "Get release yaml",
    summary: "Get release yaml",
    input_data: BulkInsertAppsInputModel,
 
    result:[
        {status_code: 204, description: "Ok Result"},
        {status_code: 403, description: "Validation error"},
    ]
)]
pub struct BulkInsertAppsHttpAction {
    app: Arc<AppContext>,
}

impl BulkInsertAppsHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &BulkInsertAppsHttpAction,
    input_data: BulkInsertAppsInputModel,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let env_id = action.app.resolve_env_id(ctx).await?;


    let model: Result<BulkInsertAppsYamlModel, _> = serde_yaml::from_slice(&input_data.body);

    if let Err(err) = &model{
        return HttpFailResult::as_validation_error(format!("{:?}", err)).into_err();
    }


    for (group, repos) in model.unwrap().repos{
        for repo in repos{
            action.app.app_information_repo.insert_or_update(AppVersionTagNameDto{
                env: env_id.to_string(),
                app_id: repo.id,
                release_version_tag: repo.release_version_tag,
                group: group.clone(),
            }).await;
        }
    }

    
   HttpOutput::Empty.into_ok_result(false)
}


#[derive(Debug, MyHttpInput)]
pub struct BulkInsertAppsInputModel {
    #[http_body_raw(description = "Apps")]
    pub body: Vec<u8>,
}



#[derive(Debug, Deserialize)]
pub struct BulkInsertAppsYamlModel{
    pub repos: BTreeMap<String, Vec<AppModel>>
}

#[derive(Debug, Deserialize)]
pub  struct AppModel{
    pub id: String,
    pub release_version_tag : String,
}