use crate::handlers;
use crate::entities::metadata::MetaData;

use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> &'static str {
   return handlers::common::common_index();
}

#[get("/?<name>")]
pub fn query(name: Option<String>) -> String {
   return handlers::common::common_query(name);
}

#[get("/", format = "application/json")]
pub fn with_json() -> Json<MetaData> {
   return handlers::common::common_with_json();
}
