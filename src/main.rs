#[macro_use] extern crate rocket;
mod routes;
mod handlers;
mod entities;
mod middlewares;
mod guards;
mod logics;
mod config;
mod error;
mod external;

use rocket::serde::json::Json;

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
        Ok(welcome) => println!("Server is running on port: {}", welcome),
        Err(err) => {
            println!("Error: {}", err.message);
            return Ok(());
        }
    }

    let connection_result = external::database::initialize_db();
    match connection_result {
        Ok(_) => println!("Database connection established"),
        Err(err) => {
            println!("Error: {}", err);
            return Ok(());
        }
    }
    let connection_result_ro = external::database::initialize_db_ro();
    match connection_result_ro {
        Ok(_) => println!("Database connection established"),
        Err(err) => {
            println!("Error: {}", err);
            return Ok(());
        }
    }

    let _rocket = rocket::build()
        .mount("/", routes![routes::index, routes::query, routes::with_json, routes::with_json_201, routes::maybe, routes::with_data_validation])
        .register("/", catchers![unprocessable_entity, notfound])
        .launch()
        .await?;

    Ok(())
}