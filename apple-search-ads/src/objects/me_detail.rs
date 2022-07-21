// https://developer.apple.com/documentation/apple_search_ads/medetail

use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MeDetail {
    #[serde(rename = "parentOrgId")]
    pub parent_org_id: u64,
    #[serde(rename = "userId")]
    pub user_id: u64,
}
