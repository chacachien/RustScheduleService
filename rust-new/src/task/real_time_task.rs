use std::sync::Arc;
use chrono::{ Duration, TimeZone};
use lambda_runtime::Error;
use tokio::task::JoinHandle;
use crate::model::Reminder::{create_reminder_request, ReminderRequest};
use crate::model::User_Course_Model::UserCourse;
use crate::service::db_service::query_service::DatabaseService;
use crate::service::api_service::reminder_service::Reminder;

use chrono_tz;
use chrono_tz::Tz;

// Your existing database service struct and models (e.g., UserCourse, Quiz) go here.
#[derive(Clone)]
pub struct Realtime_task {
    db: Arc<DatabaseService>,
    interval: Duration,
    reminder_service: Arc<Reminder>,
    tz: Tz,
    before_reminder: i64
}

impl Realtime_task {
    // Constructor for `App` to initialize with the DB connection pool
    pub async fn new() -> Result<Self, Error> {

        let db = Arc::new(DatabaseService::new().await?);
        let interval = Duration::seconds(60);
        let reminder_service = Arc::new(Reminder::new().await?);
        let tz = chrono_tz::Asia::Bangkok;
        let before_reminder = 86400;
        Ok(Self { db, interval, reminder_service, tz, before_reminder })
    }

    pub fn check_time(&self, timestamp: i64, last_update_time: i64) -> bool {
        let now = self.tz.from_utc_datetime(&chrono::Utc::now().naive_utc()).timestamp();
        let condition = (timestamp > last_update_time) && (timestamp < now);
        return condition;
    }
    // This function checks the quiz for a given `UserCourse`
    pub async fn check_quiz(&self, user_course: Arc<UserCourse>, last_update_time: i64) -> Result<bool, Error> {
        let user_course = Arc::try_unwrap(user_course).unwrap_or_else(|arc| (*arc).clone());

        let quizzes = self.db.get_quiz(user_course.clone()).await?;
        let user = self.db.get_username(user_course.user_id).await?;
        let user_name = format!("{} {} ", user.lastname, user.firstname); // Assuming there's a middlename field
        let course= self.db.get_coursename(user_course.course_id).await?;

        for quiz in quizzes {
            if(self.check_time(quiz.timecreated, last_update_time)){
                let time_created_event = self.tz.timestamp_opt(quiz.timecreated,0).unwrap();
                let formatted_time = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();

                let reminder_content = create_reminder_request("quiz".to_string(), quiz.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "was created".to_string(), formatted_time.clone(), formatted_time.clone());
                let content = self.reminder_service.push_message(reminder_content).await?;
            }

            if(self.check_time(quiz.timeopen, last_update_time)){
                let time_created_event = self.tz.timestamp_opt(quiz.timeopen,0).unwrap();
                let formatted_time = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();
                let reminder_content = create_reminder_request("quiz".to_string(), quiz.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "was opened".to_string(), formatted_time.clone(), formatted_time.clone());
                let content = self.reminder_service.push_message(reminder_content).await?;
            }

            let time_close = quiz.timeclose - self.before_reminder;
            if(self.check_time(time_close, last_update_time)){
                let time_close_reminder = self.tz.timestamp_opt(quiz.timeclose,0).unwrap();
                let time_created_event = self.tz.timestamp_opt(time_close,0).unwrap();
                let time_close_reminder_format = time_close_reminder.format("%Y-%m-%d %H:%M:%S").to_string();
                let time_create_event_format = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();

                let reminder_content = create_reminder_request("quiz".to_string(), quiz.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "will close".to_string(), time_close_reminder_format, time_create_event_format);
                let content = self.reminder_service.push_message(reminder_content).await?;
            }
        }
        Ok(true)
    }

