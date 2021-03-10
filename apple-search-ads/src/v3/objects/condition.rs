// https://developer.apple.com/documentation/apple_search_ads/condition

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub field: String,

    pub operator: ConditionOperator,

    pub values: Vec<String>,
}
impl Condition {
    pub fn new(
        field: impl Into<String>,
        operator: ConditionOperator,
        values: Vec<impl Into<String>>,
    ) -> Self {
        Self {
            field: field.into(),
            operator,
            values: values.into_iter().map(Into::into).collect::<Vec<_>>(),
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
