use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::external::database::get_connection;
use crate::entities::vehicle::Vehicle;

pub fn get_all_vehicles() -> Result<Vec<Vehicle>, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();
    use crate::schema::vehicles::dsl::*;

    // Proper error propagation using ? operator instead of expect()
    let results = vehicles.filter(deleted_at.is_null()).load(uconn)?;
    Ok(results)
}