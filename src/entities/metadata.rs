use serde::Serialize;

// hold content between functions and requests
#[derive(Debug, Serialize)]
pub struct MetaData {
    pub user_agent: String,
    pub email: String,
    pub exp: i64,
}