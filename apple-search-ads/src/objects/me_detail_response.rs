// https://developer.apple.com/documentation/apple_search_ads/medetailresponse

use serde::{Deserialize, Serialize};

use crate::objects::me_detail::MeDetail;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MeDetailResponse {
    pub data: MeDetail,
}
