use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};

use super::dto::*;
pub const TABLE_NAME: &str = "app_version_tags";

pub struct AppInformationRepo {
    sqlite: SqlLiteConnection,
}

impl AppInformationRepo {
    pub async fn new(path: String) -> Self {
        Self {
            sqlite: SqlLiteConnectionBuilder::new(path)
                .create_table_if_no_exists::<AppVersionTagNameDto>(TABLE_NAME)
                .build()
                .await
                .unwrap(),
        }
    }

    pub async fn get_all(&self, env_id: &str) -> Vec<AppVersionTagNameDto> {
        let where_model = WhereByAppVersionTagEnv { env: env_id };
        self.sqlite
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap()
    }

    pub async fn get(&self, env_id: &str, app_id: &str) -> Option<AppVersionTagNameDto> {
        let where_model = WhereByAppId {
            env: env_id,
            app_id,
        };
        self.sqlite
            .query_single_row(TABLE_NAME, Some(&where_model))
            .await
            .unwrap()
    }

    pub async fn insert_or_update(&self, dto: AppVersionTagNameDto) {
        self.sqlite
            .insert_or_update_db_entity(TABLE_NAME, &dto)
            .await
            .unwrap();
    }
}
