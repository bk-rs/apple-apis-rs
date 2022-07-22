// https://developer.apple.com/documentation/apple_search_ads/locinvoicedetails

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub struct LOCInvoiceDetails {
    #[serde(rename = "billingContactEmail")]
    pub billing_contact_email: Box<str>,

    #[serde(rename = "buyerEmail")]
    pub buyer_email: Box<str>,

    #[serde(rename = "buyerName")]
    pub buyer_name: Box<str>,

    #[serde(rename = "clientName")]
    pub client_mame: Box<str>,

    #[serde(rename = "orderNumber")]
    pub order_number: Box<str>,
}
