use diesel::prelude::*;
use crate::config;

pub fn connect_db() -> PgConnection {
    let db_url = config::env::get_var("DATABASE_URL").unwrap();
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to DB"))
}