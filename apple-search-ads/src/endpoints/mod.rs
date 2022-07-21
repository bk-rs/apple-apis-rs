//
pub mod common;

pub use common::{EndpointError, EndpointRet};

pub const HEADER_KEY_X_AP_CONTEXT: &str = "X-AP-Context";

//
// Access Control List
//
pub mod get_me_details;
pub mod get_user_acl;

pub use get_me_details::GetMeDetails;
pub use get_user_acl::GetUserAcl;

//
// Campaign Endpoints
//
pub mod get_all_campaigns;

pub use get_all_campaigns::GetAllCampaigns;

//
// Reports Endpoints
//
pub mod get_ad_group_level_reports;
pub mod get_campaign_level_reports;
pub mod get_keyword_level_reports;
pub mod get_search_term_level_reports;

pub use get_ad_group_level_reports::{CustomizableGetAdGroupLevelReports, GetAdGroupLevelReports};
pub use get_campaign_level_reports::{
    CustomizableGetCampaignLevelReports, GetCampaignLevelReports,
};
pub use get_keyword_level_reports::{CustomizableGetKeywordLevelReports, GetKeywordLevelReports};
pub use get_search_term_level_reports::{
    CustomizableGetSearchTermLevelReports, GetSearchTermLevelReports,
};
