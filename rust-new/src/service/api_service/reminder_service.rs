use std::env;
use std::sync::Arc;
use chrono::Duration;
use lambda_runtime::Error;
use crate::service::db_service::query_service::DatabaseService;
use crate::task::real_time_task::Realtime_task;
use reqwest::Client;
use crate::model::Reminder::{ ReminderRequest};

pub struct Reminder {
    endpoint: String,
}

impl Reminder{
    pub async fn new() -> Result<Self, Error> {
        let endpoint = env::var("BE").expect("BE must be set");

        Ok(Self { endpoint })
    }

    pub async fn push_message(
        &self,
        message: ReminderRequest,
    ) -> Result<bool, Error> {        // call post api to that endpoint with the content of reminder
        let client = Client::new();
        let response = client
            .post(&self.endpoint)
            .json(&message)  // Automatically serialize to JSON
            .send()
            .await?;
        print!("RESPONSE: {response:?}");
        if response.status().is_success() {
            println!("Message sent successfully!");
            Ok(true)
        } else {
            eprintln!("Failed to send message. Status: {}", response.status());
            Ok(false)
        }
    }
}
