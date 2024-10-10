use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Quiz {
    pub id: i64,
    pub course: i64,
    pub name: String,
    pub timeopen: i64,
    pub timeclose: i64,
    pub timecreated: i64,
    pub timemodified: i64
}