use crate::entities::metadata::MetaData;
use crate::entities::metadata::MetaDataError;
use crate::entities::metadata::MetaDataRequest;
use crate::logics::mocks::mock_metadata;
use crate::handlers::error::validate_incoming_request;
use crate::error::ErrorResponse;

use rocket::serde::json::Json;

pub fn common_index() -> &'static str {
    "Hello, world!"
}

pub fn common_query(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name),
        None => "You did not provide a name!".to_string(),
    }
}

pub fn common_with_json() -> Json<MetaData> {
    let meta_content = mock_metadata();
    Json(meta_content)
}

pub fn common_allow_fail(fail: Option<String>) -> Result<Json<MetaData>, Json<MetaDataError>> {
    match fail {
        Some(fail) => {
            let error_content = MetaDataError {
                code: "400".to_string(),
                message: "Bad request".to_string(),
            };
            Err(Json(error_content))
        }
        None => {
            let meta_content = mock_metadata();

            Ok(Json(meta_content))
        }
    }

}

pub fn common_with_data_validation(payload: Json<MetaDataRequest>) -> Result<Json<MetaData>, Json<ErrorResponse>> {
    let deserialized = payload.into_inner();
    let valid = validate_incoming_request(&deserialized);
    match valid {
        Err(error) => {
            Err(error)
        }
        Ok(_) => {
            let meta_content = mock_metadata();
            Ok(Json(meta_content))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_index() {
        let result = common_index();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_common_query_with_name() {
        let name = Some("Alice".to_string());
        let result = common_query(name);
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_common_query_without_name() {
        let result = common_query(None);
        assert_eq!(result, "You did not provide a name!");
    }
}