mod utils;
mod entities;
use entities::metadata::MetaData;

// static LANGUAGE: &str = "Rust";
// const THRESHOLD: i32 = 10;

fn main() {
    utils::logger::info("Hello, world!".to_string());
    utils::logger::warn("message can be truncated".to_string());
    utils::logger::error("Oops, world!".to_string(), None);
    // Now you can use MetaData struct
    let metadata = MetaData {
        user_agent: String::from("Mozilla/5.0"),
        email: String::from("user@example.com"),
        exp: 1234567890,
    };
    utils::logger::debug(metadata);

}