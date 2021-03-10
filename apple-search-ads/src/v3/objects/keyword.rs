// https://developer.apple.com/documentation/apple_search_ads/keyword

use serde::Deserialize;

#[derive(Deserialize, PartialEq, strum::Display, Debug, Clone)]
pub enum KeywordMatchType {
    #[allow(clippy::upper_case_acronyms)]
    EXACT,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    BROAD,
}
