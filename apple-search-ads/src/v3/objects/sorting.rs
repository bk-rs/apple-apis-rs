// https://developer.apple.com/documentation/apple_search_ads/sorting

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sorting {
    pub field: String,

    #[serde(rename = "sortOrder")]
    pub sort_order: SortingSortOrder,
}
impl Sorting {
    pub fn new(field: impl Into<String>, sort_order: SortingSortOrder) -> Self {
        Self {
            field: field.into(),
            sort_order,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SortingSortOrder {
    #[allow(clippy::upper_case_acronyms)]
    ASCENDING,
    #[allow(clippy::upper_case_acronyms)]
    DESCENDING,
}
