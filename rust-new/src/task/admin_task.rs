use std::collections::HashMap;
use std::mem::take;
use std::sync::Arc;
use chrono::{DateTime, Duration, Local, Timelike, TimeZone};
use lambda_runtime::Error;
use tokio::task::JoinHandle;
use crate::model::Reminder::{create_reminder_request, ReminderRequest};
use crate::model::User_Course_Model::UserCourse;
use crate::service::db_service::query_service::DatabaseService;
use crate::service::api_service::reminder_service::Reminder;

use chrono_tz;
use chrono_tz::Tz;
use serde_json::{json, Value};
use tokio::task;


// Your existing database service struct and models (e.g., UserCourse, Quiz) go here.
#[derive(Clone)]
pub struct Admin_task {
    db: Arc<DatabaseService>,
    interval: Duration,
    reminder_service: Arc<Reminder>,
    tz: Tz,

}

impl Admin_task {
    // Constructor for `App` to initialize with the DB connection pool
    pub async fn new() -> Result<Self, Error> {
        let db = Arc::new(DatabaseService::new().await?);
        let interval = Duration::seconds(60);
        let reminder_service = Arc::new(Reminder::new().await?);
        let tz = chrono_tz::Asia::Bangkok;

        Ok(Self { db, interval, reminder_service, tz })
    }

    pub fn check_time(&self, reminder_time: String, last_update_time:i64) -> bool {
        let last_update_time = self.tz.timestamp_opt(last_update_time,0).unwrap();
        let last_update_hour = last_update_time.hour();
        let last_update_minute = last_update_time.minute();
        let hour = self.tz.from_utc_datetime(&chrono::Utc::now().naive_utc()).hour();
        let minute = self.tz.from_utc_datetime(&chrono::Utc::now().naive_utc()).minute();

        let mut h_m = reminder_time.split(":");
        let hour_remind = match h_m.next().and_then(|h| h.parse::<u32>().ok()) {
            Some(h) => h,
            None => {
                eprintln!("Failed to parse hour");
                return false;
            }
        };

        let minute_remind = match h_m.next().and_then(|m| m.parse::<u32>().ok()) {
            Some(m) => m,
            None => {
                eprintln!("Failed to parse minute");
                return false;
            }
        };
        println!("NOW: {hour} - {minute}");
        println!("REMIND: {hour_remind} - {minute_remind}");
        (hour == hour_remind) && (minute >= minute_remind) && (hour_remind == last_update_hour) && (minute_remind > last_update_minute)
    }
    pub async fn check_send_reminder(&self) -> Result<bool, Error>{
        let reminder_time = self.db.get_reminder_time().await?;
        let last_update_time = self.db.get_last_update().await?;

        println!("REMINDER_TIME {reminder_time:?}");
        let reminder_value = match reminder_time.value {
            Some(val) => val,
            None => {
                return Err("Reminder time not found.".into());
            }
        };
         //if true{
        if self.check_time(reminder_value, last_update_time){
            let users = self.db.get_user_daily_check().await?;
            let mut tasks = Vec::new();
            for u in users{
                let app_clone = self.clone();
                let u_clone = u.clone();
                let task = task::spawn(async move{
                    // spawn a new task
                    let result = app_clone.db.get_completion(u_clone.id).await;
                    match result {
                        Ok(rows) => {
                            let mut structured_data: HashMap<String, HashMap<String, Value>> = HashMap::new();

                            for row in rows {
                                let course_entry = structured_data.entry(row.fullname.clone()).or_insert_with(|| {
                                    let mut course = HashMap::new();
                                    course.insert("course_id".to_string(), json!(row.course_id));
                                    course.insert("components".to_string(), json!({}));
                                    course
                                });
                                let mut component_name = row.module_name.clone();
                                if row.module_name == "label" {
                                    component_name = "chapter".to_string();
                                }
                                let components = course_entry.get_mut("components")
                                    .and_then(|v| v.as_object_mut())
                                    .unwrap();

                                let mut component_entry = components.entry(component_name.clone()).or_insert_with(|| {
                                    json!({
                                "completed": 0,
                                "total": 0,
                                "completion_percentage": 0.0
                            })
                                });

                                if let Some(obj) = component_entry.as_object_mut() {
                                    obj.entry("completed")
                                        .and_modify(|v| *v = json!(v.as_i64().unwrap() + row.completed_modules as i64));
                                    obj.entry("total")
                                        .and_modify(|v| *v = json!(v.as_i64().unwrap() + row.total_modules as i64));
                                    obj.insert("completion_percentage".to_string(), json!(row.completion_percentage));
                                }
                                //self.reminder_service.push_message()
                            }
                            let json_data = serde_json::to_string_pretty(&structured_data).unwrap();
                            let current_local: DateTime<Local> = Local::now();
                            let now = current_local.format("%Y-%m-%d %H:%M:%S");
                            println!("NOW: {now:?}");

                            println!("DATA SEND: {json_data:?}");
                            let reminder_content = create_reminder_request("daily".to_string(), json_data.clone(), u_clone.id, format!("{} {}", u_clone.firstname, u_clone.lastname), 1, "course".to_string(), "was created".to_string(), now.to_string(), now.to_string());
                            match app_clone.reminder_service.push_message(reminder_content).await {
                                Ok(content) => println!("PUSH MESSAGE SUCCESS: {:?}", content),
                                Err(e) => eprintln!("Error sending message: {:?}", e),
                            }
                        }

                        Err(e) => eprintln!("Error fetching completion for user {}: {:?}", u_clone.id, e),
                    }
                });
                tasks.push(task);
            }
            for task in tasks {
                if let Err(e) = task.await {
                    eprintln!("Task failed: {:?}", e);
                }
            }
        } else {
            // do nothing
        }

        Ok(true)
    }


    pub async fn run(&self) -> Result<&'static str, Error> {
        let cloned_self = Arc::new(self.clone()); // Clone self into Arc

        let task = task::spawn(  async move {
            let result: Result<bool, Error> = cloned_self.check_send_reminder().await;

            match result {
                Ok(quizzes) => println!("Quizzes: {:?}", quizzes),
                Err(e) => eprintln!("Error fetching quizzes: {:?}", e),
            }
        });

        Ok("Success")
    }
}