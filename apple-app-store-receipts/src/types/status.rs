// ref https://developer.apple.com/documentation/appstorereceipts/status

use std::fmt;

use serde::{de, Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Success,
    Error21000,
    Error21001,
    Error21002,
    Error21003,
    Error21004,
    Error21005,
    Error21006,
    Error21007,
    Error21008,
    Error21009,
    Error21010,
    InternalDataAccessError(u16),
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u16::deserialize(deserializer)?;
        match v {
            0 => Ok(Self::Success),
            21000 => Ok(Self::Error21000),
            21001 => Ok(Self::Error21001),
            21002 => Ok(Self::Error21002),
            21003 => Ok(Self::Error21003),
            21004 => Ok(Self::Error21004),
            21005 => Ok(Self::Error21005),
            21006 => Ok(Self::Error21006),
            21007 => Ok(Self::Error21007),
            21008 => Ok(Self::Error21008),
            21009 => Ok(Self::Error21009),
            21010 => Ok(Self::Error21010),
            21100..=21199 => Ok(Self::InternalDataAccessError(v)),
            _ => Err(de::Error::custom(format!("unknown status value [{}]", v))),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "[0] the receipt is valid"),
            Self::Error21000 => write!(
                f,
                "[21000] The request to the App Store was not made using the HTTP POST request method."
            ),
            Self::Error21001 => write!(f, "[21001] This status code is no longer sent by the App Store."),
            Self::Error21002 => write!(f, "[21002] The data in the receipt-data property was malformed or the service experienced a temporary issue. Try again."),
            Self::Error21003 => write!(f, "[21003] The receipt could not be authenticated."),
            Self::Error21004 => write!(f, "[21004] The shared secret you provided does not match the shared secret on file for your account."),
            Self::Error21005 => write!(f, "[21005] The receipt server was temporarily unable to provide the receipt. Try again."),
            Self::Error21006 => write!(f, "[21006] This receipt is valid but the subscription has expired."),
            Self::Error21007 => write!(f, "[21007] This receipt is from the test environment, but it was sent to the production environment for verification."),
            Self::Error21008 => write!(f, "[21008] This receipt is from the production environment, but it was sent to the test environment for verification."),
            Self::Error21009 => write!(f, "[21009] Internal data access error. Try again later."),
            Self::Error21010 => write!(f, "[21010] The user account cannot be found or has been deleted."),
            Self::InternalDataAccessError(v) => write!(f, "[{}] internal data access errors.", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Foo {
        status: Status,
    }

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        for (v, status) in vec![
            (0, Status::Success),
            (21000, Status::Error21000),
            (21001, Status::Error21001),
            (21002, Status::Error21002),
            (21003, Status::Error21003),
            (21004, Status::Error21004),
            (21005, Status::Error21005),
            (21006, Status::Error21006),
            (21007, Status::Error21007),
            (21008, Status::Error21008),
            (21009, Status::Error21009),
            (21010, Status::Error21010),
            (21100, Status::InternalDataAccessError(21100)),
            (21101, Status::InternalDataAccessError(21101)),
            (21199, Status::InternalDataAccessError(21199)),
        ] {
            assert_de_tokens(
                &Foo { status },
                &[
                    Token::Struct {
                        name: "Foo",
                        len: 1,
                    },
                    Token::Str("status"),
                    Token::U16(v),
                    Token::StructEnd,
                ],
            );
        }

        assert_de_tokens_error::<Foo>(
            &[
                Token::Struct {
                    name: "Foo",
                    len: 1,
                },
                Token::Str("status"),
                Token::U16(1),
                Token::StructEnd,
            ],
            "unknown status value [1]",
        );

        Ok(())
    }
}
