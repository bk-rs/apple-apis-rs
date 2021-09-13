// https://developer.apple.com/documentation/apple_search_ads/campaign countriesOrRegions

use std::fmt;

use serde_enum_str::Deserialize_enum_str;

#[derive(Deserialize_enum_str, PartialEq, Eq, Hash, Debug, Clone)]
pub enum Region {
    #[allow(clippy::upper_case_acronyms)]
    US,
    #[serde(other)]
    Other(String),
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::US => write!(f, "US"),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
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
