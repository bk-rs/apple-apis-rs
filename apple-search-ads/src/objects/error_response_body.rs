// https://developer.apple.com/documentation/apple_search_ads/errorresponsebody

use serde::{Deserialize, Serialize};

use super::error_response_item::ErrorResponseItem;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorResponseBody {
    pub errors: Vec<ErrorResponseItem>,
}
