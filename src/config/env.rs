use std::env;
use crate::error::{ErrorCodeName, ErrorResponse};

pub fn setup_all_env() -> Result<bool, ErrorResponse> {
    // Verify required environment variables
    let required_vars = vec![
        "DATABASE_URL",
        "API_KEY",
        "SERVER_PORT"
    ];

    for req_var in required_vars {
        let has_env = env::var(req_var).is_ok();
        if !has_env {
            let error_response = ErrorResponse {
                code: ErrorCodeName::MissingEnvVar.as_str().to_string(),
                message: format!("Missing environment variable: {}", req_var),
                request_id: String::new(),
                i_code: 500,
            };
            return Err(error_response);
        }
    }

    Ok(true)
}
