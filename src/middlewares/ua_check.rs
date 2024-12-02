use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::Request;
use rocket::Response;

pub struct UserAgentCheck;

#[rocket::async_trait]
impl Fairing for UserAgentCheck {
    fn info(&self) -> Info {
        Info {
            name: "User Agent Check",
            kind: Kind::Request
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Response<'_>) {
        if request.uri().path() == "/201" {
            if request.headers().get_one("User-Agent").is_none() {
                response.set_status(Status::Forbidden);
                return;
            }
        }
    }
}
