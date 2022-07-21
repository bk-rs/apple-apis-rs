// https://developer.apple.com/documentation/apple_search_ads/keywordbidrecommendation

use serde::Deserialize;

use crate::objects::money::Money;

//
#[derive(Deserialize, Debug, Clone)]
pub struct KeywordBidRecommendation {
    #[serde(rename = "bidMax")]
    pub bid_max: Option<Money>,

    #[serde(rename = "bidMin")]
    pub bid_min: Option<Money>,
}
