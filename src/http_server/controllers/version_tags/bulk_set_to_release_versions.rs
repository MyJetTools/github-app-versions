use std::{collections::BTreeMap, sync::Arc};

use my_http_server::{
    macros::*,
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/Releases/ToReleaseVersions",
    controller: "VersionTags",
    description: "Bulk Set release yaml",
    summary: "Bulk Set release yaml",
    input_data: BulkSetVersionInputData,
 
    result:[
        {status_code: 200, description: "Git list of releases"},
    ]
)]
pub struct BulkSetToReleaseVersionsYamlAction {
    app: Arc<AppContext>,
}

impl BulkSetToReleaseVersionsYamlAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &BulkSetToReleaseVersionsYamlAction,
    input_data: BulkSetVersionInputData,
    ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {


    let env = action.app.resolve_env_id(ctx).await?;
    let input_data: Result<SetBulkInsertModel, _> = serde_yaml::from_slice(&input_data.body);

    if let Err(err) = &input_data{
        return HttpFailResult::as_validation_error(format!("{:?}", err)).into_err();
    }

    let input_data = input_data.unwrap();

    action.app.tags_version_maps_repo.bulk_insert_or_update(&env, input_data.vars).await;
   

    HttpOutput::Empty.into_ok_result(true).into()
}



#[derive(Debug, MyHttpInput)]
pub struct BulkSetVersionInputData {
    #[http_body_raw(description = "Repository id")]
    pub body: Vec<u8>,
}


#[derive(Debug, Deserialize)]
pub struct SetBulkInsertModel{
    pub vars: BTreeMap<String, String>
}