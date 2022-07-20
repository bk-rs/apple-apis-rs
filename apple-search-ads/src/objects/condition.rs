// https://developer.apple.com/documentation/apple_search_ads/condition

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub field: Box<str>,

    pub operator: ConditionOperator,

    pub values: Vec<Box<str>>,
}
impl Condition {
    pub fn new(
        field: impl AsRef<str>,
        operator: ConditionOperator,
        values: Vec<impl AsRef<str>>,
    ) -> Self {
        Self {
            field: field.as_ref().into(),
            operator,
            values: values
                .into_iter()
                .map(|x| x.as_ref().into())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConditionOperator {
    #[allow(clippy::upper_case_acronyms)]
    EQUALS,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    GREATER_THAN,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    LESS_THAN,
    #[allow(clippy::upper_case_acronyms)]
    IN,
    #[allow(clippy::upper_case_acronyms)]
    LIKE,
    #[allow(clippy::upper_case_acronyms)]
    STARTSWITH,
    #[allow(clippy::upper_case_acronyms)]
    CONTAINS,
    #[allow(clippy::upper_case_acronyms)]
    ENDSWITH,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    NOT_EQUALS,
    #[allow(clippy::upper_case_acronyms)]
    IS,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    CONTAINS_ANY,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    CONTAINS_ALL,
}
