use sqlx::FromRow;

#[derive(Debug,Clone, FromRow)]
pub struct UserCourse {
    pub user_id: i64,
    pub course_id: i64,
}