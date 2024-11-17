use std::error::Error;
pub fn call(is_ready: bool) -> Result<String, Box<dyn Error>> {
    if is_ready {
        let adventure_message = "You found a magical sword and defeated the dragon!".to_string();
        Ok(adventure_message)
    } else {
        let error_message = "Not prepared for adventure - forgot your shield!".to_string();
        crate::utils::logger::fatal(&error_message, None);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_success() {
        let result = call(true);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "You found a magical sword and defeated the dragon!"
        );
    }

    #[test]
    fn test_call_failure() {
        let result = call(false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Not prepared for adventure - forgot your shield!"
        );
    }
}