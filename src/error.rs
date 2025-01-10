use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

pub enum ErrorCodeName {
    MissingUserAgent,
    InvalidRequest,
    UnprocessableEntity,
    NotFound,
    MissingEnvVar,
    UnregisteredVar,
    RegistrationFailed,
}

impl fmt::Display for ErrorCodeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ErrorCodeName {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCodeName::MissingUserAgent => "MISSING_USER_AGENT",
            ErrorCodeName::InvalidRequest => "INVALID_REQUEST",
            ErrorCodeName::UnprocessableEntity => "UNPROCESSABLE_ENTITY",
            ErrorCodeName::NotFound => "NOT_FOUND",
            ErrorCodeName::MissingEnvVar => "MISSING_ENV_VAR",
            ErrorCodeName::UnregisteredVar => "UNREGISTERED_VAR",
            ErrorCodeName::RegistrationFailed => "REGISTRATION_FAILED",
        }
    }
}
