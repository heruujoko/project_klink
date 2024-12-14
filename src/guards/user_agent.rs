use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::serde::json::Json;
use crate::error::ErrorResponse;
use crate::error::ErrorCodeName;

pub struct UserAgentGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgentGuard {
    type Error = Json<ErrorResponse>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Json<ErrorResponse>> {
        match req.headers().get_one("User-Agent") {
            Some(_user_agent) => Outcome::Success(UserAgentGuard),
            None => {
                let error_response = ErrorResponse {
                    code:  ErrorCodeName::MissingUserAgent.to_string(),
                    message: "User-Agent header missing".to_string(),
                    request_id: "".to_string(),
                    i_code: Status::Unauthorized.code as u16,
                };

                Outcome::Error((Status::Unauthorized, Json(error_response)))
            }
        }
    }
}
