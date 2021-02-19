// ref https://developer.apple.com/documentation/appstorereceipts/responsebody

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RequestBody<'a> {
    #[serde(rename(serialize = "receipt-data"))]
    pub receipt_data: &'a str,
    pub password: &'a str,
    #[serde(
        rename(serialize = "exclude-old-transactions",),
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_old_transactions: Option<bool>,
}

impl<'a> RequestBody<'a> {
    pub fn new(
        receipt_data: &'a str,
        password: &'a str,
        exclude_old_transactions: Option<bool>,
    ) -> Self {
        Self {
            receipt_data,
            password,
            exclude_old_transactions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            serde_json::to_string(&RequestBody::new("foo", "pw", None))?,
            r#"{"receipt-data":"foo","password":"pw"}"#
        );

        assert_eq!(
            serde_json::to_string(&RequestBody::new("foo", "pw", Some(true)))?,
            r#"{"receipt-data":"foo","password":"pw","exclude-old-transactions":true}"#
        );

        Ok(())
    }
}
