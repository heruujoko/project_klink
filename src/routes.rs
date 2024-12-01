use crate::handlers;

#[get("/")]
pub fn index() -> &'static str {
   return handlers::common::common_index();
}

#[get("/?<name>")]
pub fn query(name: Option<String>) -> String {
   return handlers::common::common_query(name);
}
