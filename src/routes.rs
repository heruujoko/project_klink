
use std::any::Any;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::handlers;
use crate::entities::metadata::MetaData;
use crate::entities::metadata::MetaDataError;
use crate::entities::metadata::MetaDataRequest;
use crate::entities::vehicle::Vehicle;
use crate::guards::user_agent::UserAgentGuard;
use crate::error::ErrorResponse;

fn handle_guarded_request<G,T>(
    guard: Result<G, Json<ErrorResponse>>,
    handler: fn() -> Result<Json<T>, Json<ErrorResponse>>,
) -> (Status, Result<Json<T>, Json<ErrorResponse>>)
{
    match guard {
        // Ok(_) => (Status::Ok, Ok(handler())),
        Ok(_) => {
            // do handler
            let res = handler();
            match res {
                Ok(ok_resp) => (Status::Ok, Ok(ok_resp)),
                Err(e) => (Status::from_code(e.i_code).unwrap(), Err(e)),
            }
        }
        Err(e) => (Status::from_code(e.i_code).unwrap(), Err(e))
    }
}

fn handle_layered_guarded_request<T>(
    guards: Vec<Result<Box<dyn Any>, Json<ErrorResponse>>>,
    handler: fn() -> Result<Json<T>, Json<ErrorResponse>>,
) -> (Status, Result<Json<T>, Json<ErrorResponse>>)
{
    // Check if any guard failed
    if let Some(failed_guard) = guards.into_iter().find(|g| g.is_err()) {
        if let Err(e) = failed_guard {
            return (Status::from_code(e.i_code).unwrap(), Err(e));
        }
    }

    // All guards passed, execute handler
    let res = handler();
    match res {
        Ok(ok_resp) => (Status::Ok, Ok(ok_resp)),
        Err(e) => (Status::from_code(e.i_code).unwrap(), Err(e)),
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

#[get("/maybe?<fail>", format = "application/json")]
pub fn maybe(fail: Option<String>) -> (Status, Result<Json<MetaData>, Json<MetaDataError>>) {
   let result = handlers::common::common_allow_fail(fail);
    match result {
        Ok(_) => (Status::Created, result),
        Err(_) => (Status::BadRequest, result),
    }
}

#[post("/data", format = "application/json", data = "<payload>")]
pub fn with_data_validation(payload: Json<MetaDataRequest>) -> (Status, Result<Json<MetaData>, Json<ErrorResponse>>) {
    let result = handlers::common::common_with_data_validation(payload);
    match result {
        Ok(_) => (Status::Created, result),
        Err(_) => (Status::BadRequest, result),
    }
}

#[get("/vehicles", format = "application/json")]
pub fn verchile_route(user_agent_guard: Result<UserAgentGuard, Json<ErrorResponse>>) -> (Status, Result<Json<Vec<Vehicle>>, Json<ErrorResponse>>) {
    let guards = vec![
        user_agent_guard.map(|g| Box::new(g) as Box<dyn Any>),
    ];

    handle_layered_guarded_request(guards, handlers::vehicles::handler_all_vehicles)
}
