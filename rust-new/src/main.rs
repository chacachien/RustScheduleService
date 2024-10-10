
pub mod model;
pub mod service;

use std::sync::Arc;
use std::time::Duration;
use serde_json::Value;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

use sqlx::{PgPool, Pool};
use crate::service::query_service::DatabaseService;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/



#[derive(Serialize, Deserialize, Debug)]
struct EventPayload{
    task: String
}
async fn function_handler(event: LambdaEvent<Value>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload:EventPayload = serde_json::from_value(event.payload)?;
    // println!("Received event: {:?}", event);
     println!("Payload: {:?}", payload.task);
    match payload.task.as_str(){
        "30s" => {
            thirty_second().await;

        },
        "1h" => {
            thirty_second().await;

        },
        _ => {
            eprintln!("Unknown task: {}", payload.task);
           return  Err("Unknown task received".into())
        }
    }
    Ok(())
}

async fn thirty_second(){
    let (task1_res, task2_res, task3_res, task4_res) = tokio::join!(
        task1(),
        task2(),
        task3(),
        task4()
    );
    println!("Task 1 result: {:?}", task1_res);
    println!("Task 2 result: {:?}", task2_res);
    println!("Task 3 result: {:?}", task3_res);
    println!("Task 4 result: {:?}", task4_res);

}

async fn task1() -> Result<&'static str, Error>{

    // Call your function
   // get_user_courses().await?;


    println!("1 done");
    Ok("Task 1 completed")
}
async fn task2() -> Result<&'static str, Error> {

    println!("2 done");
    let db_url ="postgresql://postgres:1307x2Npk@moodle.cd2wy4iagdv9.ap-southeast-1.rds.amazonaws.com:5432/moodle";
    //let db = DatabaseService::new(db_url).await?;
    //let pool = PgPool::connect(d_url).await?;
    let db = Arc::new(DatabaseService::new(db_url).await?);
    let user_courses = db.get_user_course().await?;
    let mut tasks = vec![];

    for user_course in user_courses {
        println!("UC: {user_course:?}");
        let db_clone = Arc::clone(&db);
        let user_course_clone = user_course.clone();

        // Spawn each task concurrently
        let task = tokio::spawn(async move {
            let quiz_result = db_clone.get_quiz(user_course_clone.clone()).await;
            let course_result = db_clone.get_assign(user_course_clone.clone()).await;
            let label_result = db_clone.get_label(user_course_clone.clone()).await;

            match (quiz_result, course_result, label_result) {
                (Ok(quizzes), Ok(course), Ok(labels)) => {
                    println!("Assign: {:?}", course);
                    println!("Quizzes: {:?}", quizzes);
                    println!("Labels: {:?}", labels);
                }
                (Err(e), _, _) => eprintln!("Error fetching quizzes: {:?}", e),
                (_, Err(e), _) => eprintln!("Error fetching assign: {:?}", e),
                (_, _, Err(e)) => eprintln!("Error fetching labels: {:?}", e),
            }
        });
        tasks.push(task);
    }

        // Await all tasks to complete
        for task in tasks {
            task.await?;
        }


    Ok("Task 2 completed")
}

// Simulate Task 3
async fn task3() -> Result<&'static str, Error> {
    sleep(Duration::from_secs(3)).await; // Simulate work with sleep
    println!("3 done");
    Ok("Task 3 completed")
}

// Simulate Task 4
async fn task4() -> Result<&'static str, Error> {
    sleep(Duration::from_secs(1)).await; // Simulate work with sleep
    println!("4 done");
    Ok("Task 4 completed")
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await

}
