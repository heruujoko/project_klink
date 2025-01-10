use crate::utils::serializers::{custom_date_format, custom_optional_date_format};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::passengers)]
#[derive(Debug, Serialize, Clone)]
pub struct Passenger {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub address: Option<String>,

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

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct PassengerTokenClaims {
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Validate)]
#[diesel(table_name = crate::schema::passengers)]
pub struct PassengerRegistrationRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub phone: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassengerSafeResponse {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
}
