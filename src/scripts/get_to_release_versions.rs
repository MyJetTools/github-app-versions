use std::collections::BTreeMap;

use crate::app::AppContext;

pub async fn get_to_release_versions(app: &AppContext, env_id: &str) -> BTreeMap<String, String> {
    app.tags_version_maps_repo.get_all(env_id).await
}
