// https://developer.apple.com/documentation/apple_search_ads/extendedspendrow

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::objects::money::Money;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExtendedSpendRow {
    pub date: ExtendedSpendRowDate,

    #[serde(rename = "avgCPA", skip_serializing_if = "Option::is_none")]
    pub avg_cpa: Option<Money>,

    #[serde(rename = "avgCPT", skip_serializing_if = "Option::is_none")]
    pub avg_cpt: Option<Money>,

    #[serde(rename = "conversionRate", skip_serializing_if = "Option::is_none")]
    pub conversion_rate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub installs: Option<u64>,

    #[serde(rename = "latOffInstalls", skip_serializing_if = "Option::is_none")]
    pub lat_off_installs: Option<u64>,

    #[serde(rename = "latOnInstalls", skip_serializing_if = "Option::is_none")]
    pub lat_on_installs: Option<u64>,

    #[serde(rename = "localSpend", skip_serializing_if = "Option::is_none")]
    pub local_spend: Option<Money>,

    #[serde(rename = "newDownloads", skip_serializing_if = "Option::is_none")]
    pub new_downloads: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redownloads: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub taps: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttr: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ExtendedSpendRowDate {
    #[serde(with = "extended_spend_row_date_date_format")]
    Date(NaiveDate),
    #[serde(with = "extended_spend_row_date_date_and_hour_format")]
    DateAndHour(NaiveDateTime),
}
impl ExtendedSpendRowDate {
    pub fn to_datetime(&self) -> NaiveDateTime {
        match self {
            Self::Date(d) => d.and_hms_opt(0, 0, 0).expect(""),
            Self::DateAndHour(dt) => dt.to_owned(),
        }
    }
}

pub mod extended_spend_row_date_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod extended_spend_row_date_date_and_hour_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT_SER: &str = "%Y-%m-%d %H";
    const FORMAT_DE: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT_SER));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(format!("{s}:00:00").as_str(), FORMAT_DE)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use serde_json::Value;

    #[test]
    fn test_ser_and_de_date() -> Result<(), Box<dyn error::Error>> {
        impl ExtendedSpendRowDate {
            fn from_ymd(year: i32, month: u32, day: u32) -> Self {
                Self::Date(NaiveDate::from_ymd_opt(year, month, day).expect(""))
            }
            fn from_ymd_and_hour(year: i32, month: u32, day: u32, hour: u32) -> Self {
                Self::DateAndHour(
                    NaiveDate::from_ymd_opt(year, month, day)
                        .expect("")
                        .and_hms_opt(hour, 0, 0)
                        .expect(""),
                )
            }
        }

        assert_eq!(
            serde_json::to_value(ExtendedSpendRowDate::from_ymd(2020, 1, 1))?,
            Value::String("2020-01-01".into())
        );
        assert_eq!(
            serde_json::from_value::<ExtendedSpendRowDate>(Value::String("2020-01-01".into()))?,
            ExtendedSpendRowDate::from_ymd(2020, 1, 1)
        );

        assert_eq!(
            serde_json::to_value(ExtendedSpendRowDate::from_ymd_and_hour(2020, 1, 1, 0))?,
            Value::String("2020-01-01 00".into())
        );
        assert_eq!(
            serde_json::from_value::<ExtendedSpendRowDate>(Value::String("2020-01-01 00".into()))?,
            ExtendedSpendRowDate::from_ymd_and_hour(2020, 1, 1, 0)
        );

        assert_eq!(
            serde_json::to_value(ExtendedSpendRowDate::from_ymd_and_hour(2020, 1, 1, 1))?,
            Value::String("2020-01-01 01".into())
        );
        assert_eq!(
            serde_json::from_value::<ExtendedSpendRowDate>(Value::String("2020-01-01 01".into()))?,
            ExtendedSpendRowDate::from_ymd_and_hour(2020, 1, 1, 1)
        );

        Ok(())
    }
}
