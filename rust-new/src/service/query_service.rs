
use sqlx::{Error, PgPool, Row};
use sqlx::postgres::PgRow;
use crate::model::Assignment_Model::Assignment;
use crate::model::Label_Model::Label;
use crate::model::Quiz_Model::Quiz;
use crate::model::User_Course_Model::UserCourse;

pub struct DatabaseService {
    pub pool: PgPool
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self, Error >{
        let pool = PgPool::connect(database_url).await?;
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
}
