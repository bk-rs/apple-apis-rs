// https://developer.apple.com/documentation/apple_search_ads/reportingresponse

use serde::{Deserialize, Serialize};

use crate::objects::reporting_data_response::ReportingDataResponse;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReportingResponse<M, I>
where
    M: Sized,
    I: Sized,
{
    #[serde(rename = "reportingDataResponse")]
    pub reporting_data_response: ReportingDataResponse<M, I>,
}
