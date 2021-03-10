// https://developer.apple.com/documentation/apple_search_ads/grandtotalsrow

use serde::Deserialize;

use crate::v3::objects::spend_row::SpendRow;

#[derive(Deserialize, Debug, Clone)]
pub struct GrandTotalsRow {
    pub other: bool,
    pub total: SpendRow,
}
