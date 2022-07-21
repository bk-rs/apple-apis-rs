// https://developer.apple.com/documentation/apple_search_ads/reportingcampaign

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    objects::{
        campaign::{
            campaign_date_format, CampaignAdChannelType, CampaignDisplayStatus,
            CampaignServingStateReason, CampaignServingStatus, CampaignStatus,
            CampaignSupplySource,
        },
        campaign_app_detail::CampaignAppDetail,
        campaign_country_or_region_serving_state_reasons::CampaignCountryOrRegionServingStateReasons,
        money::Money,
    },
    types::region::Region,
};

//
#[derive(Deserialize, Debug, Clone)]
pub struct ReportingCampaign {
    // TODO
    #[serde(rename = "campaignId")]
    pub campaign_id: u64,

    #[serde(rename = "campaignName")]
    pub campaign_name: String,

    pub deleted: bool,

    #[serde(rename = "campaignStatus")]
    pub campaign_status: CampaignStatus,

    pub app: CampaignAppDetail,

    #[serde(rename = "servingStatus")]
    pub serving_status: CampaignServingStatus,

    #[serde(rename = "servingStateReasons")]
    pub serving_state_reasons: Option<Vec<CampaignServingStateReason>>,

    #[serde(rename = "countriesOrRegions")]
    pub countries_or_regions: Vec<Region>,

    #[serde(with = "campaign_date_format")]
    #[serde(rename = "modificationTime")]
    pub modification_time: DateTime<Utc>,

    #[serde(rename = "totalBudget")]
    pub total_budget: Money,

    #[serde(rename = "dailyBudget")]
    pub daily_budget: Option<Money>,

    #[serde(rename = "displayStatus")]
    pub display_status: CampaignDisplayStatus,

    #[serde(rename = "supplySources")]
    pub supply_sources: Vec<CampaignSupplySource>,

    #[serde(rename = "adChannelType")]
    pub ad_channel_type: CampaignAdChannelType,

    #[serde(rename = "orgId")]
    pub org_id: u64,

    #[serde(rename = "countryOrRegionServingStateReasons")]
    pub country_or_region_serving_state_reasons: CampaignCountryOrRegionServingStateReasons,

    #[serde(rename = "countryOrRegion")]
    pub country_or_region: Region,
}
