use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(super::releases::GetReleasesAction::new(app.clone()).into());
    result.register_get_action(super::releases::GetGitHubReleasesAction::new(app.clone()).into());
    result.register_get_action(super::releases::GetGitHubVersionAction::new(app.clone()).into());

    result.register_get_action(
        super::releases::GetToReleaseVersionsYamlAction::new(app.clone()).into(),
    );

    result
        .register_post_action(super::releases::SetVersionsToReleaseAction::new(app.clone()).into());

    result
}
