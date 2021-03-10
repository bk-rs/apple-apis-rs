// https://developer.apple.com/documentation/apple_search_ads/campaign countriesOrRegions

use std::{fmt, str::FromStr as _};

use serde::{Deserialize, Deserializer};
use strum::EnumString;

#[derive(EnumString, PartialEq, Eq, Hash, Debug, Clone)]
pub enum Region {
    #[allow(clippy::upper_case_acronyms)]
    US,
    #[strum(disabled)]
    Other(String),
}
impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        Ok(Self::from_str(str.as_ref()).unwrap_or_else(|_| Self::Other(str)))
    }
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
