pub mod custom_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }
}

pub mod custom_optional_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => serializer.serialize_i64(date.timestamp()),
            None => serializer.serialize_none()
        }
    }
}
