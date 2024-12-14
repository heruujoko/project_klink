use crate::handlers;
use crate::entities::metadata::MetaData;
use crate::entities::metadata::MetaDataError;
use rocket::http::Status;
use crate::guards::user_agent::UserAgentGuard;
use crate::guards::user_agent::NetworkResponse;

use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> &'static str {
   return handlers::common::common_index();
}

#[get("/query?<name>")]
pub fn query(name: Option<String>) -> String {
   return handlers::common::common_query(name);
}

#[get("/json", format = "application/json")]
pub fn with_json(_guard: Result<UserAgentGuard, Json<NetworkResponse>>) -> (Status, Result<Json<MetaData>, Json<NetworkResponse>>) {
    match _guard {
        Ok(_) => (Status::Ok, Ok(handlers::common::common_with_json())),
        Err(e) => (Status::BadRequest, Err(e))
    }
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
