use sqlx::FromRow;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug)]
pub struct MessagesRemindersChatbot {
    pub id: i32,                     // serial4 in PostgreSQL maps to i32 in Rust
    pub created_at: Option<NaiveDateTime>,  // timestamp can be nullable, so it's wrapped in Option
    pub r#type: String,               // "type" is a keyword in Rust, so we use `r#type` to escape it
    pub content: String,              // varchar in PostgreSQL maps to String in Rust
    pub chat_id: i32,                 // int4 in PostgreSQL maps to i32 in Rust
    pub time_remind: i64,   // timestamp maps to NaiveDateTime for date-time values
    pub is_remind: bool,              // bool in PostgreSQL maps to bool in Rust
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReminderRequest {
    pub name: String,
    pub title: String,
    pub user_id: i64,
    pub user: String,
    pub course_id: i64,
    pub course: String,
    pub type_action: String,
    pub time_action:String,
    pub time_reminder: String
}

pub fn create_reminder_request( name: String,
                                title: String,
                                user_id: i64,
                                user: String,
                                course_id: i64,
                                course: String,
                                type_action: String,
                                time_action:String,
                               time_reminder: String)-> ReminderRequest {
    ReminderRequest {
        name: name,
        title: title,
        user_id: user_id,
        user: user,
        course_id: course_id,
        course: course,
        type_action: type_action,
        time_action: time_action,
        time_reminder: time_reminder
    }
}