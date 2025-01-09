use rocket::serde::json::Json;

use crate::entities::passengers::{PassengerAuthRequest, PassengerAuthResponse};
use crate::error::{ErrorResponse, ErrorCodeName};

pub fn handler_authenticate_passenger(payload: Json<PassengerAuthRequest>) -> Result<Json<PassengerAuthResponse>, Json<ErrorResponse>> {
    // let parsed = payload.into_inner();
    let mock = PassengerAuthResponse {
        token: "mock".to_string(),
    };

    Ok(Json(mock))
}
