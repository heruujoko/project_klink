use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::serializers::{custom_date_format, custom_optional_date_format};
use validator::Validate;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::passengers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize)]
pub struct Passenger {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
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
pub struct PassengerAuthRequest {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct PassengerAuthResponse {
    pub token: String,
}
