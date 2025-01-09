use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::serializers::{custom_date_format, custom_optional_date_format};
use validator::Validate;
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::vehicles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize)]
pub struct Vehicle {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "custom_date_format")]
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub created_at: chrono::NaiveDateTime,

    #[serde(with = "custom_date_format")]
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub updated_at: chrono::NaiveDateTime,

    #[serde(with = "custom_optional_date_format")]
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamptz>)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::vehicles)]
pub struct NewVehicleRequest {
    #[validate(length(min = 1, max = 2))]
    pub name: String,
    pub description: Option<String>,
}