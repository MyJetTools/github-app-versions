use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};

pub struct DbInner<T: DeserializeOwned + Serialize + Default + Clone> {
    pub path: String,
    pub file_is_created: bool,
    pub cached_data: HashMap<String, T>,
}

impl<T: DeserializeOwned + Serialize + Default + Clone> DbInner<T> {
    pub fn new(path: String) -> Self {
        Self {
            file_is_created: false,
            path,
            cached_data: HashMap::new(),
        }
    }
    pub async fn load(&mut self, env: &str, file: &str) -> T {
        if let Some(data) = self.cached_data.get(env) {
            return data.clone();
        }

        let mut path = self.path.clone();

        if !path.ends_with(std::path::MAIN_SEPARATOR) {
            path.push(std::path::MAIN_SEPARATOR);
        }

        path.push_str(env);
        path.push(std::path::MAIN_SEPARATOR);

        path.push_str(file);

        let result = match tokio::fs::read(path.as_str()).await {
            Ok(data) => match serde_yaml::from_slice(data.as_slice()) {
                Ok(result) => {
                    self.file_is_created = true;
                    result
                }
                Err(_) => {
                    panic!("Error while deserializing file {}", path)
                }
            },
            Err(_) => T::default(),
        };

        println!("Loaded file: {}", path);
        self.cached_data.insert(env.to_string(), result.clone());

        result
    }

    pub async fn save(&mut self, env: &str, file: &str, model: T) {
        let mut path = self.path.clone();

        if !path.ends_with(std::path::MAIN_SEPARATOR) {
            path.push(std::path::MAIN_SEPARATOR);
        }

        path.push_str(env);

        if !self.file_is_created {
            let _ = tokio::fs::create_dir(path.as_str()).await;
        }

        path.push(std::path::MAIN_SEPARATOR);
        path.push_str(file);

        let to_save = serde_yaml::to_string(&model).unwrap();

        tokio::fs::write(path, to_save).await.unwrap();
        self.file_is_created = true;
        self.cached_data.insert(env.to_string(), model);
    }
}
