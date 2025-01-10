use crate::entities::passengers::{
    PassengerRegistrationRequest, PassengerSafeResponse, PassengerTokenClaims,
};
use crate::error::{ErrorCodeName, ErrorResponse};
use crate::services::passenger_service::{get_a_passenger, register_passenger};
use crate::utils::crypto::{hash_compare, hash_password};
use jsonwebtoken::{encode, EncodingKey, Header};

use chrono::{Duration, Utc};

pub fn generate_session_token(email: String, password: String) -> Result<String, ErrorResponse> {
    let p = get_a_passenger(email);
    if p.is_err() {
        return Err(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            message: "Failed to get passenger".to_string(),
        });
    }
    let found_p = p.unwrap();
    let password_match = hash_compare(password.as_str(), found_p.password.as_str());
    if !password_match {
        return Err(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            message: "Invalid password".to_string(),
        });
    }

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
) -> Result<PassengerSafeResponse, ErrorResponse> {
    let hashed = hash_password(payload.password);
    let hashed_payload = PassengerRegistrationRequest {
        email: payload.email,
        password: hashed.to_string(),
        name: payload.name,
        phone: payload.phone,
        address: payload.address,
    };
    let p = register_passenger(hashed_payload);
    if p.is_err() {
        return Err(ErrorResponse {
            code: ErrorCodeName::RegistrationFailed.to_string(),
            message: "Failed to register passenger".to_string(),
        });
    }
    let p = p.unwrap();
    let resp = PassengerSafeResponse {
        name: p.name,
        email: p.email,
        phone: p.phone,
        address: p.address,
    };
    Ok(resp)
}
