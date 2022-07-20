// https://developer.apple.com/documentation/apple_search_ads/pagination

use core::cmp::min;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<u32>,
}
impl Pagination {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_limit(&mut self, val: u32) -> &mut Self {
        self.limit = Some(min(val, 1000));
        self
    }
    pub fn set_offset(&mut self, val: u32) -> &mut Self {
        self.offset = Some(val);
        self
    }
}
