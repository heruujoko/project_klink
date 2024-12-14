use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use uuid::Uuid;

pub struct RequestId;

#[rocket::async_trait]
impl Fairing for RequestId {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let request_id = Uuid::new_v4().to_string();
        request.local_cache(|| request_id.clone());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let request_id = request.local_cache(|| Uuid::new_v4().to_string());
        response.set_header(rocket::http::Header::new("X-Request-ID", request_id));
    }
}
