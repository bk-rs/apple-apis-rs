// https://developer.apple.com/documentation/apple_search_ads/useracllistresponse

use serde::Deserialize;

use crate::v3::objects::{page_detail::PageDetail, user_acl::UserAcl};

#[derive(Deserialize, Debug, Clone)]
pub struct UserAclListResponse {
    pub data: Vec<UserAcl>,
    pub pagination: Option<PageDetail>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs};

    use crate::v3::{
        objects::user_acl::UserAclRoleName,
        types::{currency::Currency, payment_model::PaymentModel},
    };

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            fs::read_to_string("tests/v3/response_body_json_files/user_acl_list_response.json")
                .unwrap();

        let body: UserAclListResponse = serde_json::from_str(&json_content)?;
        println!("{:?}", body);

        assert_eq!(body.data.len(), 1);
        assert!(body.pagination.is_none());
        let user_acl = body.data.first().unwrap();
        assert_eq!(user_acl.currency, Currency::USD);
        assert_eq!(user_acl.org_id, 1);
        assert_eq!(user_acl.org_name, "orgName");
        assert_eq!(user_acl.payment_model, PaymentModel::LOC);
        assert_eq!(
            user_acl.role_names,
            vec![UserAclRoleName::CampaignGroupManager]
        );
        assert_eq!(user_acl.time_zone, "Asia/Hong_Kong");

        Ok(())
    }
}
