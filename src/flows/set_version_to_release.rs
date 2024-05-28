use std::collections::BTreeMap;

use crate::{app::AppContext, scripts::VersionsYamlModel};

pub async fn set_version_to_release(
    app: &AppContext,
    service_id: &str,
    version: String,
) -> Result<(), String> {
    let mut write_access = app.cache.lock().await;

    let version_tag = app
        .settings_reader
        .get_service_version_tag(service_id)
        .await;

    if version_tag.is_none() {
        return Err(format!("Service {} not found", service_id));
    }

    let version_tag = version_tag.unwrap();

    if write_access.to_release_versions.is_none() {
        write_access.to_release_versions = Some(BTreeMap::new());
    }

    let to_release_versions = write_access.to_release_versions.as_mut().unwrap();
    to_release_versions.insert(version_tag, version);

    let to_write_model = VersionsYamlModel {
        vars: to_release_versions.clone(),
    };

    let payload = serde_yaml::to_string(&to_write_model).unwrap();

    let file_name = app.settings_reader.get_versions_yaml_file_path().await;

    tokio::fs::write(file_name, payload).await.unwrap();

    Ok(())
}
