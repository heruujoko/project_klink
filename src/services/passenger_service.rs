use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::entities::passengers::{Passenger, PassengerRegistrationRequest};
use crate::external::database::get_connection;
use crate::schema::passengers::dsl::*;

pub fn get_a_passenger(passenger_email: String) -> Result<Passenger, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();

    // Proper error propagation using ? operator instead of expect()
    let results: Vec<Passenger> = passengers
        .filter(email.eq(passenger_email))
        .filter(deleted_at.is_null())
        .load(uconn)?;
    // Using get() method
    let passenger = results.get(0).ok_or(diesel::result::Error::NotFound)?;
    Ok(passenger.clone())
}

pub fn register_passenger(
    payload: PassengerRegistrationRequest,
) -> Result<Passenger, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();

    let passenger: Passenger = diesel::insert_into(passengers)
        .values(&payload)
        .get_result(uconn)?;
    Ok(passenger)
}
