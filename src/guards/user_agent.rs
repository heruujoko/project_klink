use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use serde::Serialize;

pub struct UserAgentGuard;
#[derive(Debug)]
pub enum UAError {
    Missing,
    Invalid,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: String,
    message: String,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgentGuard {
    type Error = Json<ErrorResponse>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("User-Agent") {
            Some(_user_agent) => Outcome::Success(UserAgentGuard),
            None => {
                let response = ErrorResponse {
                    code: "MISSING".to_string(),
                    message: "User-Agent header is missing".to_string(),
                };

                Outcome::Error((Status::Forbidden, Json(response)))
            },
        }
    }
}
