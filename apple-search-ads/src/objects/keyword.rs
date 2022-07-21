// https://developer.apple.com/documentation/apple_search_ads/keyword

use serde_enum_str::Deserialize_enum_str;

//
#[derive(Deserialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum KeywordMatchType {
    #[allow(clippy::upper_case_acronyms)]
    EXACT,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    BROAD,
}
