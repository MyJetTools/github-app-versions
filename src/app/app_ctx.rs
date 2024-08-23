use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult};
use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::{
    db::{AppInformationRepo, TagsVersionMapsRepo},
    settings::SettingsReader,
};

use super::CachedData;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub settings_reader: SettingsReader,
    pub states: Arc<AppStates>,
    pub cache: Mutex<CachedData>,
    pub tags_version_maps_repo: TagsVersionMapsRepo,

    pub app_information_repo: AppInformationRepo,
}

impl AppContext {
    pub async fn new(settings_reader: SettingsReader) -> Self {
        let tags_db_path = settings_reader.get_app_versions_db_path("tags").await;
        let app_tags_db_path = settings_reader.get_app_versions_db_path("apps-tags").await;
        Self {
            settings_reader,
            states: AppStates::create_initialized().into(),
            cache: Mutex::new(CachedData::new()),
            tags_version_maps_repo: TagsVersionMapsRepo::new(tags_db_path).await,
            app_information_repo: AppInformationRepo::new(app_tags_db_path).await,
        }
    }

    pub async fn resolve_env_id(&self, ctx: &HttpContext) -> Result<String, HttpFailResult> {
        let domain = ctx.request.get_host();
        match self.settings_reader.find_env_id(domain).await {
            Some(env_id) => Ok(env_id),
            None => Err(HttpFailResult::as_unauthorized(
                format!("Env not found for domain {}", domain).into(),
            )),
        }
    }
}
