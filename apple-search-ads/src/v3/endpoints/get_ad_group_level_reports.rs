// https://developer.apple.com/documentation/apple_search_ads/get_ad_group_level_reports

use std::marker::PhantomData;

use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Error as HttpError, Method, StatusCode, Version,
};
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeJsonError;

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
    type RenderRequestError = CustomizableGetAdGroupLevelReportsError;

    #[allow(clippy::type_complexity)]
    type ParseResponseOutput = (
        Option<Result<ReportingResponseBody<M, I>, GeneralErrorResponse>>,
        StatusCode,
    );
    type ParseResponseError = CustomizableGetAdGroupLevelReportsError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let body = serde_json::to_vec(&self.reporting_request)
            .map_err(CustomizableGetAdGroupLevelReportsError::SerRequestBodyFailed)?;

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
            .map_err(CustomizableGetAdGroupLevelReportsError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = match response.status() {
            StatusCode::OK => Some(Ok(serde_json::from_slice::<ReportingResponseBody<M, I>>(
                response.body(),
            )
            .map_err(CustomizableGetAdGroupLevelReportsError::DeResponseBodyOkJsonFailed)?)),
            StatusCode::GONE => None,
            _ => Some(Err(serde_json::from_slice::<GeneralErrorResponse>(
                response.body(),
            )
            .map_err(
                CustomizableGetAdGroupLevelReportsError::DeResponseBodyErrJsonFailed,
            )?)),
        };

        Ok((body, response.status()))
    }
}

pub type GetAdGroupLevelReports = CustomizableGetAdGroupLevelReports<AdGroupLevelRowMetaData, ()>;

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum CustomizableGetAdGroupLevelReportsError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeJsonError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    #[error("DeResponseBodyErrJsonFailed {0}")]
    DeResponseBodyErrJsonFailed(SerdeJsonError),
}
