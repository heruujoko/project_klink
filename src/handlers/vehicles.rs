use rocket::serde::json::Json;
use crate::entities::vehicle::Vehicle;
use crate::error::{ErrorResponse, ErrorCodeName};
use crate::services::vehicle_service::get_all_vehicles;

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