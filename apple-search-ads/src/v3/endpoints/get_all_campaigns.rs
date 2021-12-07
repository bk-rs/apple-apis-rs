// https://developer.apple.com/documentation/apple_search_ads/get_user_acl

use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Error as HttpError, Method, StatusCode, Version,
};
use once_cell::sync::Lazy;
use serde_json::Error as SerdeJsonError;
use url::Url;

use crate::v3::objects::{
    campaign_list_response::CampaignListResponse, error_response_body::GeneralErrorResponse,
    pagination::Pagination,
};

use super::endpoint_prelude::*;

static URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.searchads.apple.com/api/v3/campaigns").unwrap());

#[derive(Debug)]
pub struct GetAllCampaigns {
    org_id: u64,
    pagination: Option<Pagination>,
}
impl GetAllCampaigns {
    pub fn new(org_id: u64) -> Self {
        Self {
            org_id,
            pagination: None,
        }
    }
    pub fn set_pagination(&mut self, pagination: impl Into<Option<Pagination>>) -> &mut Self {
        self.pagination = pagination.into();
        self
    }
}
impl Endpoint for GetAllCampaigns {
    type RenderRequestError = GetAllCampaignsError;

    type ParseResponseOutput = (
        Option<Result<CampaignListResponse, GeneralErrorResponse>>,
        StatusCode,
    );
    type ParseResponseError = GetAllCampaignsError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut uri = URL.to_owned();

        if let Some(ref pagination) = self.pagination {
            if let Some(limit) = pagination.limit {
                uri.query_pairs_mut()
                    .append_pair("limit", format!("{}", limit).as_str());
            }
            if let Some(offset) = pagination.offset {
                uri.query_pairs_mut()
                    .append_pair("offset", format!("{}", offset).as_str());
            }
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri(uri.as_str())
            .version(Version::HTTP_11)
            .header(USER_AGENT, "curl/7.72.0")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("orgId={}", self.org_id))
            .body(vec![])
            .map_err(GetAllCampaignsError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = match response.status() {
            StatusCode::OK => Some(Ok(serde_json::from_slice::<CampaignListResponse>(
                response.body(),
            )
            .map_err(GetAllCampaignsError::DeResponseBodyOkJsonFailed)?)),
            StatusCode::GONE => None,
            _ => Some(Err(serde_json::from_slice::<GeneralErrorResponse>(
                response.body(),
            )
            .map_err(GetAllCampaignsError::DeResponseBodyErrJsonFailed)?)),
        };

        Ok((body, response.status()))
    }
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum GetAllCampaignsError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    #[error("DeResponseBodyErrJsonFailed {0}")]
    DeResponseBodyErrJsonFailed(SerdeJsonError),
}
