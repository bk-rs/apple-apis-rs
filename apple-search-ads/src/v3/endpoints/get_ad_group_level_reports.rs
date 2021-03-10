// https://developer.apple.com/documentation/apple_search_ads/get_ad_group_level_reports

use std::{io, marker::PhantomData};

use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Method, StatusCode, Version,
};
use serde::de::DeserializeOwned;

use crate::v3::objects::{
    error_response_body::GeneralErrorResponse, reporting_request::ReportingRequest,
    reporting_response_body::ReportingResponseBody, row::AdGroupLevelRowMetaData,
};

use super::endpoint_prelude::*;

#[derive(Debug)]
pub struct CustomizableGetAdGroupLevelReports<M, I> {
    org_id: u64,
    campaign_id: u64,
    reporting_request: ReportingRequest,
    phantom_m: PhantomData<M>,
    phantom_i: PhantomData<I>,
}
impl<M, I> CustomizableGetAdGroupLevelReports<M, I> {
    pub fn new(org_id: u64, campaign_id: u64, reporting_request: ReportingRequest) -> Self {
        Self {
            org_id,
            campaign_id,
            reporting_request,
            phantom_m: PhantomData,
            phantom_i: PhantomData,
        }
    }
}
impl<M, I> Endpoint for CustomizableGetAdGroupLevelReports<M, I>
where
    M: DeserializeOwned,
    I: DeserializeOwned,
{
    #[allow(clippy::type_complexity)]
    type ParseResponseOutput = (
        Option<Result<ReportingResponseBody<M, I>, GeneralErrorResponse>>,
        StatusCode,
    );
    type RetryReason = ();

    fn render_request(&self) -> io::Result<Request<Body>> {
        let body = serde_json::to_vec(&self.reporting_request)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(
                format!(
                    "https://api.searchads.apple.com/api/v3/reports/campaigns/{}/adgroups",
                    self.campaign_id
                )
                .as_str(),
            )
            .version(Version::HTTP_11)
            .header(USER_AGENT, "curl/7.72.0")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("orgId={}", self.org_id))
            .body(body)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(request)
    }

    fn parse_response(
        &mut self,
        response: Response<Body>,
    ) -> io::Result<EndpointParseResponseOutput<Self::ParseResponseOutput, Self::RetryReason>> {
        let body = match response.status() {
            StatusCode::OK => Some(Ok(serde_json::from_slice::<ReportingResponseBody<M, I>>(
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

pub type GetAdGroupLevelReports = CustomizableGetAdGroupLevelReports<AdGroupLevelRowMetaData, ()>;
