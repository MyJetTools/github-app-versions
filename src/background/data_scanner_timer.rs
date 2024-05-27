use std::sync::Arc;

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

        let repos = crate::github::list_of_repos(&git_hub_api_key).await;
        self.app.cache.lock().await.set_github_repos(repos);

        let versions = crate::flows::get_released_versions(&self.app).await;
        self.app.cache.lock().await.set_released_versions(versions);

        read_versions(&self.app, &git_hub_api_key).await;

        sw.pause();
        println!(
            "DataScannerTimer tick finished in {}",
            sw.duration_as_string()
        );
    }
}

async fn read_versions(app: &AppContext, git_hub_api_key: &str) {
    let repos = app.settings_reader.get_repos().await;

    for (_, repos) in repos {
        for repo in repos {
            match crate::github::get_last_release(git_hub_api_key, &repo.id).await {
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