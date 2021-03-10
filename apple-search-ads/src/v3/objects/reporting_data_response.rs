// https://developer.apple.com/documentation/apple_search_ads/reportingdataresponse

use serde::Deserialize;

use crate::v3::objects::{grand_totals_row::GrandTotalsRow, row::Row};

#[derive(Deserialize, Debug, Clone)]
pub struct ReportingDataResponse<M, I>
where
    M: Sized,
    I: Sized,
{
    #[serde(rename = "grandTotals")]
    pub grand_totals: Option<GrandTotalsRow>,

    pub row: Vec<Row<M, I>>,
}
