#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: i32,
    pub email: String,
    pub password_hash: String
}

#[derive(sea_query::Iden)]
pub enum DbUserIden {
    #[iden = "db_user"]
    Table,
    Id,
    Email,
    PasswordHash
}
