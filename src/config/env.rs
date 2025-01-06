use std::env;
use crate::error::{ErrorCodeName, ErrorResponse};

// Define as a static array to make it accessible across the file
static REQUIRED_VARS: &[&str] = &[
    "KLINK_WELCOME_MESSAGE",
];

pub fn setup_all_env() -> Result<bool, ErrorResponse> {
    for req_var in REQUIRED_VARS {
        let has_env = env::var(req_var).is_ok();
        if !has_env {
            let error_response = ErrorResponse {
                code: ErrorCodeName::MissingEnvVar.as_str().to_string(),
                message: format!("Missing environment variable: please ensure {} is available in your system variable", req_var),
                request_id: String::new(),
                i_code: 500,
            };
            return Err(error_response);
        }
    }

    Ok(true)
}

pub fn get_var(varname: &str) -> Result<String, ErrorResponse> {
    let fetched = env::var(varname);
    // check if varname not in REQUED_VARS
    if !REQUIRED_VARS.contains(&varname) {
        let error_response = ErrorResponse {
            code: ErrorCodeName::UnregisteredVar.as_str().to_string(),
            message: format!("Access to unregistered environment variable: {}", varname),
            request_id: String::new(),
            i_code: 500,
        };
        return Err(error_response);
    }

    match fetched {
        Ok(value) => {
            let parsed = value.parse().map_err(|_| {
                ErrorResponse {
                code: ErrorCodeName::UnregisteredVar.as_str().to_string(),
                    message: format!("Invalid value for environment variable: {}", varname),
                    request_id: String::new(),
                    i_code: 500,
                }
            })?;
            Ok(parsed)
        }
        Err(_) => {
            let error_response = ErrorResponse {
                code: ErrorCodeName::MissingEnvVar.as_str().to_string(),
                message: format!("Missing environment variable: {}", varname),
                request_id: String::new(),
                i_code: 500,
            };
            Err(error_response)
        }
    }
}