use rocket::http::Status;
use rocket::serde::json::Json;

use crate::handlers;
use crate::entities::metadata::MetaData;
use crate::entities::metadata::MetaDataError;
use crate::guards::user_agent::UserAgentGuard;
use crate::error::ErrorResponse;

fn handle_guarded_request<G,T>(
    guard: Result<G, Json<ErrorResponse>>,
    handler: fn() -> Json<T>
) -> (Status, Result<Json<T>, Json<ErrorResponse>>)
{
    match guard {
        Ok(_) => (Status::Ok, Ok(handler())),
        Err(e) => (Status::from_code(e.i_code).unwrap(), Err(e))
    }
}

#[get("/")]
pub fn index() -> &'static str {
   return handlers::common::common_index();
}

#[get("/query?<name>")]
pub fn query(name: Option<String>) -> String {
   return handlers::common::common_query(name);
}

#[get("/json", format = "application/json")]
pub fn with_json(_guard: Result<UserAgentGuard, Json<ErrorResponse>>) -> (Status, Result<Json<MetaData>, Json<ErrorResponse>>) {
    handle_guarded_request::<UserAgentGuard, MetaData>(_guard, handlers::common::common_with_json)
}
#[get("/201", format = "application/json")]
pub fn with_json_201() -> (Status, Json<MetaData>) {
   (Status::Created, handlers::common::common_with_json())
}

#[get("/maybe?<fail>", format = "application/json")]
pub fn maybe(fail: Option<String>) -> (Status, Result<Json<MetaData>, Json<MetaDataError>>) {
   let result = handlers::common::common_allow_fail(fail);
    match result {
        Ok(_) => (Status::Created, result),
        Err(_) => (Status::BadRequest, result),
    }
}
