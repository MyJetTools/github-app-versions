use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};

use super::dto::*;
pub const TABLE_NAME: &str = "app_versions";
pub struct TagsVersionMapsRepo {
    sqlite: SqlLiteConnection,
}

impl TagsVersionMapsRepo {
    pub async fn new(path: String) -> Self {
        Self {
            sqlite: SqlLiteConnectionBuilder::new(path)
                .create_table_if_no_exists::<TagVersionMapDto>(TABLE_NAME)
                .build()
                .await
                .unwrap(),
        }
    }

    pub async fn insert_or_update(&self, dto: TagVersionMapDto) {
        self.sqlite
            .insert_or_update_db_entity(TABLE_NAME, &dto)
            .await
            .unwrap();
    }

    pub async fn get_all(&self, env_id: &str) -> Vec<TagVersionMapDto> {
        let where_model = WhereOfEnvModel { env: env_id };

        self.sqlite
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap()
    }
}
