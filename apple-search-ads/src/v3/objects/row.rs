// https://developer.apple.com/documentation/apple_search_ads/row

use serde::Deserialize;

use crate::v3::objects::{
    extended_spend_row::ExtendedSpendRow, reporting_ad_group::ReportingAdGroup,
    reporting_campaign::ReportingCampaign, reporting_keyword::ReportingKeyword,
    reporting_search_term::ReportingSearchTerm, spend_row::SpendRow,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Row<M, I>
where
    M: Sized,
    I: Sized,
{
    pub granularity: Option<Vec<ExtendedSpendRow>>,

    pub other: bool,

    pub total: Option<SpendRow>,

    pub metadata: M,

    pub insights: Option<I>,
}

pub type CampaignLevelRowMetaData = ReportingCampaign;
pub type AdGroupLevelRowMetaData = ReportingAdGroup;
pub type KeywordLevelRowMetaData = ReportingKeyword;
pub type SearchTermLevelRowMetaData = ReportingSearchTerm;
