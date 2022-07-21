// https://developer.apple.com/documentation/apple_search_ads/get_campaign-level_reports

use core::marker::PhantomData;

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    endpoints::{
        common::{EndpointError, EndpointRet},
        HEADER_KEY_X_AP_CONTEXT,
    },
    objects::{
        reporting_request::ReportingRequest, reporting_response_body::ReportingResponseBody,
        row::CampaignLevelRowMetaData,
    },
};

pub const URL: &str = "https://api.searchads.apple.com/api/v4/reports/campaigns";

//
#[derive(Debug)]
pub struct CustomizableGetCampaignLevelReports<M, I> {
    pub org_id: u64,
    pub reporting_request: ReportingRequest,
    //
    pub access_token: Box<str>,
    //
    phantom_m: PhantomData<M>,
    phantom_i: PhantomData<I>,
}
impl<M, I> CustomizableGetCampaignLevelReports<M, I> {
    pub fn new(
        org_id: u64,
        reporting_request: ReportingRequest,
        access_token: impl AsRef<str>,
    ) -> Self {
        Self {
            org_id,
            reporting_request,
            access_token: access_token.as_ref().into(),
            phantom_m: PhantomData,
            phantom_i: PhantomData,
        }
    }
}

impl<M, I> Endpoint for CustomizableGetCampaignLevelReports<M, I>
where
    M: DeserializeOwned,
    I: DeserializeOwned,
{
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<ReportingResponseBody<M, I>>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;

        let body = serde_json::to_vec(&self.reporting_request)
            .map_err(EndpointError::SerRequestBodyJsonFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url.as_str())
            .header(USER_AGENT, "apple-search-ads")
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .header(CONTENT_TYPE, MIME_APPLICATION_JSON)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(HEADER_KEY_X_AP_CONTEXT, format!("orgId={}", self.org_id))
            .body(body)
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

//
//
//
pub type GetCampaignLevelReports =
    CustomizableGetCampaignLevelReports<CampaignLevelRowMetaData, ()>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        let ep = GetCampaignLevelReports::new(1, Default::default(), "TOKEN");
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), "POST");
        assert_eq!(
            req.uri(),
            "https://api.searchads.apple.com/api/v4/reports/campaigns"
        );
        assert_eq!(req.headers().get("Authorization").unwrap(), "Bearer TOKEN");
        assert_eq!(req.headers().get("X-AP-Context").unwrap(), "orgId=1");
        assert!(!req.body().is_empty());
    }
}
