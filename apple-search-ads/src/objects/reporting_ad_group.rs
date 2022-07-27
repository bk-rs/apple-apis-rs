// https://developer.apple.com/documentation/apple_search_ads/reportingadgroup

use serde::{Deserialize, Serialize};

use crate::types::region::Region;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReportingAdGroup {
    // TODO
    #[serde(rename = "adGroupId")]
    pub ad_group_id: u64,

    #[serde(rename = "adGroupName")]
    pub ad_group_name: Box<str>,

    #[serde(rename = "campaignId")]
    pub campaign_id: u64,

    #[serde(rename = "orgId")]
    pub org_id: u64,

    // Some when groupBy countryOrRegion
    #[serde(rename = "countryOrRegion", skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<Region>,
}
