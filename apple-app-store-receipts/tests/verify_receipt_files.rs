use std::{error, fs, path::PathBuf};

use apple_app_store_receipts::{
    endpoints::verify_receipt::{ReceiptData, VerifyReceipt},
    objects::response_body::ResponseBody,
};
use futures_lite::future::block_on;
use http_api_isahc_client::{
    http_api_client::RetryableClientRespondEndpointUntilDoneError,
    isahc::error::ErrorKind as IsahcErrorKind, IsahcClient, RetryableClient as _,
};

#[test]
fn respond_all() -> Result<(), Box<dyn error::Error>> {
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

                let verify_receipt = VerifyReceipt::new(
                    password.to_owned(),
                    ReceiptData::Base64String(content),
                    None,
                );

                let isahc_client = IsahcClient::new()?;

                match isahc_client
                    .respond_endpoint_until_done(&verify_receipt)
                    .await
                {
                    Ok(response_body) => match response_body {
                        ResponseBody::Success(_) => {
                            println!("path {path:?} respond successful");
                        }
                        ResponseBody::Error(body) => {
                            println!("path {path:?} respond successful, body: {body:?}");
                        }
                    },
                    Err(err) => {
                        eprintln!("path {path:?} respond failed, err: {err:?}");
                        match err {
                            RetryableClientRespondEndpointUntilDoneError::RespondFailed(
                                ref isahc_err,
                            ) => match isahc_err.kind() {
                                IsahcErrorKind::Timeout => {}
                                _ => return Err(err.into()),
                            },
                            err => return Err(err.into()),
                        }
                    }
                }
            }
        }

        Ok(())
    })
}
