// https://developer.apple.com/documentation/apple_search_ads/useracl currency

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    #[allow(clippy::upper_case_acronyms)]
    AUD,
    #[allow(clippy::upper_case_acronyms)]
    CAD,
    #[allow(clippy::upper_case_acronyms)]
    EUR,
    #[allow(clippy::upper_case_acronyms)]
    GBP,
    #[allow(clippy::upper_case_acronyms)]
    JPY,
    #[allow(clippy::upper_case_acronyms)]
    MXN,
    #[allow(clippy::upper_case_acronyms)]
    NZD,
    #[allow(clippy::upper_case_acronyms)]
    RUB,
    #[allow(clippy::upper_case_acronyms)]
    USD,
    //
    #[serde(other)]
    Other(Box<str>),
}
