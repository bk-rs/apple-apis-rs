// https://developer.apple.com/documentation/apple_search_ads/locinvoicedetails

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct LOCInvoiceDetails {
    #[serde(rename = "billingContactEmail")]
    pub billing_contact_email: String,

    #[serde(rename = "buyerEmail")]
    pub buyer_email: String,

    #[serde(rename = "buyerName")]
    pub buyer_name: String,

    #[serde(rename = "clientName")]
    pub client_mame: String,

    #[serde(rename = "orderNumber")]
    pub order_number: String,
}
