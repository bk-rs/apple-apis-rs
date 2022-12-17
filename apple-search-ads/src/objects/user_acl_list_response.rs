// https://developer.apple.com/documentation/apple_search_ads/useracllistresponse

use serde::{Deserialize, Serialize};

use crate::objects::{page_detail::PageDetail, user_acl::UserAcl};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserAclListResponse {
    pub data: Vec<UserAcl>,
    pub pagination: Option<PageDetail>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use crate::{
        objects::user_acl::UserAclRoleName,
        types::{currency::Currency, payment_model::PaymentModel},
    };

    #[test]
    fn test_v4_de() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            include_str!("../../tests/v4/response_body_json_files/user_acl_list_response.json");

        let body: UserAclListResponse = serde_json::from_str(json_content)?;
        println!("{body:?}");

        assert_eq!(body.data.len(), 1);
        assert!(body.pagination.is_none());
        let user_acl = body.data.first().unwrap();
        assert_eq!(user_acl.currency, Currency::USD);
        assert_eq!(user_acl.org_id, 40669820);
        assert_eq!(user_acl.org_name.as_ref(), "org name example");
        assert_eq!(user_acl.payment_model, PaymentModel::PAYG);
        assert_eq!(
            user_acl.role_names,
            vec![UserAclRoleName::Other("Admin".into())]
        );
        assert_eq!(user_acl.time_zone.as_ref(), "America/Los_Angeles");

        Ok(())
    }

    #[test]
    fn test_v3_de() -> Result<(), Box<dyn error::Error>> {
        let json_content =
            include_str!("../../tests/v3/response_body_json_files/user_acl_list_response.json");

        let body: UserAclListResponse = serde_json::from_str(json_content)?;
        println!("{body:?}");

        assert_eq!(body.data.len(), 1);
        assert!(body.pagination.is_none());
        let user_acl = body.data.first().unwrap();
        assert_eq!(user_acl.currency, Currency::USD);
        assert_eq!(user_acl.org_id, 1);
        assert_eq!(user_acl.org_name.as_ref(), "orgName");
        assert_eq!(user_acl.payment_model, PaymentModel::LOC);
        assert_eq!(
            user_acl.role_names,
            vec![UserAclRoleName::Other("Campaign Group Manager".into())]
        );
        assert_eq!(user_acl.time_zone.as_ref(), "Asia/Hong_Kong");

        Ok(())
    }
}
