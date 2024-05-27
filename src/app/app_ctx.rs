use std::sync::Arc;

use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::settings::SettingsReader;

use super::CachedData;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub settings_reader: SettingsReader,
    pub states: Arc<AppStates>,
    pub cache: Mutex<CachedData>,
}

impl AppContext {
    pub fn new(settings_reader: SettingsReader) -> Self {
        Self {
            settings_reader,
            states: AppStates::create_initialized().into(),
            cache: Mutex::new(CachedData::new()),
        }
    }
}
