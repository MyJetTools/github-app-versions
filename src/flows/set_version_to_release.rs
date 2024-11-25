use crate::app::AppContext;

pub async fn set_version_to_release(
    app: &AppContext,
    env_id: String,
    app_id: &str,
    version: String,
    git_hub_name: Option<String>,
) -> Result<(), String> {
    let version_tag = app.app_information_repo.get(&env_id, app_id).await;

    if version_tag.is_none() {
        return Err(format!("Service {} not found", app_id));
    }

    let version_tag = version_tag.unwrap();

    app.tags_version_maps_repo
        .insert_or_update(
            env_id.as_str(),
            version_tag.release_version_tag,
            version,
            git_hub_name,
        )
        .await;

    Ok(())
}
