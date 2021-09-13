// ref https://developer.apple.com/documentation/appstorereceipts/requestbody

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use serde_aux::field_attributes::{
    deserialize_bool_from_anything, deserialize_datetime_utc_from_milliseconds,
    deserialize_number_from_string,
};
use serde_enum_str::Deserialize_enum_str;
use serde_json::{Map, Value};

use crate::types::status::Status;

#[derive(Debug)]
pub enum ResponseBody {
    Success(ResponseBodyWithSuccess),
    Error(ResponseBodyWithError),
}
impl<'de> Deserialize<'de> for ResponseBody {
    fn deserialize<D>(deserializer: D) -> Result<ResponseBody, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = Map::deserialize(deserializer)?;

        let status: Status = map
            .get("status")
            .ok_or_else(|| de::Error::missing_field("status"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        match status {
            Status::Success => ResponseBodyWithSuccess::deserialize(rest)
                .map(ResponseBody::Success)
                .map_err(de::Error::custom),
            _ => ResponseBodyWithError::deserialize(rest)
                .map(ResponseBody::Error)
                .map_err(de::Error::custom),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseBodyWithSuccess {
    pub status: Status,
    pub environment: Environment,
    pub receipt: Receipt,
    pub latest_receipt: Option<String>,
    pub latest_receipt_info: Option<Vec<LatestReceiptInfo>>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseBodyWithError {
    pub status: Status,
    pub environment: Option<Environment>,
    pub is_retryable: Option<bool>,
    pub exception: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

#[derive(Deserialize_enum_str, Debug, PartialEq, Eq)]
pub enum ReceiptType {
    Production,
    #[allow(clippy::upper_case_acronyms)]
    ProductionVPP,
    ProductionSandbox,
    #[allow(clippy::upper_case_acronyms)]
    ProductionVPPSandbox,
    #[serde(other)]
    Other(String),
}

#[derive(Deserialize, Debug)]
pub struct Receipt {
    pub receipt_type: ReceiptType,

    pub adam_id: usize,
    pub app_item_id: usize,
    pub bundle_id: String,
    pub application_version: String,
    pub download_id: Option<usize>,

    pub version_external_identifier: usize,

    #[serde(
        rename(deserialize = "receipt_creation_date_ms"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub receipt_creation_date: DateTime<Utc>,
    pub receipt_creation_date_pst: String,

    #[serde(
        rename(deserialize = "request_date_ms"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub request_date: DateTime<Utc>,
    pub request_date_pst: String,

    #[serde(
        rename(deserialize = "original_purchase_date_ms"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub original_purchase_date: DateTime<Utc>,
    pub original_purchase_date_pst: String,

    pub original_application_version: Option<String>,

    pub in_app: Option<Vec<ReceiptInApp>>,

    #[serde(
        rename(deserialize = "expiration_date_ms"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub expiration_date: Option<DateTime<Utc>>,
    pub expiration_date_pst: Option<String>,

    #[serde(
        rename(deserialize = "preorder_date_ms"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub preorder_date: Option<DateTime<Utc>>,
    pub preorder_date_pst: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub quantity: usize,

    pub product_id: String,

    pub transaction_id: String,
    pub original_transaction_id: String,

    #[serde(
        rename(deserialize = "purchase_date_ms"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub purchase_date: DateTime<Utc>,
    pub purchase_date_pst: String,

    #[serde(
        rename(deserialize = "original_purchase_date_ms"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub original_purchase_date: DateTime<Utc>,
    pub original_purchase_date_pst: String,

    #[serde(
        rename(deserialize = "expires_date_ms"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub expires_date: Option<DateTime<Utc>>,
    pub expires_date_pst: Option<String>,

    #[serde(
        rename(deserialize = "cancellation_date_ms"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub cancellation_date: Option<DateTime<Utc>>,
    pub cancellation_date_pst: Option<String>,

    pub cancellation_reason: Option<String>,

    pub web_order_line_item_id: Option<String>,

    #[serde(default, deserialize_with = "deserialize_bool_from_anything_option")]
    pub is_trial_period: Option<bool>,

    pub promotional_offer_id: Option<String>,

    #[serde(default, deserialize_with = "deserialize_bool_from_anything_option")]
    pub is_in_intro_offer_period: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct LatestReceiptInfo {
    #[serde(flatten)]
    pub transaction: Transaction,

    pub subscription_group_identifier: Option<String>,

    #[serde(default, deserialize_with = "deserialize_bool_from_anything_option")]
    pub is_upgraded: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ReceiptInApp {
    #[serde(flatten)]
    pub transaction: Transaction,
}

//
//
//
fn deserialize_bool_from_anything_option<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bool_from_anything(deserializer).map(Some)
}

fn deserialize_datetime_utc_from_milliseconds_option<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_datetime_utc_from_milliseconds(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn simple_error() -> Result<(), Box<dyn error::Error>> {
        match serde_json::from_str(r#"{"status":21007}"#)? {
            ResponseBody::Error(body) => {
                assert_eq!(body.status, Status::Error21007);
                assert_eq!(body.environment, None);
                assert_eq!(body.is_retryable, None);
                assert_eq!(body.exception, None)
            }
            _ => panic!(),
        }

        match serde_json::from_str(
            r#"{"status":21010, "environment":"Production", "is_retryable":false, "exception":"com.apple.jingle.commercelogic.inapplocker.exception.MZInAppLockerAccessException"}"#,
        )? {
            ResponseBody::Error(body) => {
                assert_eq!(body.status, Status::Error21010);
                assert_eq!(body.environment, Some(Environment::Production));
                assert_eq!(body.is_retryable, Some(false));
                assert_eq!(body.exception, Some("com.apple.jingle.commercelogic.inapplocker.exception.MZInAppLockerAccessException".to_owned()));
            }
            _ => panic!(),
        }

        match serde_json::from_str(
            r#"{"status":21104, "environment":"Production", "is_retryable":true, "exception":"com.apple.jingle.commercelogic.inapplocker.exception.MZInAppLockerAccessException"}"#,
        )? {
            ResponseBody::Error(body) => {
                assert_eq!(body.status, Status::InternalDataAccessError(21104));
                assert_eq!(body.environment, Some(Environment::Production));
                assert_eq!(body.is_retryable, Some(true));
                assert_eq!(body.exception, Some("com.apple.jingle.commercelogic.inapplocker.exception.MZInAppLockerAccessException".to_owned()));
            }
            _ => panic!(),
        }

        Ok(())
    }
}
