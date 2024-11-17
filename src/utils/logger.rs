pub fn info(msg: String) -> String {
    println!("[INFO] {}", msg);
    return format!("[INFO] {}", msg)
}

pub fn warn(msg: String) {
    println!("[WARN] {}", msg);
}

pub fn fatal(msg: &String, error: Option<String>) -> String {
    match error {
        Some(err) => {
            println!("[ERROR] {} - {}", msg, err);
            return format!("[ERROR] {} - {}", msg, err);
        },
        None => {
            println!("[ERROR] {}", msg);
            return format!("[ERROR] {}", msg);
        }
    }
}

pub fn debug<T: std::fmt::Debug>(obj: T) {
    println!("[DEBUG] {:?}", obj);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_logging() {
        let message = "Test info message".to_string();
        let result = info(message);
        assert_eq!(result, "[INFO] Test info message");
    }

    #[test]
    fn test_fatal_with_error() {
        let message = "Main error".to_string();
        let error = Some("Additional details".to_string());
        let result = fatal(&message, error);
        assert_eq!(result, "[ERROR] Main error - Additional details");
    }

    #[test]
    fn test_fatal_without_error() {
        let message = "Simple error".to_string();
        let result = fatal(&message, None);
        assert_eq!(result, "[ERROR] Simple error");
    }

    #[test]
    fn test_debug_logging() {
        let test_struct = vec![1, 2, 3];
        // Since debug() only prints and doesn't return, we just verify it doesn't panic
        debug(test_struct);
    }

    #[test]
    fn test_warn_logging() {
        let message = "Test warning".to_string();
        // Since warn() only prints and doesn't return, we just verify it doesn't panic
        warn(message);
    }
}



