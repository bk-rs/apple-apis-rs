// https://developer.apple.com/documentation/apple_search_ads/get_user_acl

use http::{
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
    Error as HttpError, Method, StatusCode, Version,
};
use serde_json::Error as SerdeJsonError;

use crate::v3::objects::{
    error_response_body::GeneralErrorResponse, user_acl_list_response::UserAclListResponse,
};

use super::endpoint_prelude::*;

const URL: &str = "https://api.searchads.apple.com/api/v3/acls";

#[derive(Debug, Default)]
pub struct GetUserAcl {}
impl GetUserAcl {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Endpoint for GetUserAcl {
    type RenderRequestError = GetUserAclError;

    type ParseResponseOutput = (
        Option<Result<UserAclListResponse, GeneralErrorResponse>>,
        StatusCode,
    );
    type ParseResponseError = GetUserAclError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(URL)
            .version(Version::HTTP_11)
            .header(USER_AGENT, "curl/7.72.0")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(GetUserAclError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = match response.status() {
            StatusCode::OK => Some(Ok(serde_json::from_slice::<UserAclListResponse>(
                response.body(),
            )
            .map_err(GetUserAclError::DeResponseBodyOkJsonFailed)?)),
            StatusCode::GONE => None,
            _ => Some(Err(serde_json::from_slice::<GeneralErrorResponse>(
                response.body(),
            )
            .map_err(GetUserAclError::DeResponseBodyErrJsonFailed)?)),
        };

        Ok((body, response.status()))
    }
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum GetUserAclError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    #[error("DeResponseBodyErrJsonFailed {0}")]
    DeResponseBodyErrJsonFailed(SerdeJsonError),
}
