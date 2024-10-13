use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Course{
    pub fullname: String
}