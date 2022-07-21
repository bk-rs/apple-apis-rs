// https://developer.apple.com/documentation/apple_search_ads/useracl paymentModel

use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum PaymentModel {
    #[allow(clippy::upper_case_acronyms)]
    PAYG,
    #[allow(clippy::upper_case_acronyms)]
    LOC,
    #[serde(rename = "")]
    NotSet,
}
