use crate::error::{ErrorCodeName, ErrorResponse};
use std::env;

// Define as a static array to make it accessible across the file
static REQUIRED_VARS: &[&str] = &[
    "KLINK_WELCOME_MESSAGE",
    "KLINK_DATABASE_URL",
    "KLINK_DATABASE_URL_RO",
];

pub fn setup_all_env() -> Result<bool, ErrorResponse> {
    for req_var in REQUIRED_VARS {
        let has_env = env::var(req_var).is_ok();
        if !has_env {
            let error_response = ErrorResponse {
                code: ErrorCodeName::MissingEnvVar.as_str().to_string(),
                message: format!("Missing environment variable: please ensure {} is available in your system variable", req_var),
            };
            return Err(error_response);
        }
    }

    Ok(true)
}

pub fn get_var(varname: &str) -> Result<String, ErrorResponse> {
    let fetched = env::var(varname);
    if !REQUIRED_VARS.contains(&varname) {
        let error_response = ErrorResponse {
            code: ErrorCodeName::UnregisteredVar.as_str().to_string(),
            message: format!("Access to unregistered environment variable: {}", varname),
        };
        return Err(error_response);
    }

    match fetched {
        Ok(value) => {
            let parsed = value.parse().map_err(|_| ErrorResponse {
                code: ErrorCodeName::UnregisteredVar.as_str().to_string(),
                message: format!("Invalid value for environment variable: {}", varname),
            })?;
            Ok(parsed)
        }
        Err(_) => {
            let error_response = ErrorResponse {
                code: ErrorCodeName::MissingEnvVar.as_str().to_string(),
                message: format!("Missing environment variable: {}", varname),
            };
            Err(error_response)
        }
    }
}
