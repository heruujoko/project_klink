use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum AdventureError {
    NotPrepared(String),
    NoEquipment(String),
}

impl fmt::Display for AdventureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdventureError::NotPrepared(msg) => write!(f, "Not prepared: {}", msg),
            AdventureError::NoEquipment(msg) => write!(f, "Missing equipment: {}", msg),
        }
    }
}

pub fn call(is_ready: bool) -> Result<String, AdventureError> {
    if is_ready {
        let adventure_message = "You found a magical sword and defeated the dragon!".to_string();
        Ok(adventure_message)
    } else {
        let error_message = "Not prepared for adventure - forgot your shield!".to_string();
        crate::utils::logger::fatal(&error_message, None);
        Err(AdventureError::NoEquipment(error_message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUCCESS_MESSAGE: &str = "You found a magical sword and defeated the dragon!";
    const FAILURE_MESSAGE: &str = "Not prepared for adventure - forgot your shield!";

    #[test]
    fn test_successful_adventure() {
        // When
        let result = call(true);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SUCCESS_MESSAGE);
    }

    #[test]
    fn test_failed_adventure() {
        // When
        let result = call(false);

        // Then
        assert!(result.is_err());
        match result.unwrap_err() {
            AdventureError::NoEquipment(msg) => assert_eq!(msg, FAILURE_MESSAGE),
            _ => panic!("Expected NoEquipment error variant")
        }
    }

    #[test]
    fn test_error_display_format() {
        // Given
        let error = AdventureError::NoEquipment(FAILURE_MESSAGE.to_string());

        // Then
        assert_eq!(
            error.to_string(),
            format!("Missing equipment: {}", FAILURE_MESSAGE)
        );
    }
}