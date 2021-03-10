// https://developer.apple.com/documentation/apple_search_ads/useracl currency

use serde::Deserialize;

#[derive(strum::Display, Deserialize, PartialEq, Debug, Clone)]
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
}
