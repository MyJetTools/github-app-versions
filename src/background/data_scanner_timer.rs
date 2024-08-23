use std::sync::Arc;

use flurl::HttpClientsCache;
use rust_extensions::{MyTimerTick, StopWatch};

use crate::app::AppContext;

pub struct DataScannerTimer {
    app: Arc<AppContext>,
}

impl DataScannerTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for DataScannerTimer {
    async fn tick(&self) {
        println!("DataScannerTimer tick started");
        let mut sw = StopWatch::new();
        sw.start();

        let git_hub_api_key = self.app.settings_reader.get_git_hub_api_key().await;

        let http_clients_cache = Arc::new(flurl::HttpClientsCache::new());

        let repos =
            crate::github::list_of_repos(&git_hub_api_key, http_clients_cache.clone()).await;
        self.app.cache.lock().await.set_github_repos(repos);

        read_versions(&self.app, &git_hub_api_key, http_clients_cache).await;

        sw.pause();
        println!(
            "DataScannerTimer tick finished in {}",
            sw.duration_as_string()
        );
    }
}

async fn read_versions(
    app: &AppContext,
    git_hub_api_key: &str,
    http_clients_cache: Arc<HttpClientsCache>,
) {
    let envs = app.settings_reader.get_envs().await;

    for env_id in envs {
        let app_infos = app.app_information_repo.get_all(&env_id).await;

        for (_, repos) in app_infos {
            for repo in repos {
                println!("Reading version for repo {}", repo.id);
                match crate::github::get_last_release(
                    git_hub_api_key,
                    &repo.id,
                    http_clients_cache.clone(),
                )
                .await
                {
                    Ok(ver) => {
                        println!("Version for repo {} is {}", repo.id, ver);
                        app.git_hub_versions_repo.save(repo.id, ver).await;
                    }
                    Err(err) => {
                        println!("Error reading version for repo {}. Err: {}", repo.id, err);
                    }
                }
            }
        }
    }
}
