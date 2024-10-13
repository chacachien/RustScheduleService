
use sqlx::{Error, PgPool, Row};
use crate::model::Assignment_Model::Assignment;
use crate::model::Course::Course;
use crate::model::Label_Model::Label;
use crate::model::Quiz_Model::Quiz;
use crate::model::User::User;
use crate::model::User_Course_Model::UserCourse;



impl crate::service::db_service::query_service::DatabaseService {

    pub async fn store_reminder(&self) {


    }

    pub async fn update_last_update(&self, time: i64) -> Result<&'static str, Error>{
        let row = sqlx::query(
            r#"
                UPDATE service_checker
                SET last_update = $1
            "#
        ).bind(time).execute(&self.pool).await?;
        Ok("Update successful!")
    }
}