pub fn common_index() -> &'static str {
    "Hello, world!"
}

pub fn common_query(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name),
        None => "You did not provide a name!".to_string(),
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