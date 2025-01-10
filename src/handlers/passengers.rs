use rocket::serde::json::Json;

use crate::entities::passengers::{
    Passenger, PassengerAuthRequest, PassengerAuthResponse, PassengerRegistrationRequest,
};
use crate::error::{ErrorCodeName, ErrorResponse};
use crate::logics::passengers::{generate_session_token, register_new_passenger};

pub fn handler_authenticate_passenger(
    payload: Json<PassengerAuthRequest>,
) -> Result<Json<PassengerAuthResponse>, Json<ErrorResponse>> {
    let parsed = payload.into_inner();
    let token = generate_session_token(parsed.email.to_string());
    if token.is_err() {
        return Err(Json(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            message: "Failed to authenticate".to_string(),
        }));
    }

    let mock = PassengerAuthResponse {
        token: token.unwrap(),
    };

    Ok(Json(mock))
}

pub fn handler_register_passenger(
    payload: Json<PassengerRegistrationRequest>,
) -> Result<Json<Passenger>, Json<ErrorResponse>> {
    let parsed = payload.into_inner();
    let resp = register_new_passenger(parsed);
    return match resp {
        Ok(p) => Ok(Json(p)),
        Err(e) => Err(Json(e)),
    };
}
