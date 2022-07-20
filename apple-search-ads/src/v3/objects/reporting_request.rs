// https://developer.apple.com/documentation/apple_search_ads/reportingrequest

use chrono::NaiveDate;
use serde::Serialize;

use crate::v3::objects::selector::Selector;

#[derive(Serialize, Debug, Clone)]
pub struct ReportingRequest {
    #[serde(with = "reporting_request_date_format")]
    #[serde(rename = "startTime")]
    pub start_time: NaiveDate,

    #[serde(with = "reporting_request_date_format")]
    #[serde(rename = "endTime")]
    pub end_time: NaiveDate,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<ReportingRequestGranularity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupBy")]
    pub group_by: Option<Vec<ReportingRequestGroupBy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnGrandTotals")]
    pub return_grand_totals: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnRecordsWithNoMetrics")]
    pub return_records_with_no_metrics: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnRowTotals")]
    pub return_row_totals: Option<bool>,

    pub selector: Selector,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeZone")]
    pub time_zone: Option<ReportingRequestTimeZone>,
}
impl ReportingRequest {
    pub fn new(start_time: NaiveDate, end_time: NaiveDate, selector: Selector) -> Self {
        Self {
            start_time,
            end_time,
            granularity: None,
            group_by: None,
            return_grand_totals: None,
            return_records_with_no_metrics: None,
            return_row_totals: None,
            selector,
            time_zone: None,
        }
    }

    pub fn set_granularity(
        &mut self,
        val: impl Into<Option<ReportingRequestGranularity>>,
    ) -> &mut Self {
        self.granularity = val.into();

        // https://developer.apple.com/documentation/apple_search_ads/row
        // Note: if granularity is specified in the payload, then returnRowTotals and returnGrandTotals must be false
        self.return_row_totals = Some(false);
        self.return_grand_totals = Some(false);

        self
    }

    pub fn set_group_by(
        &mut self,
        val: impl Into<Option<Vec<ReportingRequestGroupBy>>>,
    ) -> &mut Self {
        self.group_by = val.into();
        self
    }

    pub fn set_return_grand_totals(&mut self, val: impl Into<Option<bool>>) -> &mut Self {
        if self.granularity.is_none() {
            self.return_grand_totals = val.into();
        }
        self
    }

    pub fn set_return_records_with_no_metrics(
        &mut self,
        val: impl Into<Option<bool>>,
    ) -> &mut Self {
        self.return_records_with_no_metrics = val.into();
        self
    }

    pub fn set_return_row_totals(&mut self, val: impl Into<Option<bool>>) -> &mut Self {
        if self.granularity.is_none() {
            self.return_row_totals = val.into();
        }
        self
    }

    pub fn set_time_zone(&mut self, val: impl Into<Option<ReportingRequestTimeZone>>) -> &mut Self {
        self.time_zone = val.into();
        self
    }
}

pub mod reporting_request_date_format {
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

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
pub enum ReportingRequestGroupBy {
    #[serde(rename = "deviceClass")]
    DeviceClass,
    #[serde(rename = "ageRange")]
    AgeRange,
    #[serde(rename = "gender")]
    Gender,
    #[serde(rename = "countryCode")]
    CountryCode,
    #[serde(rename = "adminArea")]
    AdminArea,
    #[serde(rename = "locality")]
    Locality,
    #[serde(rename = "countryOrRegion")]
    CountryOrRegion,
}

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
pub enum ReportingRequestTimeZone {
    #[allow(clippy::upper_case_acronyms)]
    UTC,
    #[allow(clippy::upper_case_acronyms)]
    ORTZ,
}

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
pub enum ReportingRequestGranularity {
    #[allow(clippy::upper_case_acronyms)]
    MONTHLY,
    #[allow(clippy::upper_case_acronyms)]
    WEEKLY,
    #[allow(clippy::upper_case_acronyms)]
    DAILY,
    #[allow(clippy::upper_case_acronyms)]
    HOURLY,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs};

    use serde_json::Value;

    use crate::v3::objects::{
        condition::{Condition, ConditionOperator::*},
        pagination::Pagination,
        reporting_request::{
            ReportingRequestGranularity::*, ReportingRequestGroupBy::*, ReportingRequestTimeZone::*,
        },
        sorting::{Sorting, SortingSortOrder::*},
    };

    #[test]
    fn test_se_get_campaign_level_reports() -> Result<(), Box<dyn error::Error>> {
        let mut pagination = Pagination::new();
        pagination.set_limit(1000).set_offset(0);

        let mut selector = Selector::new(vec![Sorting::new("countryOrRegion", ASCENDING)]);
        selector
            .set_conditions(vec![
                Condition::new("countriesOrRegions", CONTAINS_ANY, vec!["US", "GB"]),
                Condition::new("countryOrRegion", IN, vec!["US"]),
            ])
            .set_pagination(pagination);

        let mut reporting_request = ReportingRequest::new(
            "2020-08-04".parse().unwrap(),
            "2020-08-14".parse().unwrap(),
            selector,
        );
        reporting_request
            .set_group_by(vec![CountryOrRegion])
            .set_time_zone(UTC)
            .set_return_records_with_no_metrics(true)
            .set_return_row_totals(true)
            .set_return_grand_totals(true);

        let value1: Value = serde_json::to_value(reporting_request)?;

        let json_content =
            fs::read_to_string("tests/v3/request_body_json_files/get_campaign_level_reports.json")
                .unwrap();
        let value2: Value = serde_json::from_str(&json_content)?;

        assert_eq!(value1, value2);

        Ok(())
    }

    #[test]
    fn test_se_get_campaign_level_reports_with_granularity() -> Result<(), Box<dyn error::Error>> {
        let mut pagination = Pagination::new();
        pagination.set_limit(1000).set_offset(0);

        let mut selector = Selector::new(vec![Sorting::new("countryOrRegion", ASCENDING)]);
        selector
            .set_conditions(vec![
                Condition::new("countriesOrRegions", CONTAINS_ANY, vec!["US", "GB"]),
                Condition::new("countryOrRegion", IN, vec!["US"]),
            ])
            .set_pagination(pagination);

        let mut reporting_request = ReportingRequest::new(
            "2020-08-04".parse().unwrap(),
            "2020-08-14".parse().unwrap(),
            selector,
        );
        reporting_request
            .set_group_by(vec![CountryOrRegion])
            .set_time_zone(UTC)
            .set_return_records_with_no_metrics(true)
            .set_return_row_totals(false)
            .set_granularity(DAILY)
            .set_return_grand_totals(false);

        let value1: Value = serde_json::to_value(reporting_request)?;

        let json_content = fs::read_to_string(
            "tests/v3/request_body_json_files/get_campaign_level_reports_with_granularity.json",
        )
        .unwrap();
        let value2: Value = serde_json::from_str(&json_content)?;

        assert_eq!(value1, value2);

        Ok(())
    }
}
