#[macro_use] extern crate rocket;
mod routes;
mod handlers;
mod entities;
mod middlewares;
mod guards;
mod logics;

mod error;

use rocket::serde::json::Json;
use rocket::request::{Request};
use crate::guards::user_agent::UserAgentGuard;

#[catch(422)]
fn unprocessable_entity() -> Json<error::ErrorResponse> {
    let default422 = error::ErrorResponse {
        code: error::ErrorCodeName::UnprocessableEntity
            .as_str()
            .to_string(),
        message: "Unprocessable Entity".to_string(),
        request_id: String::new(),
        i_code: 422,
    };
    Json(default422)
}

#[catch(404)]
fn notfound() -> Json<error::ErrorResponse> {
    let default422 = error::ErrorResponse {
        code: error::ErrorCodeName::NotFound
            .as_str()
            .to_string(),
        message: "Not Found".to_string(),
        request_id: String::new(),
        i_code: 404,
    };
    Json(default422)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![routes::index, routes::query, routes::with_json, routes::with_json_201, routes::maybe, routes::with_data_validation])
        .register("/", catchers![unprocessable_entity, notfound])
        .launch()
        .await?;

    Ok(())
}