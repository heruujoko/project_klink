use std::error::Error;
use std::fmt;
use std::time::Duration;
use tokio::time::sleep;

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

async fn task1() -> i32 {
    println!("Task 1");
    // sleep 3 seconds
    sleep(Duration::from_secs(3)).await;
    println!("Task 1 done");
    1000
}

async fn task2() -> i32 {
    println!("Task 2");
    // sleep 3 seconds
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 done");
    2000
}

pub async fn concurrency_call() -> (i32, i32) {
    let task1_handle = tokio::spawn(task1());
    let task2_handle = tokio::spawn(task2());

    // First unwrap: Task execution status
    let (task1_result, task2_result) = match tokio::try_join!(task1_handle, task2_handle) {
        Ok((t1, t2)) => (t1, t2),
        Err(e) => (0,0)
    };

    (task1_result,task2_result)
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