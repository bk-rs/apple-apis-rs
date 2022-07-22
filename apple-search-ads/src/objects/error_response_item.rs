// https://developer.apple.com/documentation/apple_search_ads/errorresponseitem

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorResponseItem {
    pub field: Box<str>,
    pub message: Box<str>,
    #[serde(rename = "messageCode")]
    pub message_code: ErrorResponseItemMessageCode,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum ErrorResponseItemMessageCode {
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,
    #[serde(rename = "INVALID_DATE_FORMAT")]
    InvalidDateFormat,
    #[serde(other)]
    Other(Box<str>),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let item: ErrorResponseItem = serde_json::from_str(
            r#"{
                "messageCode": "<CODE>",
                "message": "<MESSAGE>",
                "field": "<FIELD>"
            }"#,
        )?;

        assert_eq!(item.field.as_ref(), "<FIELD>");
        assert_eq!(item.message.as_ref(), "<MESSAGE>");
        assert_eq!(
            item.message_code,
            ErrorResponseItemMessageCode::Other("<CODE>".into())
        );

        Ok(())
    }

    #[test]
    fn test_de_message_code() -> Result<(), Box<dyn error::Error>> {
        #[derive(Deserialize)]
        pub struct Foo {
            pub message_code: ErrorResponseItemMessageCode,
        }

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"message_code": "UNAUTHORIZED"}"#,)?.message_code,
            ErrorResponseItemMessageCode::Unauthorized
        );

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"message_code": "INVALID_DATE_FORMAT"}"#,)?
                .message_code,
            ErrorResponseItemMessageCode::InvalidDateFormat
        );

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"message_code": "Bar"}"#,)?.message_code,
            ErrorResponseItemMessageCode::Other("Bar".into())
        );

        Ok(())
    }
}
