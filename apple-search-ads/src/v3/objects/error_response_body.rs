// https://developer.apple.com/documentation/apple_search_ads/errorresponsebody

use serde::Deserialize;

use super::error_response_item::ErrorResponseItem;

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponseBody {
    pub errors: Vec<ErrorResponseItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeneralErrorResponse {
    pub error: ErrorResponseBody,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs};

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let body = serde_json::from_str::<GeneralErrorResponse>(
            r#"{
                "error": {
                    "errors": [
                        {
                        "messageCode": "<CODE>",
                        "message": "<MESSAGE>",
                        "field": "<FIELD>"
                        }
                    ]
                }
            }"#,
        )?
        .error;

        assert_eq!(body.errors.len(), 1);

        Ok(())
    }

    #[test]
    fn test_de_with_unauthorized() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            fs::read_to_string("tests/v3/response_body_json_files/error_with_unauthorized.json")
                .unwrap();

        let _: GeneralErrorResponse = serde_json::from_str(&json_content)?;

        Ok(())
    }
}
