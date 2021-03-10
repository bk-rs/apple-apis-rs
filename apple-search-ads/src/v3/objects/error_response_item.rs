// https://developer.apple.com/documentation/apple_search_ads/errorresponseitem

use std::str::FromStr as _;

use serde::{Deserialize, Deserializer};
use strum::EnumString;

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponseItem {
    pub field: String,
    pub message: String,
    #[serde(rename = "messageCode")]
    pub message_code: ErrorResponseItemMessageCode,
}

#[derive(EnumString, PartialEq, Debug, Clone)]
pub enum ErrorResponseItemMessageCode {
    #[strum(serialize = "UNAUTHORIZED")]
    Unauthorized,
    #[strum(serialize = "INVALID_DATE_FORMAT")]
    InvalidDateFormat,
    #[strum(disabled)]
    Other(String),
}
impl<'de> Deserialize<'de> for ErrorResponseItemMessageCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        Ok(Self::from_str(str.as_ref()).unwrap_or_else(|_| Self::Other(str)))
    }
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
        };

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
