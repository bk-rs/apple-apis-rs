// https://developer.apple.com/documentation/apple_search_ads/reportingkeyword

use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

use crate::types::region::Region;

//
#[derive(Deserialize, Debug, Clone)]
pub struct ReportingKeyword {
    // TODO
    #[serde(rename = "keywordId")]
    pub keyword_id: u64,

    pub keyword: Box<str>,

    #[serde(rename = "matchType")]
    pub match_type: ReportingKeywordMatchType,

    #[serde(rename = "adGroupId")]
    pub ad_group_id: u64,

    #[serde(rename = "adGroupName")]
    pub ad_group_name: Box<str>,

    #[serde(rename = "countryOrRegion")]
    pub country_or_region: Region,
}

#[derive(Deserialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum ReportingKeywordMatchType {
    #[allow(clippy::upper_case_acronyms)]
    AUTO,
    #[allow(clippy::upper_case_acronyms)]
    EXACT,
    #[allow(clippy::upper_case_acronyms)]
    BROAD,
}
