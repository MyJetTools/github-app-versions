use crate::{app::AppContext, db::TagVersionMapDto};

pub async fn set_version_to_release(
    app: &AppContext,
    env_id: String,
    app_id: &str,
    version: String,
) -> Result<(), String> {
    let version_tag = app.app_information_repo.get(&env_id, app_id).await;

    if version_tag.is_none() {
        return Err(format!("Service {} not found", app_id));
    }

    let version_tag = version_tag.unwrap();

    app.tags_version_maps_repo
        .insert_or_update(TagVersionMapDto {
            env: env_id.to_string(),
            tag: version_tag.release_version_tag,
            version,
        })
        .await;

    Ok(())
}
