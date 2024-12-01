#[macro_use] extern crate rocket;
mod routes;
mod handlers;
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![routes::index])
        .mount("/query", routes![routes::query])
        .launch()
        .await?;

    Ok(())
}