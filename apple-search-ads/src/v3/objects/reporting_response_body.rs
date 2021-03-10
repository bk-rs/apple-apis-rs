// https://developer.apple.com/documentation/apple_search_ads/reportingresponsebody

use serde::Deserialize;

use crate::v3::objects::{
    keyword_insights::KeywordInsights,
    page_detail::PageDetail,
    reporting_response::ReportingResponse,
    row::{
        AdGroupLevelRowMetaData, CampaignLevelRowMetaData, KeywordLevelRowMetaData,
        SearchTermLevelRowMetaData,
    },
};

#[derive(Deserialize, Debug, Clone)]
pub struct ReportingResponseBody<M, I>
where
    M: Sized,
    I: Sized,
{
    pub data: ReportingResponse<M, I>,
    pub pagination: PageDetail,
}

pub type CampaignLevelReportingResponseBody = ReportingResponseBody<CampaignLevelRowMetaData, ()>;
pub type AdGroupLevelReportingResponseBody = ReportingResponseBody<AdGroupLevelRowMetaData, ()>;
pub type KeywordLevelReportingResponseBody =
    ReportingResponseBody<KeywordLevelRowMetaData, KeywordInsights>;
pub type SearchTermLevelReportingResponseBody =
    ReportingResponseBody<SearchTermLevelRowMetaData, ()>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs};

    #[test]
    fn test_de_get_campaign_level_reports() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            fs::read_to_string("tests/v3/response_body_json_files/get_campaign_level_reports.json")
                .unwrap();

        let _: CampaignLevelReportingResponseBody = serde_json::from_str(&json_content)?;

        Ok(())
    }

    #[test]
    fn test_de_get_campaign_level_reports_with_granularity() -> Result<(), Box<dyn error::Error>> {
        let json_content = fs::read_to_string(
            "tests/v3/response_body_json_files/get_campaign_level_reports_with_granularity.json",
        )
        .unwrap();

        let _: CampaignLevelReportingResponseBody = serde_json::from_str(&json_content)?;

        Ok(())
    }

    #[test]
    fn test_de_get_ad_group_level_reports() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            fs::read_to_string("tests/v3/response_body_json_files/get_ad_group_level_reports.json")
                .unwrap();

        let _: AdGroupLevelReportingResponseBody = serde_json::from_str(&json_content)?;

        Ok(())
    }

    #[test]
    fn test_de_get_keyword_level_reports() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            fs::read_to_string("tests/v3/response_body_json_files/get_keyword_level_reports.json")
                .unwrap();

        let _: KeywordLevelReportingResponseBody = serde_json::from_str(&json_content)?;

        Ok(())
    }

    #[test]
    fn test_de_get_search_term_level_reports() -> Result<(), Box<dyn error::Error>> {
        let json_content = fs::read_to_string(
            "tests/v3/response_body_json_files/get_search_term_level_reports.json",
        )
        .unwrap();

        let _: SearchTermLevelReportingResponseBody = serde_json::from_str(&json_content)?;

        Ok(())
    }

    #[test]
    fn test_de_with_empty() -> Result<(), Box<dyn error::Error>> {
        let json_content = fs::read_to_string(
            "tests/v3/response_body_json_files/reporting_response_body_with_empty.json",
        )
        .unwrap();

        let body: CampaignLevelReportingResponseBody = serde_json::from_str(&json_content)?;
        assert_eq!(body.data.reporting_data_response.row.len(), 0);
        assert_eq!(body.pagination.total_results, 0);

        Ok(())
    }
}
