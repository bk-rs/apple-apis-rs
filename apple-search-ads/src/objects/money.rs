// https://developer.apple.com/documentation/apple_search_ads/money

use serde::Deserialize;

use crate::types::currency::Currency;

#[derive(Deserialize, Debug, Clone)]
pub struct Money {
    pub currency: Currency,

    pub amount: Box<str>,
}
impl Default for Money {
    fn default() -> Self {
        Self {
            currency: Currency::USD,
            amount: "0".into(),
        }
    }
}
