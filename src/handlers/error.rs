use crate::error::{ErrorCodeName, ErrorResponse};
use rocket::serde::json::Json;
use validator::Validate;

pub fn validate_incoming_request<T: Validate>(payload: &T) -> Result<bool, Json<ErrorResponse>> {
    if let Err(validation_errors) = payload.validate() {
        let error_content = ErrorResponse {
            code: ErrorCodeName::InvalidRequest.as_str().to_string(),
            message: validation_errors.to_string(),
            request_id: "".to_string(), // TODO: get request id from request
            i_code: 400,
        };
        
        return Err(Json(error_content))
    }

    Ok(true)
}