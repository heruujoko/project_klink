use diesel::prelude::*;
use serde::Serialize;
use crate::utils::serializers::{custom_date_format, custom_optional_date_format};
#[derive(Queryable, Selectable)]
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
    pub deleted_at: Option<chrono::NaiveDateTime>,}