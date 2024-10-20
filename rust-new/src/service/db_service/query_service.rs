use std::env;
use std::sync::Arc;
use sqlx::{Error, PgPool, Row};
use crate::model::Assignment_Model::Assignment;
use crate::model::Course::Course;
use crate::model::Label_Model::{Label, LabelDocument};
use crate::model::Quiz_Model::Quiz;
use crate::model::User::User;
use crate::model::User_Course_Model::UserCourse;

pub struct DatabaseService {
    pub pool: PgPool
}

impl DatabaseService {
    pub async fn new() -> Result<Self, Error >{
        let db_url = env::var("DB_URL").expect("DB_URL must be set");
        let pool = PgPool::connect(&db_url).await?;
        Ok(DatabaseService{pool})
    }
    pub async fn get_user_course(&self) -> Result<Vec<UserCourse>, Error>{
         let rows = sqlx::query_as::<_ , UserCourse>(
            r#"
            SELECT u.id AS user_id, c.id AS course_id
            FROM mdl_user u
            JOIN mdl_user_enrolments ue ON u.id = ue.userid
            JOIN mdl_enrol e ON ue.enrolid = e.id
            JOIN mdl_course c ON e.courseid = c.id
            "#
            ).fetch_all(&self.pool).await?;
        Ok(rows)
    }

    pub async fn get_quiz(&self, user_course: UserCourse) -> Result<Vec<Quiz>, Error>{
        let rows = sqlx::query_as::<_, Quiz>(
            r#"
            SELECT q.id, q.course, q.name, q.timeopen, q.timeclose, q.timecreated, q.timemodified
                    FROM mdl_quiz q
                    LEFT JOIN mdl_quiz_attempts qa ON q.id = qa.quiz AND qa.userid = $1
                    WHERE q.course = $2
                    AND (qa.state IS NULL OR qa.state != 'finished')
            "#
        ).bind(user_course.user_id).bind(user_course.course_id)
        .fetch_all(&self.pool).await?;
        Ok(rows)
    }
    pub async fn get_assign(&self, user_course: UserCourse) -> Result<Vec<Assignment>, Error>{
        let rows = sqlx::query_as::<_, Assignment>(
            r#"
             SELECT a.id, a.course, a.name, a.duedate, a.allowsubmissionsfromdate
                    FROM mdl_assign a
                    LEFT JOIN mdl_assign_submission a_s ON a.id = a_s.assignment AND a_s.userid = $1
                    WHERE a.course = $2
                    AND (a_s.status IS NULL OR a_s.status != 'submitted')
            "#
        ).bind(user_course.user_id).bind(user_course.course_id)
        .fetch_all(&self.pool).await?;
        Ok(rows)
    }
    pub async fn get_label(&self, user_course: UserCourse) -> Result<Vec<Label>, Error>{
        let rows = sqlx::query_as::<_, Label>(
            r#"
           SELECT l.id, l.course, l.name, cm.added, l.timemodified
                    FROM mdl_label l
                    LEFT JOIN mdl_course_modules cm ON cm.instance = l.id
                    LEFT JOIN mdl_course_modules_completion cmc ON cm.id = cmc.coursemoduleid AND cmc.userid = $1
                    WHERE cm.module = 14
                    AND l.course = $2
                    AND (cmc.userid IS NULL OR cmc.completionstate != 1);
            "#
        ).bind(user_course.user_id).bind(user_course.course_id)
        .fetch_all(&self.pool).await?;
        Ok(rows)
    }

    pub async fn get_username(&self, user_id: i64)->Result<User, Error>{
        let row = sqlx::query_as::<_, User>(
            r#"
          SELECT id, firstname, lastname FROM mdl_user where id = $1
            "#
        ).bind(user_id).fetch_one(&self.pool).await?;
        Ok(row)
    }

    pub async fn get_coursename(&self, course_id: i64)->Result<Course, Error>{
        let row = sqlx::query_as::<_, Course>(
            r#"
          SELECT fullname FROM mdl_course where id = $1
            "#
        ).bind(course_id).fetch_one(&self.pool).await?;
        Ok(row)
    }

    pub async fn get_last_update(&self) -> Result<i64, Error>{
        let row = sqlx::query_scalar(
            r#"
                SELECT last_update from service_checker
            "#
        ).fetch_optional(&self.pool).await?;
        let result  = row.unwrap();
        Ok(result)
    }

    pub async fn get_all_label(&self) -> Result<Vec<LabelDocument>, Error>{
        let rows = sqlx::query_as::<_, LabelDocument>(
            r#"
           SELECT * FROM mdl_label;
            "#
        ).fetch_all(&self.pool).await?;
        Ok(rows)
    }

}
