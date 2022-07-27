// https://developer.apple.com/documentation/apple_search_ads/reportingdataresponse

use serde::{Deserialize, Serialize};

use crate::objects::{grand_totals_row::GrandTotalsRow, row::Row};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReportingDataResponse<M, I>
where
    M: Sized,
    I: Sized,
{
    #[serde(rename = "grandTotals", skip_serializing_if = "Option::is_none")]
    pub grand_totals: Option<GrandTotalsRow>,

    pub row: Vec<Row<M, I>>,
}
