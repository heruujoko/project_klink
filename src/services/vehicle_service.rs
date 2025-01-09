use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::external::database::get_connection;
use crate::entities::vehicle::{NewVehicleRequest, Vehicle};
use crate::entities::raw_vehicle::RawVehicle;
use diesel::sql_query;
use crate::schema::vehicles::dsl::*;

pub fn get_all_vehicles() -> Result<Vec<Vehicle>, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();

    // Proper error propagation using ? operator instead of expect()
    let results = vehicles.filter(deleted_at.is_null()).load(uconn)?;
    Ok(results)
}

pub fn get_raw_vehicles() -> Result<Vec<RawVehicle>, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();
    
    let results: Vec<RawVehicle> = sql_query("SELECT id,name,description FROM vehicles")
            .load::<RawVehicle>(uconn)?;
    Ok(results)
}

pub fn add_vehicle(payload: NewVehicleRequest) -> Result<Vehicle, diesel::result::Error> {
    let conn = get_connection();
    let uconn = &mut *conn.lock().unwrap();

    let res = diesel::insert_into(vehicles)
        .values(&payload)
        .get_result(uconn);
    return res;
}