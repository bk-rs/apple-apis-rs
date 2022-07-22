// https://developer.apple.com/documentation/apple_search_ads/grandtotalsrow

use serde::{Deserialize, Serialize};

use crate::objects::spend_row::SpendRow;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GrandTotalsRow {
    pub other: bool,
    pub total: SpendRow,
}
