use crate::entities::raw_vehicle::RawVehicle;
use crate::entities::vehicle::{NewVehicleRequest, Vehicle};
use crate::error::{ErrorCodeName, ErrorResponse};
use crate::services::vehicle_service::{add_vehicle, get_all_vehicles, get_raw_vehicles};
use rocket::serde::json::Json;

pub fn handler_all_vehicles() -> Result<Json<Vec<Vehicle>>, Json<ErrorResponse>> {
    let vehicles = get_all_vehicles();
    match vehicles {
        Ok(vehicles) => Ok(Json(vehicles)),
        Err(err) => Err(Json({
            ErrorResponse {
                code: ErrorCodeName::InvalidRequest.as_str().to_string(),
                message: err.to_string(),
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
            }
        })),
    }
}

pub fn handler_add_vehicle(
    payload: Json<NewVehicleRequest>,
) -> Result<Json<Vehicle>, Json<ErrorResponse>> {
    let resp = add_vehicle(payload.into_inner());
    match resp {
        Ok(vehicle) => Ok(Json(vehicle)),
        Err(err) => Err(Json(ErrorResponse {
            code: ErrorCodeName::InvalidRequest.as_str().to_string(),
            message: err.to_string(),
        })),
    }
}
