use crate::entities::metadata::MetaData;
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
    let meta_content = MetaData {
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36".to_string(),
        email: "alice@example.com".to_string(),
        exp: 1690000000,
    };

    Json(meta_content)
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