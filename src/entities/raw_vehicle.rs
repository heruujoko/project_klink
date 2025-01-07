use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, QueryableByName)]
#[diesel(table_name = crate::schema::vehicles)]
#[derive(Serialize)]
pub struct RawVehicle {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}