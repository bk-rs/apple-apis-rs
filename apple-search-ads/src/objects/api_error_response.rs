// https://developer.apple.com/documentation/apple_search_ads/apierrorresponse

use serde::{Deserialize, Serialize};

use super::error_response_body::ErrorResponseBody;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiErrorResponse {
    pub error: ErrorResponseBody,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let body = serde_json::from_str::<ApiErrorResponse>(
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
    fn test_v3_de_with_unauthorized() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            include_str!("../../tests/v3/response_body_json_files/error_with_unauthorized.json");

        let _: ApiErrorResponse = serde_json::from_str(json_content)?;

        Ok(())
    }
}
