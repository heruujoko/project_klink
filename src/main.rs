#[macro_use]
extern crate rocket;
mod config;
mod entities;
mod error;
mod external;
mod guards;
mod handlers;
mod logics;
mod middlewares;
mod routes;
mod schema;
mod services;
mod utils;

use rocket::serde::json::Json;
use tokio::try_join;

#[catch(422)]
fn unprocessable_entity() -> Json<error::ErrorResponse> {
    let default422 = error::ErrorResponse {
        code: error::ErrorCodeName::UnprocessableEntity
            .as_str()
            .to_string(),
        message: "Unprocessable Entity".to_string(),
    };
    Json(default422)
}

#[catch(404)]
fn notfound() -> Json<error::ErrorResponse> {
    let default422 = error::ErrorResponse {
        code: error::ErrorCodeName::NotFound.as_str().to_string(),
        message: "Not Found".to_string(),
    };
    Json(default422)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let all_required_env_set = config::env::setup_all_env();
    match all_required_env_set {
        Ok(_) => println!("All required environment variables are set"),
        Err(err) => {
            println!("Error: {}", err.message);
            return Ok(());
        }
    }

    let welcome = config::env::get_var("KLINK_WELCOME_MESSAGE");
    match welcome {
        Ok(welcome) => println!("{}", welcome),
        Err(err) => {
            println!("Error: {}", err.message);
            return Ok(());
        }
    }

    let connection_results = try_join!(
        external::database::initialize_db(),
        external::database::initialize_db_ro()
    );
    match connection_results {
        Ok(_) => println!("All dependency connections established"),
        Err(err) => {
            println!("Error establishing dependency connections: {}", err);
            return Ok(());
        }
    }

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                routes::index,
                routes::query,
                routes::with_json,
                routes::maybe,
                routes::with_data_validation,
                routes::verchile_route,
                routes::vehicle_raw_route,
                routes::vehicle_post_route
            ],
        )
        .mount(
            "/api/v1/passenger",
            routes![
                routes::api_v1_passengers_auth,
                routes::api_v1_passengers_registration
            ],
        )
        .register("/", catchers![unprocessable_entity, notfound])
        .launch()
        .await?;

    Ok(())
}
