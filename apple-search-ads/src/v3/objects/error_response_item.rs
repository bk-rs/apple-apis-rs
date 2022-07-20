// https://developer.apple.com/documentation/apple_search_ads/errorresponseitem

use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponseItem {
    pub field: String,
    pub message: String,
    #[serde(rename = "messageCode")]
    pub message_code: ErrorResponseItemMessageCode,
}

#[derive(Deserialize_enum_str, PartialEq, Eq, Debug, Clone)]
pub enum ErrorResponseItemMessageCode {
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,
    #[serde(rename = "INVALID_DATE_FORMAT")]
    InvalidDateFormat,
    #[serde(other)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_de_item() -> Result<(), Box<dyn error::Error>> {
        let item: ErrorResponseItem = serde_json::from_str(
            r#"{
                "messageCode": "<CODE>",
                "message": "<MESSAGE>",
                "field": "<FIELD>"
            }"#,
        )?;

        assert_eq!(item.field, "<FIELD>");
        assert_eq!(item.message, "<MESSAGE>");
        assert_eq!(
            item.message_code,
            ErrorResponseItemMessageCode::Other("<CODE>".to_owned())
        );

        Ok(())
    }

    #[test]
    fn test_de_code() -> Result<(), Box<dyn error::Error>> {
        #[derive(Deserialize)]
        pub struct Foo {
            pub code: ErrorResponseItemMessageCode,
        }

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"code": "UNAUTHORIZED"}"#,)?.code,
            ErrorResponseItemMessageCode::Unauthorized
        );

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"code": "INVALID_DATE_FORMAT"}"#,)?.code,
            ErrorResponseItemMessageCode::InvalidDateFormat
        );

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"code": "Bar"}"#,)?.code,
            ErrorResponseItemMessageCode::Other("Bar".to_owned())
        );

        Ok(())
    }
}
