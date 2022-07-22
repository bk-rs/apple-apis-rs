// https://developer.apple.com/documentation/apple_search_ads/spendrow

use serde::{Deserialize, Serialize};

use crate::objects::money::Money;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpendRow {
    #[serde(rename = "avgCPA")]
    pub avg_cpa: Money,

    #[serde(rename = "avgCPT")]
    pub avg_cpt: Money,

    #[serde(rename = "conversionRate")]
    pub conversion_rate: f64,

    pub impressions: u64,

    pub installs: u64,

    #[serde(rename = "latOffInstalls")]
    pub lat_off_installs: u64,

    #[serde(rename = "latOnInstalls")]
    pub lat_on_installs: u64,

    #[serde(rename = "localSpend")]
    pub local_spend: Money,

    #[serde(rename = "newDownloads")]
    pub new_downloads: u64,

    pub redownloads: u64,

    pub taps: u64,

    pub ttr: f64,
}
