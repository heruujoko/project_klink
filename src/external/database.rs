use diesel::prelude::*;
use crate::config;
use std::sync::Mutex;
use once_cell::sync::OnceCell;

static DB_CONNECTION: OnceCell<Mutex<PgConnection>> = OnceCell::new();
static DB_CONNECTION_RO: OnceCell<Mutex<PgConnection>> = OnceCell::new();

pub fn initialize_db() -> Result<(), String> {
    let db_url = config::env::get_var("KLINK_DATABASE_URL").unwrap();
    let conn = PgConnection::establish(&db_url)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    DB_CONNECTION.set(Mutex::new(conn))
        .map_err(|_| "Database already initialized".to_string())
}

pub fn initialize_db_ro() -> Result<(), String> {
    let db_url = config::env::get_var("KLINK_DATABASE_URL_RO").unwrap();
    let conn = PgConnection::establish(&db_url)
        .map_err(|e| format!("Failed to connect to database RO: {}", e))?;
    
    DB_CONNECTION_RO.set(Mutex::new(conn))
        .map_err(|_| "Database already initialized".to_string())
}

pub fn get_connection() -> &'static Mutex<PgConnection> {
    DB_CONNECTION.get().expect("Database not initialized")
}

pub fn get_connection_ro() -> &'static Mutex<PgConnection> {
    DB_CONNECTION.get().expect("Database not initialized")
}
