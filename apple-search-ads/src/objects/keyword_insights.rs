// https://developer.apple.com/documentation/apple_search_ads/keywordinsights

use serde::{Deserialize, Serialize};

use crate::objects::keyword_bid_recommendation::KeywordBidRecommendation;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeywordInsights {
    #[serde(rename = "bidRecommendation")]
    pub bid_recommendation: KeywordBidRecommendation,
}
