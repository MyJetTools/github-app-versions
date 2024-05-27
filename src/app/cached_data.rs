use std::collections::HashMap;

use crate::github::GitRepoItem;

pub struct CachedData {
    pub github_repos: Vec<GitRepoItem>,
    pub released_versions: HashMap<String, String>,
    pub git_hub_versions: HashMap<String, String>,
}

impl CachedData {
    pub fn new() -> Self {
        Self {
            github_repos: vec![],
            released_versions: HashMap::new(),
            git_hub_versions: HashMap::new(),
        }
    }

    pub fn set_github_repos(&mut self, github_repos: Vec<GitRepoItem>) {
        self.github_repos = github_repos;
    }

    pub fn get_github_repos(&self) -> Vec<GitRepoItem> {
        self.github_repos.clone()
    }

    pub fn set_released_versions(&mut self, value: HashMap<String, String>) {
        self.released_versions = value;
    }

    pub fn get_released_versions(&self) -> HashMap<String, String> {
        self.released_versions.clone()
    }

    pub fn update_github_version(&mut self, repo_id: String, version: String) {
        self.git_hub_versions.insert(repo_id, version);
    }
}
