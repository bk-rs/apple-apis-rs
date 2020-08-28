use std::fs;
use std::io;
use std::path::PathBuf;

use apple_app_store_receipts::endpoints::verify_receipt::{ReceiptData, VerifyReceipt};
use apple_app_store_receipts::objects::response_body::ResponseBody;
use apple_web_service_isahc_client::{Client, IsahcClient};
use futures_lite::future::block_on;

#[test]
fn respond_all() -> io::Result<()> {
    block_on(async {
        let dir = PathBuf::new().join("tests/verify_receipt_files");

        if !dir.join("password").exists() {
            return Ok(());
        }

        let password = fs::read_to_string(dir.join("password"))?;

        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && Some(Some("base64")) == path.extension().map(|x| x.to_str()) {
                let content = fs::read_to_string(&path)?;

                let mut verify_receipt = VerifyReceipt::new(
                    password.to_owned(),
                    ReceiptData::Base64String(content),
                    None,
                );

                let isahc_client = IsahcClient::new()?;

                match isahc_client
                    .respond_endpoint_until_done(&mut verify_receipt, None)
                    .await
                {
                    Ok(response_body) => match response_body {
                        ResponseBody::Success(_) => {
                            println!("path {:?} respond successful", path);
                        }
                        ResponseBody::Error(body) => {
                            println!("path {:?} respond successful, body: {:?}", path, body);
                        }
                    },
                    Err(err) => {
                        eprintln!("path {:?} respond failed, err: {:?}", path, err);
                        match err.kind() {
                            io::ErrorKind::TimedOut => {}
                            _ => {
                                return Err(io::Error::new(io::ErrorKind::Other, err));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    })
}
