// ref https://developer.apple.com/documentation/appstorereceipts

pub mod endpoints;
pub mod objects;
pub mod types;

pub use endpoints::verify_receipt::{ReceiptData, VerifyReceipt};
