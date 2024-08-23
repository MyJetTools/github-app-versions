use std::collections::BTreeMap;

use crate::app::AppContext;

pub async fn get_to_release_versions(app: &AppContext, env_id: &str) -> BTreeMap<String, String> {
    let app_versions = app.tags_version_maps_repo.get_all(env_id).await;

    let mut result = BTreeMap::new();

    for itm in app_versions {
        result.insert(itm.tag, itm.version);
    }

    result
}
