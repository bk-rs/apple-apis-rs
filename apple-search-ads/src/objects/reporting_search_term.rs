// https://developer.apple.com/documentation/apple_search_ads/reportingsearchterm

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::types::region::Region;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReportingSearchTerm {
    // TODO
    #[serde(rename = "searchTermText")]
    pub search_term_text: Option<String>,

    #[serde(rename = "searchTermSource")]
    pub search_term_source: SearchTermSource,

    #[serde(rename = "keywordId")]
    pub keyword_id: Option<u64>,

    pub keyword: Option<Box<str>>,

    #[serde(rename = "matchType")]
    pub match_type: ReportingSearchTermMatchType,

    #[serde(rename = "adGroupId")]
    pub ad_group_id: u64,

    #[serde(rename = "adGroupName")]
    pub ad_group_name: Box<str>,

    // Some when groupBy countryOrRegion
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: Option<Region>,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum SearchTermSource {
    #[allow(clippy::upper_case_acronyms)]
    AUTO,
    #[allow(clippy::upper_case_acronyms)]
    TARGETED,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum ReportingSearchTermMatchType {
    #[allow(clippy::upper_case_acronyms)]
    AUTO,
    #[allow(clippy::upper_case_acronyms)]
    EXACT,
    #[allow(clippy::upper_case_acronyms)]
    BROAD,
}
