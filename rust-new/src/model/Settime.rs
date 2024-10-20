use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Settime{
    pub name: Option<String>,
    pub value: Option<String>

}