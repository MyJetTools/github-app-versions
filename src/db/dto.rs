use my_sqlite::macros::*;

#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct TagVersionMapDto {
    #[primary_key(0)]
    pub env: String,
    #[primary_key(1)]
    pub tag: String,
    pub version: String,
}

#[derive(WhereDbModel)]
pub struct WhereAppVersion<'s> {
    pub env: &'s str,
    pub tag: &'s str,
}

#[derive(WhereDbModel)]
pub struct WhereOfEnvModel<'s> {
    pub env: &'s str,
}

// Apps Tags
#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct AppVersionTagNameDto {
    #[primary_key(0)]
    pub env: String,
    #[primary_key(1)]
    pub app_id: String,
    #[primary_key(2)]
    pub release_version_tag: String,
    pub group: String,
}
#[derive(WhereDbModel)]
pub struct WhereByAppVersionTagEnv<'s> {
    pub env: &'s str,
}

#[derive(WhereDbModel)]
pub struct WhereByAppId<'s> {
    pub env: &'s str,
    pub app_id: &'s str,
}
