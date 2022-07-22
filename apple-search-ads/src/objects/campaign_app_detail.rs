// https://developer.apple.com/documentation/apple_search_ads/campaignappdetail

use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct CampaignAppDetail {
    #[serde(rename = "appName")]
    pub app_name: Box<str>,

    #[serde(rename = "adamId")]
    pub adam_id: u64,
}
