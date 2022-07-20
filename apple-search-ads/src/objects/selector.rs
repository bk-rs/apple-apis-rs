// https://developer.apple.com/documentation/apple_search_ads/selector

use serde::{Deserialize, Serialize};

use crate::objects::{condition::Condition, pagination::Pagination, sorting::Sorting};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Selector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Box<str>>>,

    #[serde(rename = "orderBy")]
    pub order_by: Vec<Sorting>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}
impl Selector {
    pub fn new(order_by: Vec<Sorting>) -> Self {
        Self {
            conditions: None,
            fields: None,
            order_by,
            pagination: None,
        }
    }
    pub fn set_conditions(&mut self, val: impl Into<Option<Vec<Condition>>>) -> &mut Self {
        self.conditions = val.into();
        self
    }
    pub fn set_fields(&mut self, val: impl Into<Option<Vec<Box<str>>>>) -> &mut Self {
        self.fields = val.into();
        self
    }
    pub fn set_pagination(&mut self, val: impl Into<Option<Pagination>>) -> &mut Self {
        self.pagination = val.into();
        self
    }
}
