#[macro_use] extern crate rocket;
mod routes;
mod handlers;
mod entities;
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![routes::index])
        .mount("/query", routes![routes::query])
        .mount("/json", routes![routes::with_json])
        .launch()
        .await?;

    Ok(())
}