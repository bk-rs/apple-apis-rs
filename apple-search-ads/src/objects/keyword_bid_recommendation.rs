// https://developer.apple.com/documentation/apple_search_ads/keywordbidrecommendation

use serde::{Deserialize, Serialize};

use crate::objects::money::Money;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeywordBidRecommendation {
    #[serde(rename = "bidMax", skip_serializing_if = "Option::is_none")]
    pub bid_max: Option<Money>,

    #[serde(rename = "bidMin", skip_serializing_if = "Option::is_none")]
    pub bid_min: Option<Money>,
}
