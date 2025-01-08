use rocket::serde::json::Json;
use crate::entities::raw_vehicle::RawVehicle;
use crate::entities::vehicle::{NewVehicleRequest, Vehicle};
use crate::error::{ErrorResponse, ErrorCodeName};
use crate::services::vehicle_service::{get_all_vehicles, get_raw_vehicles};

pub fn handler_all_vehicles() -> Result<Json<Vec<Vehicle>>, Json<ErrorResponse>> {
    let vehicles = get_all_vehicles();
    match vehicles {
        Ok(vehicles) => Ok(Json(vehicles)),
        Err(err) => Err(Json({
            ErrorResponse {
                code: ErrorCodeName::InvalidRequest.as_str().to_string(),
                message: err.to_string(),
                request_id: "RID".to_string(),
                i_code: 400,
            }
        })),
        
    }
}

pub fn handler_raw_vehicles() -> Result<Json<Vec<RawVehicle>>, Json<ErrorResponse>> {
    let vehicles = get_raw_vehicles();
    match vehicles {
        Ok(vehicles) => Ok(Json(vehicles)),
        Err(err) => Err(Json({
            ErrorResponse {
                code: ErrorCodeName::InvalidRequest.as_str().to_string(),
                message: err.to_string(),
                request_id: "RID".to_string(),
                i_code: 400,
            }
        })),
        
    }
}

pub fn handler_add_vehicle(payload: Json<NewVehicleRequest>) -> Result<String, Json<ErrorResponse>> {
    Ok("Yea".to_string())
}
