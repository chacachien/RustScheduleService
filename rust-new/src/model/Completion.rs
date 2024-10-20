use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Completion{
    pub fullname: String,
    pub course_id: i64,
    pub module_name: String,
    pub completed_modules: i64,
    pub total_modules: i64,
    pub completion_percentage: i64
}