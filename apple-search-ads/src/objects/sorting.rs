// https://developer.apple.com/documentation/apple_search_ads/sorting

use serde::{Deserialize, Serialize};

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sorting {
    pub field: Box<str>,

    #[serde(rename = "sortOrder")]
    pub sort_order: SortingSortOrder,
}

impl Default for Sorting {
    fn default() -> Self {
        Self::new("localSpend", SortingSortOrder::DESCENDING)
    }
}

impl Sorting {
    pub fn new(field: impl AsRef<str>, sort_order: SortingSortOrder) -> Self {
        Self {
            field: field.as_ref().into(),
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
