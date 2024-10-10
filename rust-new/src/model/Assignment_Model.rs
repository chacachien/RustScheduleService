use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Assignment {
    pub id: i64,
    pub course: i64,
    pub name: String,
    pub allowsubmissionsfromdate: i64,
    pub duedate: i64,
}