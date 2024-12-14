use crate::entities::metadata::MetaData;
pub fn mock_metadata() -> MetaData {
    MetaData {
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36".to_string(),
        email: "alice@example.com".to_string(),
        exp: 1690000000,
    }
}