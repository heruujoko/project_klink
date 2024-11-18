mod utils;
mod entities;
mod services;

// static LANGUAGE: &str = "Rust";
// const THRESHOLD: i32 = 10;

#[tokio::main]
async fn main() {
    utils::logger::info("Hello, world!".to_string());
    utils::logger::warn("message can be truncated".to_string());
    let err_m = "Error message".to_string();
    utils::logger::fatal(&err_m, None);
    // Now you can use MetaData struct
    let metadata = entities::metadata::MetaData {
        user_agent: String::from("Mozilla/5.0"),
        email: String::from("user@example.com"),
        exp: 1234567890,
    };
    utils::logger::debug(metadata);
    match services::failing_service::call(false) {
        Ok(message) => println!("Success: {}", message),
        Err(e) => println!("Failed: {}", e)
    }
    let (tk1_result, tk2_result) = services::failing_service::concurrency_call().await;
    println!("Task 1 returned: {}", tk1_result);
    println!("Task 2 returned: {}", tk2_result);
    println!("combined {}", tk2_result + tk1_result);
}