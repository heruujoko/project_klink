use crate::entities::passengers::{Passenger, PassengerRegistrationRequest, PassengerTokenClaims};
use crate::error::{ErrorCodeName, ErrorResponse};
use crate::services::passenger_service::{get_a_passenger, register_passenger};
use jsonwebtoken::{encode, EncodingKey, Header};

use chrono::{Duration, Utc};

pub fn generate_session_token(email: String) -> Result<String, ErrorResponse> {
    let p = get_a_passenger(email);
    if p.is_err() {
        return Err(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            message: "Failed to get passenger".to_string(),
        });
    }

    let found_p = p.unwrap();

    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(8))
        .expect("Failed to calculate expiration time")
        .timestamp() as usize;

    let claims = PassengerTokenClaims {
        email: found_p.email.to_string(),
        exp: expiration_time,
    };

    let encode_resp = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    );
    if encode_resp.is_err() {
        return Err(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            message: "Failed to generate token".to_string(),
        });
    }

    Ok(encode_resp.unwrap())
}

pub fn register_new_passenger(
    payload: PassengerRegistrationRequest,
) -> Result<Passenger, ErrorResponse> {
    let p = register_passenger(payload);
    if p.is_err() {
        return Err(ErrorResponse {
            code: ErrorCodeName::RegistrationFailed.to_string(),
            message: "Failed to register passenger".to_string(),
        });
    }
    Ok(p.unwrap())
}
