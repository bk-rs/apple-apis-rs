// https://developer.apple.com/documentation/apple_search_ads/reportingrequest

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_aux_ext::field_attributes::deserialize_option_bool_from_anything;

use crate::objects::selector::Selector;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
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

    #[serde(
        default,
        deserialize_with = "deserialize_option_bool_from_anything",
        skip_serializing_if = "Option::is_none",
        rename = "returnGrandTotals"
    )]
    pub return_grand_totals: Option<bool>,

    #[serde(
        default,
        deserialize_with = "deserialize_option_bool_from_anything",
        skip_serializing_if = "Option::is_none",
        rename = "returnRecordsWithNoMetrics"
    )]
    pub return_records_with_no_metrics: Option<bool>,

    #[serde(
        default,
        deserialize_with = "deserialize_option_bool_from_anything",
        skip_serializing_if = "Option::is_none",
        rename = "returnRowTotals"
    )]
    pub return_row_totals: Option<bool>,

    pub selector: Selector,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeZone")]
    pub time_zone: Option<ReportingRequestTimeZone>,
}

impl Default for ReportingRequest {
    fn default() -> Self {
        let now = Utc::today();
        Self::new(
            (now - chrono::Duration::days(3)).naive_utc(),
            now.naive_utc(),
            Selector::default(),
        )
    }
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ReportingRequestTimeZone {
    #[allow(clippy::upper_case_acronyms)]
    UTC,
    #[allow(clippy::upper_case_acronyms)]
    ORTZ,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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

    use std::error;

    use serde_json::Value;

    use crate::objects::{
        condition::{Condition, ConditionOperator::*},
        pagination::Pagination,
        reporting_request::{
            ReportingRequestGranularity::*, ReportingRequestGroupBy::*, ReportingRequestTimeZone::*,
        },
        sorting::{Sorting, SortingSortOrder::*},
    };

    #[test]
    fn test_v4_ser_get_campaign_level_reports() -> Result<(), Box<dyn error::Error>> {
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
            "2021-04-08".parse().unwrap(),
            "2021-04-09".parse().unwrap(),
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
            include_str!("../../tests/v4/request_body_json_files/get_campaign_level_reports_payload_example_1.json");
        let value2: Value = serde_json::from_str(json_content)?;

        assert_eq!(value1, value2);

        Ok(())
    }

    #[test]
    fn test_v4_ser_get_campaign_level_reports_with_granularity() -> Result<(), Box<dyn error::Error>>
    {
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
            "2021-04-08".parse().unwrap(),
            "2021-04-18".parse().unwrap(),
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

        let json_content = include_str!(
            "../../tests/v4/request_body_json_files/get_campaign_level_reports_payload_example_3.json",
        );
        let value2: Value = serde_json::from_str(json_content)?;

        assert_eq!(value1, value2);

        Ok(())
    }

    #[test]
    fn test_v3_ser_get_campaign_level_reports() -> Result<(), Box<dyn error::Error>> {
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
            include_str!("../../tests/v3/request_body_json_files/get_campaign_level_reports.json");
        let value2: Value = serde_json::from_str(json_content)?;

        assert_eq!(value1, value2);

        Ok(())
    }

    #[test]
    fn test_v3_ser_get_campaign_level_reports_with_granularity() -> Result<(), Box<dyn error::Error>>
    {
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

        let json_content = include_str!(
            "../../tests/v3/request_body_json_files/get_campaign_level_reports_with_granularity.json",
        );
        let value2: Value = serde_json::from_str(json_content)?;

        assert_eq!(value1, value2);

        Ok(())
    }

    #[test]
    fn test_v4_de() {
        match serde_json::from_str::<ReportingRequest>(include_str!("../../tests/v4/request_body_json_files/get_campaign_level_reports_payload_example_1.json")) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }

        match serde_json::from_str::<ReportingRequest>(include_str!("../../tests/v4/request_body_json_files/get_campaign_level_reports_payload_example_2.json")) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }

        match serde_json::from_str::<ReportingRequest>(include_str!("../../tests/v4/request_body_json_files/get_campaign_level_reports_payload_example_3.json")) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }
    }

    #[test]
    fn test_v3_de() {
        match serde_json::from_str::<ReportingRequest>(include_str!("../../tests/v3/request_body_json_files/get_ad_group_level_reports_using_group_by_field_with_geo_values.json")) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }

        match serde_json::from_str::<ReportingRequest>(include_str!(
            "../../tests/v3/request_body_json_files/get_ad_group_level_reports.json"
        )) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }

        match serde_json::from_str::<ReportingRequest>(include_str!("../../tests/v3/request_body_json_files/get_campaign_level_reports_with_granularity.json")) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }

        match serde_json::from_str::<ReportingRequest>(include_str!(
            "../../tests/v3/request_body_json_files/get_campaign_level_reports.json"
        )) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }

        match serde_json::from_str::<ReportingRequest>(include_str!(
            "../../tests/v3/request_body_json_files/get_keyword_level_reports.json"
        )) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }

        match serde_json::from_str::<ReportingRequest>(include_str!(
            "../../tests/v3/request_body_json_files/get_search_term_level_reports.json"
        )) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }
    }
}
