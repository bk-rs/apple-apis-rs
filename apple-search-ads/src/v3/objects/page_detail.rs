// https://developer.apple.com/documentation/apple_search_ads/pagedetail

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PageDetail {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: u64,
    #[serde(rename = "startIndex")]
    pub start_index: u64,
    #[serde(rename = "totalResults")]
    pub total_results: u64,
}
impl PageDetail {
    pub fn next_pagination_offset(&self) -> Option<u64> {
        if self.total_results > self.start_index + self.items_per_page {
            Some(self.start_index + self.items_per_page)
        } else {
            None
        }
    }
}
