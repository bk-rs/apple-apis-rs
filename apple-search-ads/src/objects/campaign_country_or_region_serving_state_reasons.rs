// https://developer.apple.com/documentation/apple_search_ads/campaign/countryorregionservingstatereasons

use std::collections::HashMap;

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::types::region::Region;

pub type CampaignCountryOrRegionServingStateReasons =
    HashMap<Region, Vec<CampaignCountryOrRegionServingStateReason>>;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum CampaignCountryOrRegionServingStateReason {
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_ELIGIBLE,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_ELIGIBLE_SEARCHADS,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_PUBLISHED_YET,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    SAPIN_LAW_AGENT_UNKNOWN,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    SAPIN_LAW_FRENCH_BIZ_UNKNOWN,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    SAPIN_LAW_FRENCH_BIZ,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_ELIGIBLE_SUPPLY_SOURCE,
    //
    #[serde(other)]
    Other(Box<str>),
}
