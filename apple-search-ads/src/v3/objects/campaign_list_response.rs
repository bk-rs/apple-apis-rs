// https://developer.apple.com/documentation/apple_search_ads/campaignlistresponse

use serde::Deserialize;

use crate::v3::objects::{campaign::Campaign, page_detail::PageDetail};

#[derive(Deserialize, Debug, Clone)]
pub struct CampaignListResponse {
    pub data: Vec<Campaign>,
    pub pagination: PageDetail,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs};

    use crate::v3::types::payment_model::PaymentModel;

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            fs::read_to_string("tests/v3/response_body_json_files/campaign_list_response.json")
                .unwrap();

        let body: CampaignListResponse = serde_json::from_str(&json_content)?;
        println!("{:?}", body);

        assert_eq!(body.data.len(), 1);
        assert_eq!(body.pagination.total_results, 1);
        let campaign = body.data.first().unwrap();
        assert_eq!(campaign.payment_model, PaymentModel::LOC);

        Ok(())
    }

    #[test]
    fn test_de_with_empty() -> Result<(), Box<dyn error::Error>> {
        let json_content = fs::read_to_string(
            "tests/v3/response_body_json_files/campaign_list_response_with_empty.json",
        )
        .unwrap();

        let body: CampaignListResponse = serde_json::from_str(&json_content)?;
        assert_eq!(body.data.len(), 0);
        assert_eq!(body.pagination.total_results, 0);

        Ok(())
    }
}
