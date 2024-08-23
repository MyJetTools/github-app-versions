use crate::github::GitRepoItem;

pub struct CachedData {
    pub github_repos: Vec<GitRepoItem>,
}

impl CachedData {
    pub fn new() -> Self {
        Self {
            github_repos: vec![],
        }
    }

    pub fn set_github_repos(&mut self, github_repos: Vec<GitRepoItem>) {
        self.github_repos = github_repos;
    }

    pub fn get_github_repos(&self) -> Vec<GitRepoItem> {
        self.github_repos.clone()
    }
}
