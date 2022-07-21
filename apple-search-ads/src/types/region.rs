// https://developer.apple.com/documentation/apple_search_ads/campaign countriesOrRegions

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    #[allow(clippy::upper_case_acronyms)]
    US,
    #[serde(other)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(Region::US.to_string(), "US");
        assert_eq!(Region::Other("CN".to_owned()).to_string(), "CN");
    }
}
