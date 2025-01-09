use rocket::serde::json::Json;
use rocket::http::Status;

use crate::entities::passengers::{PassengerAuthRequest, PassengerAuthResponse};
use crate::error::{ErrorResponse, ErrorCodeName};
use crate::logics::passengers::generate_session_token;

pub fn handler_authenticate_passenger(payload: Json<PassengerAuthRequest>) -> Result<Json<PassengerAuthResponse>, Json<ErrorResponse>> {
    

    let parsed = payload.into_inner();
    let token = generate_session_token(parsed.email.to_string());
    if token.is_err() {
        return Err(Json(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            i_code: Status::BadRequest.code,
            message: "Failed to authenticate".to_string(),
            request_id: "".to_string(),
        }));
    }

    let mock = PassengerAuthResponse {
        token: token.unwrap(),
    };

    Ok(Json(mock))
}