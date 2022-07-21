// https://developer.apple.com/documentation/apple_search_ads/get_all_campaigns

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use url::Url;

use crate::{
    endpoints::{
        common::{EndpointError, EndpointRet},
        HEADER_KEY_X_AP_CONTEXT,
    },
    objects::{campaign_list_response::CampaignListResponse, pagination::Pagination},
};

pub const URL: &str = "https://api.searchads.apple.com/api/v4/campaigns";

//
#[derive(Debug, Clone)]
pub struct GetAllCampaigns {
    pub org_id: u64,
    pub pagination: Option<Pagination>,
    //
    pub access_token: Box<str>,
}

impl GetAllCampaigns {
    pub fn new(org_id: u64, access_token: impl AsRef<str>) -> Self {
        Self {
            org_id,
            pagination: None,
            access_token: access_token.as_ref().into(),
        }
    }

    pub fn set_pagination(&mut self, pagination: impl Into<Option<Pagination>>) -> &mut Self {
        self.pagination = pagination.into();
        self
    }
}

impl Endpoint for GetAllCampaigns {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CampaignListResponse>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;

        if let Some(ref pagination) = self.pagination {
            if let Some(limit) = pagination.limit {
                url.query_pairs_mut()
                    .append_pair("limit", format!("{}", limit).as_str());
            }
            if let Some(offset) = pagination.offset {
                url.query_pairs_mut()
                    .append_pair("offset", format!("{}", offset).as_str());
            }
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header(USER_AGENT, "apple-search-ads")
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .header(CONTENT_TYPE, MIME_APPLICATION_JSON)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(HEADER_KEY_X_AP_CONTEXT, format!("orgId={}", self.org_id))
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
        let ep = GetAllCampaigns::new(1, "TOKEN");
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), "GET");
        assert_eq!(
            req.uri(),
            "https://api.searchads.apple.com/api/v4/campaigns"
        );
        assert_eq!(req.headers().get("Authorization").unwrap(), "Bearer TOKEN");
        assert_eq!(req.headers().get("X-AP-Context").unwrap(), "orgId=1");
    }
}
