#[macro_use] extern crate rocket;
mod routes;
mod handlers;
mod entities;
mod middlewares;

mod logics;
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(middlewares::ua_check::UserAgentCheck)
        .mount("/", routes![routes::index, routes::query, routes::with_json, routes::with_json_201, routes::maybe])
        .launch()
        .await?;

    Ok(())
}