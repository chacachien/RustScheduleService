use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct User{
    pub id: i64,
    pub firstname: String,
    pub lastname: String
}