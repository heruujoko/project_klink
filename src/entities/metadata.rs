use serde::{Deserialize, Serialize};
use validator::Validate;

// hold content between functions and requests
#[derive(Debug, Serialize)]
pub struct MetaData {
    pub user_agent: String,
    pub email: String,
    pub exp: i64,
}

#[derive(Debug, Serialize)]
pub struct MetaDataError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MetaDataRequest {
    #[validate(range(min = 1, max=6))]
    pub amount: i32,
    #[validate(length(min = 1, max = 2))]
    pub description: Option<String>,
}