// https://developer.apple.com/documentation/apple_search_ads/get_me_details

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    objects::me_detail_response::MeDetailResponse,
};

pub const URL: &str = "https://api.searchads.apple.com/api/v4/me";

//
#[derive(Debug, Clone)]
pub struct GetMeDetails {
    pub access_token: Box<str>,
}

impl GetMeDetails {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().into(),
        }
    }
}

impl Endpoint for GetMeDetails {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<MeDetailResponse>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(URL)
            .header(USER_AGENT, "apple-search-ads")
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .header(CONTENT_TYPE, MIME_APPLICATION_JSON)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => Ok(EndpointRet::Ok(
                serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
            )),
            status => match serde_json::from_slice(response.body()) {
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        let ep = GetMeDetails::new("TOKEN");
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), "GET");
        assert_eq!(req.uri(), "https://api.searchads.apple.com/api/v4/me");
        assert_eq!(req.headers().get("Authorization").unwrap(), "Bearer TOKEN");
    }
}
