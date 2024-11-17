pub fn info(msg: String) -> String {
    println!("[INFO] {}", msg);
    return format!("[INFO] {}", msg)
}

pub fn warn(msg: String) {
    println!("[WARN] {}", msg);
}

pub fn error(msg: String, error: Option<String>) -> String {
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
mod logger {
    use super::*;

    #[test]
    fn test_info_logging() {
        let message = String::from("Test message");
        let result = Info(message);
        assert_eq!(result, "[INFO] Test message");
    }

    #[test]
    fn test_info_error_no_additional() {
        let message = String::from("Test message");
        let result = Error(message, None);
        assert_eq!(result, "[ERROR] Test message");
    }

    #[test]
    fn test_info_error_additional_info() {
        let message = String::from("Test message");
        let add = Some(String::from("Info"));
        let result = Error(message, add);
        assert_eq!(result, "[ERROR] Test message - Info");
    }
}



