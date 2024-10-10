use sqlx::FromRow;
#[derive(Debug, FromRow)]
pub struct Label {
    id: i64,
    course: i64,
    name: String,
    added: i64,
    timemodified: i64
}