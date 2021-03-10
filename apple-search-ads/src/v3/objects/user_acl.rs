// https://developer.apple.com/documentation/apple_search_ads/useracl

use serde::{Deserialize, Serialize};

use crate::v3::types::{currency::Currency, payment_model::PaymentModel};

#[derive(Deserialize, Debug, Clone)]
pub struct UserAcl {
    pub currency: Currency,

    #[serde(rename = "orgId")]
    pub org_id: u64,

    #[serde(rename = "orgName")]
    pub org_name: String,

    #[serde(rename = "paymentModel")]
    pub payment_model: PaymentModel,

    #[serde(rename = "roleNames")]
    pub role_names: Vec<UserAclRoleName>,

    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum UserAclRoleName {
    #[serde(rename = "Account Read Only")]
    AccountReadOnly,
    #[serde(rename = "Read Only")]
    ReadOnly,
    #[serde(rename = "Campaign Manager")]
    CampaignManager,
    #[serde(rename = "Campaign Group Manager")]
    CampaignGroupManager,
    Admin,
}
impl UserAclRoleName {
    pub fn is_editable(&self) -> bool {
        matches!(
            self,
            Self::CampaignManager | Self::CampaignGroupManager | Self::Admin
        )
    }
}
