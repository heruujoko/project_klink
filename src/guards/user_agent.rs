use crate::error::ErrorCodeName;
use crate::error::ErrorResponse;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;

pub struct UserAgentGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgentGuard {
    type Error = Json<ErrorResponse>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Json<ErrorResponse>> {
        match req.headers().get_one("User-Agent") {
            Some(_user_agent) => Outcome::Success(UserAgentGuard),
            None => {
                let error_response = ErrorResponse {
                    code: ErrorCodeName::MissingUserAgent.to_string(),
                    message: "User-Agent header missing".to_string(),
                };

                Outcome::Error((Status::Unauthorized, Json(error_response)))
            }
        }
    }
}
