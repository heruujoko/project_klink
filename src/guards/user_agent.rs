use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::serde::json::Json;

pub struct UserAgentGuard;
#[derive(Debug)]
pub enum UAError {
    Missing,
    Invalid,
}

#[derive(serde::Serialize, Debug)]
pub enum NetworkResponse {
    Created(String),
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgentGuard {
    type Error = Json<NetworkResponse>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Json<NetworkResponse>> {
        match req.headers().get_one("User-Agent") {
            Some(_user_agent) => Outcome::Success(UserAgentGuard),
            None => Outcome::Error((Status::Unauthorized, Json(NetworkResponse::Unauthorized("User-Agent header missing".to_string())))),
        }
    }
}
