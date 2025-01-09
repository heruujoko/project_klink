use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::external::database::get_connection;
use crate::entities::passengers::Passenger;
use crate::schema::passengers::dsl::*;

pub fn get_a_passenger() -> Result<Passenger, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();

    // Proper error propagation using ? operator instead of expect()
    let results: Vec<Passenger> = passengers.filter(deleted_at.is_null()).load(uconn)?;
    // Using get() method
    let passenger = results.get(0).ok_or(diesel::result::Error::NotFound)?;
    Ok(passenger.clone())
}