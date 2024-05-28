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
    let repos = app.settings_reader.get_repos().await;

    for (_, repos) in repos {
        for repo in repos {
            match crate::github::get_last_release(
                git_hub_api_key,
                &repo.id,
                http_clients_cache.clone(),
            )
            .await
            {
                Ok(ver) => {
                    app.cache.lock().await.update_github_version(repo.id, ver);
                }
                Err(err) => {
                    println!("Error reading version for repo {}. Err: {}", repo.id, err);
                }
            }
        }
    }
}
