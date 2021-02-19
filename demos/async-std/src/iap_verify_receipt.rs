/*
cargo run -p appleapis-demo-async-std --bin iap_verify_receipt 'YOUR_APPLE_IAP_PASSWORD' 'RECEIPT_BASE64_STRING'
*/

use std::{env, error};

use apple_app_store_receipts::{ReceiptData, VerifyReceipt};
use apple_web_service_isahc_client::{Client, IsahcClient};

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let apple_iap_password = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("APPLE_IAP_PASSWORD").unwrap());
    let receipt_base64_string = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("RECEIPT_BASE64_STRING").unwrap());

    println!("iap_verify_receipt");

    //
    let mut verify_receipt = VerifyReceipt::new(
        apple_iap_password,
        ReceiptData::Base64String(receipt_base64_string),
        None,
    );

    let isahc_client = IsahcClient::new()?;

    let response_body = isahc_client
        .respond_endpoint_until_done(&mut verify_receipt, None)
        .await?;

    println!("{:?}", response_body);

    println!("done");

    Ok(())
}
