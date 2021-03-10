// https://developer.apple.com/documentation/apple_search_ads/get_user_acl

use std::io;

use http::{
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
    Method, StatusCode, Version,
};

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
    type ParseResponseOutput = (
        Option<Result<UserAclListResponse, GeneralErrorResponse>>,
        StatusCode,
    );
    type RetryReason = ();

    fn render_request(&self) -> io::Result<Request<Body>> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(URL)
            .version(Version::HTTP_11)
            .header(USER_AGENT, "curl/7.72.0")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(request)
    }

    fn parse_response(
        &mut self,
        response: Response<Body>,
    ) -> io::Result<EndpointParseResponseOutput<Self::ParseResponseOutput, Self::RetryReason>> {
        let body = match response.status() {
            StatusCode::OK => Some(Ok(serde_json::from_slice::<UserAclListResponse>(
                response.body(),
            )?)),
            StatusCode::GONE => None,
            _ => Some(Err(serde_json::from_slice::<GeneralErrorResponse>(
                response.body(),
            )?)),
        };

        Ok(EndpointParseResponseOutput::Done((body, response.status())))
    }
}
