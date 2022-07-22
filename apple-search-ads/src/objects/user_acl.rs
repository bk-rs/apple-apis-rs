// https://developer.apple.com/documentation/apple_search_ads/useracl

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::types::{currency::Currency, payment_model::PaymentModel};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserAcl {
    pub currency: Currency,

    #[serde(rename = "orgId")]
    pub org_id: u64,

    #[serde(rename = "orgName")]
    pub org_name: Box<str>,

    #[serde(rename = "paymentModel")]
    pub payment_model: PaymentModel,

    #[serde(rename = "roleNames")]
    pub role_names: Vec<UserAclRoleName>,

    #[serde(rename = "timeZone")]
    pub time_zone: Box<str>,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum UserAclRoleName {
    #[serde(rename = "API Account Manager")]
    ApiAccountManager,
    #[serde(rename = "API Account Read Only")]
    ApiAccountReadOnly,
    #[serde(rename = "Limited Access: API Read & Write")]
    LimitedAccessApiReadWrite,
    #[serde(rename = "Limited Access: API Read Only")]
    LimitedAccessApiReadOnly,
    /*
    API Campaign Manager
    */
    #[serde(other)]
    Other(Box<str>),
}
impl UserAclRoleName {
    pub fn is_editable(&self) -> bool {
        match self {
            Self::ApiAccountManager | Self::LimitedAccessApiReadWrite => true,
            Self::Other(s) => s.contains("Manager"),
            _ => false,
        }
    }
}
