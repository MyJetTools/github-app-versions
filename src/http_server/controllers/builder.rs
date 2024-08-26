use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_post_action(super::apps::BulkInsertAppsHttpAction::new(app.clone()).into());
    result.register_post_action(super::apps::InsertAppHttpAction::new(app.clone()).into());
    result.register_post_action(super::apps::RenameAppIdHttpAction::new(app.clone()).into());

    result.register_get_action(super::releases::GetReleasesAction::new(app.clone()).into());
    result.register_get_action(super::releases::GetGitHubReleasesAction::new(app.clone()).into());
    result.register_get_action(super::releases::GetGitHubVersionAction::new(app.clone()).into());

    result
        .register_post_action(super::releases::SetVersionsToReleaseAction::new(app.clone()).into());

    result.register_get_action(
        super::version_tags::GetToReleaseVersionsYamlAction::new(app.clone()).into(),
    );

    result.register_post_action(
        super::version_tags::SetToReleaseVersionHttpAction::new(app.clone()).into(),
    );

    result.register_post_action(
        super::version_tags::BulkSetToReleaseVersionsYamlAction::new(app.clone()).into(),
    );

    result
}
