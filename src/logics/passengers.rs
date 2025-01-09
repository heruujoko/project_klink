use crate::entities::passengers::PassengerTokenClaims;
use crate::error::{ErrorCodeName, ErrorResponse};
use crate::services::passenger_service::get_a_passenger;
use jsonwebtoken::{encode, Header, EncodingKey};

use chrono::{Utc, Duration};
use rocket::http::Status;

pub fn generate_session_token(email: String) -> Result<String, ErrorResponse> {
    let p = get_a_passenger();
    if p.is_err() {
        return Err(ErrorResponse{
            code: ErrorCodeName::InvalidRequest.to_string(),
            i_code: Status::BadRequest.code,
            message: "Failed to get passenger".to_string(),
            request_id: "".to_string(),
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

    let encode_resp = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()));
    if encode_resp.is_err() {
        return Err(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.to_string(),
            i_code: Status::BadRequest.code,
            message: "Failed to generate token".to_string(),
            request_id: "".to_string(),
        });
    }

    Ok(encode_resp.unwrap())
}