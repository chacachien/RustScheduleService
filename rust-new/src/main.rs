
pub mod model;
pub mod service;
mod task;

use std::env;
use std::sync::Arc;
use std::time::Duration;
use chrono::TimeZone;
use serde_json::Value;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

use sqlx::{PgPool, Pool};
use crate::service::db_service::query_service::DatabaseService;
use crate::task::real_time_task::Realtime_task;

#[derive(Serialize, Deserialize, Debug)]
struct EventPayload{
    task: String
}
async fn function_handler(event: LambdaEvent<Value>) -> Result<&'static str, Error> {
    // Extract some useful information from the request
    let payload:EventPayload = serde_json::from_value(event.payload)?;
    // println!("Received event: {:?}", event);
     println!("Payload: {:?}", payload.task);
    match payload.task.as_str(){
        "30s" => {
            let result = thirty_second().await?;
            println!("Result: {}", result);
            let db = DatabaseService::new().await?;
            let tz  = chrono_tz::Asia::Bangkok;
            let current_time =  tz.from_utc_datetime(&chrono::Utc::now().naive_utc()).timestamp();

            let update_time = db.update_last_update(current_time).await?;
            println!("Update time into db! \n{}", update_time);
            return Ok(result);
        },
        "1h" => {
            thirty_second().await;

        },
        _ => {
            eprintln!("Unknown task: {}", payload.task);
           return  Err("Unknown task received".into())
        }
    }
    Ok("Success")
}

async fn thirty_second() -> Result<&'static str, Error>{

    println!("1 done");

    // Initialize the `App` with the database connection
    let realtime_task = Realtime_task::new().await.expect("Failed to initialize the app");
    // Run the tasks
    match realtime_task.run().await {
        Ok(msg) => {

            return Ok("Task 30s completed")

        },
        Err(e) => {
            eprintln!("Error running the app: {:?}", e);
        }
    }
    Ok("Task 30s is failed!")
}

async fn one_hour() -> Result<&'static str, Error>{

    println!("1 done");

    // Initialize the `App` with the database connection
    let realtime_task = Realtime_task::new().await.expect("Failed to initialize the app");
    // Run the tasks
    match realtime_task.run().await {
        Ok(msg) => {
            println!("Result: {}", msg);
            let db = DatabaseService::new().await?;
            let tz  = chrono_tz::Asia::Bangkok;
            let current_time =  tz.from_utc_datetime(&chrono::Utc::now().naive_utc()).timestamp();

            let update_time = db.update_last_update(current_time).await?;
            println!("Update time into db! \n{}", update_time);
            return Ok("Task 30s completed")

        },
        Err(e) => {
            eprintln!("Error running the app: {:?}", e);

        }
    }
    Ok("Task 1 hour failed")

}

async fn task1() -> Result<&'static str, Error>{

    // Call your function
   // get_user_courses().await?;


    println!("1 done");

    // Initialize the `App` with the database connection
    let realtime_task = Realtime_task::new().await.expect("Failed to initialize the app");
    // Run the tasks
    match realtime_task.run().await {
        Ok(msg) => println!("Result: {}", msg),
        Err(e) => eprintln!("Error running the app: {:?}", e),
    }
    // update time
    let db = DatabaseService::new().await?;
    let tz  = chrono_tz::Asia::Bangkok;
    let current_time =  tz.from_utc_datetime(&chrono::Utc::now().naive_utc()).timestamp();

    let update_time = db.update_last_update(current_time).await?;
    println!("Update time into db! \n{}", update_time);
    Ok("Task 1 completed")
}
// async fn task2() -> Result<&'static str, Error> {
//
//     println!("2 done");
//     let db_url ="postgresql://postgres:1307x2Npk@moodle.cd2wy4iagdv9.ap-southeast-1.rds.amazonaws.com:5432/moodle";
//     //let db = DatabaseService::new(db_url).await?;
//     //let pool = PgPool::connect(d_url).await?;
//     let db = Arc::new(DatabaseService::new().await?);
//     let user_courses = db.get_user_course().await?;
//     let mut tasks = vec![];
//
//     for user_course in user_courses {
//         println!("UC: {user_course:?}");
//         let db_clone = Arc::clone(&db);
//         let user_course_clone = user_course.clone();
//
//         // Spawn each task concurrently
//         let task = tokio::spawn(async move {
//             let quiz_result = db_clone.get_quiz(user_course_clone.clone()).await;
//             let course_result = db_clone.get_assign(user_course_clone.clone()).await;
//             let label_result = db_clone.get_label(user_course_clone.clone()).await;
//
//             match (quiz_result, course_result, label_result) {
//                 (Ok(quizzes), Ok(course), Ok(labels)) => {
//                     println!("Assign: {:?}", course);
//                     println!("Quizzes: {:?}", quizzes);
//                     println!("Labels: {:?}", labels);
//                 }
//                 (Err(e), _, _) => eprintln!("Error fetching quizzes: {:?}", e),
//                 (_, Err(e), _) => eprintln!("Error fetching assign: {:?}", e),
//                 (_, _, Err(e)) => eprintln!("Error fetching labels: {:?}", e),
//             }
//         });
//         tasks.push(task);
//     }
//
//         // Await all tasks to complete
//         for task in tasks {
//             task.await?;
//         }
//
//
//     Ok("Task 2 completed")
// }

// Simulate Task 3
async fn task3() -> Result<&'static str, Error> {
    sleep(Duration::from_secs(3)).await; // Simulate work with sleep
    println!("3 done");
    let db_url ="postgresql://postgres:1307x2Npk@moodle.cd2wy4iagdv9.ap-southeast-1.rds.amazonaws.com:5432/moodle";
    //let db = DatabaseService::new(db_url).await?;
    //let pool = PgPool::connect(d_url).await?;
    let db = DatabaseService::new().await?;
    let course = db.get_coursename(4).await?;
    println!("Course name: {course:?}");
    Ok("Task 3 completed")
}

// Simulate Task 4
async fn task4() -> Result<&'static str, Error> {
    sleep(Duration::from_secs(1)).await; // Simulate work with sleep
    println!("4 done");
    let db_url ="postgresql://postgres:1307x2Npk@moodle.cd2wy4iagdv9.ap-southeast-1.rds.amazonaws.com:5432/moodle";
    //let db = DatabaseService::new(db_url).await?;
    //let pool = PgPool::connect(d_url).await?;
    let db = DatabaseService::new().await?;
    let user = db.get_username(2).await?;
    println!("Course name: {user:?}");
    Ok("Task 4 completed")
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    use std::env;

    let key = "DB_URL";
    unsafe {
        env::set_var(key, "postgresql://postgres:1307x2Npk@moodle.cd2wy4iagdv9.ap-southeast-1.rds.amazonaws.com:5432/moodle");
    }
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await

}
