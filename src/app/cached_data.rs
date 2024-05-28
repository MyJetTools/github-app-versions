use std::collections::{BTreeMap, HashMap};

use crate::github::GitRepoItem;

pub struct CachedData {
    pub github_repos: Vec<GitRepoItem>,
    pub git_hub_versions: HashMap<String, String>,

    pub to_release_versions: Option<BTreeMap<String, String>>,
}

impl CachedData {
    pub fn new() -> Self {
        Self {
            github_repos: vec![],
            git_hub_versions: HashMap::new(),
            to_release_versions: None,
        }
    }

    pub fn set_github_repos(&mut self, github_repos: Vec<GitRepoItem>) {
        self.github_repos = github_repos;
    }

    pub fn get_github_repos(&self) -> Vec<GitRepoItem> {
        self.github_repos.clone()
    }

    pub fn update_github_version(&mut self, repo_id: String, version: String) {
        self.git_hub_versions.insert(repo_id, version);
    }
}
