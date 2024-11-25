use std::{collections::HashSet, sync::Arc};

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

    let mut loaded_data = HashSet::new();

    let mut debug = false;

    if let Ok(debug_value) = std::env::var("DEBUG") {
        if debug_value == "true" {
            debug = true;
        }
    }

    for env_id in envs {
        let app_infos = app.app_information_repo.get_all(&env_id).await;

        for (_, repos) in app_infos {
            for repo in repos {
                if debug {
                    println!("Reading version for repo {}", repo.id);
                }

                match crate::github::get_last_release(
                    git_hub_api_key,
                    &repo.id,
                    http_clients_cache.clone(),
                )
                .await
                {
                    Ok(ver) => {
                        if debug {
                            println!("Version for repo {} is {}", repo.id, ver);
                        }
                        loaded_data.insert(repo.id.to_string());
                        app.git_hub_versions_repo.save(repo.id, ver).await;
                    }
                    Err(err) => {
                        println!("Error reading version for repo {}. Err: {}", repo.id, err);
                    }
                }
            }
        }

        let to_release = app.tags_version_maps_repo.get_all(env_id.as_str()).await;

        for (_, ver) in to_release {
            if let Some(git_hub_repo_id) = ver.git_hub_repo_id.as_ref() {
                if loaded_data.contains(git_hub_repo_id) {
                    continue;
                }
                match crate::github::get_last_release(
                    git_hub_api_key,
                    git_hub_repo_id,
                    http_clients_cache.clone(),
                )
                .await
                {
                    Ok(ver) => {
                        if debug {
                            println!("Version for repo {} is {}", git_hub_repo_id, ver);
                        }
                        app.git_hub_versions_repo
                            .save(git_hub_repo_id.to_string(), ver)
                            .await;
                    }
                    Err(err) => {
                        println!(
                            "Error reading version for repo {}. Err: {}",
                            git_hub_repo_id, err
                        );
                    }
                }
            }
        }
    }
}
