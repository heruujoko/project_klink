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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![routes::index, routes::query, routes::with_json, routes::with_json_201, routes::maybe])
        .launch()
        .await?;

    Ok(())
}