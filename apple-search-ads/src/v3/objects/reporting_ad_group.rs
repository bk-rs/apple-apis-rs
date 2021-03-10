// https://developer.apple.com/documentation/apple_search_ads/reportingadgroup

use serde::Deserialize;

use crate::v3::types::region::Region;

#[derive(Deserialize, Debug, Clone)]
pub struct ReportingAdGroup {
    // TODO
    #[serde(rename = "adGroupId")]
    pub ad_group_id: u64,

    #[serde(rename = "adGroupName")]
    pub ad_group_name: String,

    #[serde(rename = "campaignId")]
    pub campaign_id: u64,

    #[serde(rename = "orgId")]
    pub org_id: u64,

    #[serde(rename = "countryOrRegion")]
    pub country_or_region: Region,
}