    pub async fn check_assign(&self, user_course: Arc<UserCourse>, last_update_time: i64) -> Result<bool, Error> {
        let user_course = Arc::try_unwrap(user_course).unwrap_or_else(|arc| (*arc).clone());

        let assignments = self.db.get_assign(user_course.clone()).await?;
        let user = self.db.get_username(user_course.user_id).await?;
        let user_name = format!("{} {} ", user.lastname, user.firstname); // Assuming there's a middlename field
        let course= self.db.get_coursename(user_course.course_id).await?;

        for assign in assignments {
            if(self.check_time(assign.allowsubmissionsfromdate, last_update_time)){
                let time_created_event = self.tz.timestamp_opt(assign.allowsubmissionsfromdate,0).unwrap();
                let time_create_event_format = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();
                let reminder_content = create_reminder_request("assign".to_string(), assign.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "was allowed to submit".to_string(), time_create_event_format.clone(), time_create_event_format.clone());
                let content = self.reminder_service.push_message(reminder_content).await?;
            }
            let before_duedate = assign.duedate - self.before_reminder;
            if(self.check_time(before_duedate, last_update_time)){
                let time_close_reminder = self.tz.timestamp_opt(assign.duedate,0).unwrap();
                let time_created_event = self.tz.timestamp_opt(before_duedate,0).unwrap();
                let time_create_event_format = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();
                let time_close_event_format = time_close_reminder.format("%Y-%m-%d %H:%M:%S").to_string();

                let reminder_content = create_reminder_request("assign".to_string(), assign.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "will duedate".to_string(), time_close_event_format, time_create_event_format);
                let content = self.reminder_service.push_message(reminder_content).await?;
            }
        }

        Ok(true)
    }

    pub async fn check_label(&self, user_course: Arc<UserCourse>, last_update_time: i64) -> Result<bool, Error> {
        let user_course = Arc::try_unwrap(user_course).unwrap_or_else(|arc| (*arc).clone());

        let labels = self.db.get_label(user_course.clone()).await?;
        let user = self.db.get_username(user_course.user_id).await?;
        let user_name = format!("{} {} ", user.lastname, user.firstname); // Assuming there's a middlename field
        let course= self.db.get_coursename(user_course.course_id).await?;

        for label in labels {
            if(self.check_time(label.added, last_update_time)){
                let time_created_event = self.tz.timestamp_opt(label.added,0).unwrap();
                let time_create_event_format = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();
                let reminder_content = create_reminder_request("label".to_string(), label.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "was added".to_string(), time_create_event_format.clone(), time_create_event_format.clone());
                let content = self.reminder_service.push_message(reminder_content).await?;
            }
            if(self.check_time(label.timemodified, last_update_time)){
                let time_created_event = self.tz.timestamp_opt(label.timemodified,0).unwrap();
                let time_create_event_format = time_created_event.format("%Y-%m-%d %H:%M:%S").to_string();
                let reminder_content = create_reminder_request("label".to_string(), label.name.clone(), user_course.user_id, user_name.to_string(), user_course.course_id, course.fullname.to_string(), "was modified".to_string(), time_create_event_format.clone(), time_create_event_format.clone());
                let content = self.reminder_service.push_message(reminder_content).await?;
            }
        }
        Ok(true)
    }

    // Main function to run all tasks
    pub async fn run(&self) -> Result<&'static str, Error> {
        println!("30s task start");
        let app = Arc::new(self.clone());
        let last_update_time = self.db.get_last_update().await?;
        // Fetch user courses from the database
        let user_courses = self.db.get_user_course().await?;
        let mut tasks: Vec<JoinHandle<()>> = vec![];

        // Spawn tasks for each user course
        for user_course in user_courses {
            let app_clone = Arc::clone(&app);
            let user_course = Arc::new(user_course); // Wrap user_course in Arc.

            let task = tokio::spawn({
                let user_course = Arc::clone(&user_course);
                let app_clone = Arc::clone(&app_clone);
            async move {
                match app_clone.check_quiz(user_course, last_update_time).await {
                    Ok(quizzes) => println!("Quizzes: {:?}", quizzes),
                    Err(e) => eprintln!("Error fetching quizzes: {:?}", e),
                }
            }});

            let task_assign = tokio::spawn({
                let user_course = Arc::clone(&user_course);
                let app_clone = Arc::clone(&app_clone);

                async move {
                match app_clone.check_assign(user_course, last_update_time).await {
                    Ok(assignments) => println!("Labels: {:?}", assignments),
                    Err(e) => eprintln!("Error fetching quizzes: {:?}", e),
                }
            }});

            let task_label = tokio::spawn({
                let user_course = Arc::clone(&user_course);
                let app_clone = Arc::clone(&app_clone);

                async move {
                    match app_clone.check_label(user_course, last_update_time).await {
                        Ok(labels) => println!("Labels: {:?}", labels),
                        Err(e) => eprintln!("Error fetching quizzes: {:?}", e),
                    }
                }});

            tasks.push(task);
            tasks.push(task_assign);
            tasks.push(task_label);
        }

        // Wait for all tasks to complete
        for task in tasks {
            task.await?;
        }
        Ok("All tasks done")
    }
}
