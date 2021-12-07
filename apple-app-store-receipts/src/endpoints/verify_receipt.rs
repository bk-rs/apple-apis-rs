// ref https://developer.apple.com/documentation/appstorereceipts/verifyreceipt

use http::{
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
    Error as HttpError, Method, StatusCode, Version,
};
use serde_json::Error as SerdeJsonError;

use crate::{
    objects::{request_body::RequestBody, response_body::ResponseBody},
    types::status::Status,
};

use super::endpoint_prelude::*;

const URL_PRODUCTION: &str = "https://buy.itunes.apple.com/verifyReceipt";
const URL_SANDBOX: &str = "https://sandbox.itunes.apple.com/verifyReceipt";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryReason {
    ServiceUnavailable,
    GotoSandbox,
    TryAgainWithStatus(Status),
}

pub struct VerifyReceipt {
    password: String,
    receipt_data: ReceiptData,
    exclude_old_transactions: Option<bool>,
}
impl VerifyReceipt {
    pub fn new(
        password: String,
        receipt_data: ReceiptData,
        exclude_old_transactions: Option<bool>,
    ) -> Self {
        Self {
            password,
            receipt_data,
            exclude_old_transactions,
        }
    }
}

pub enum ReceiptData {
    #[cfg(feature = "with-base64")]
    Bytes(Vec<u8>),
    Base64String(String),
}
impl ReceiptData {
    pub fn data(&self) -> String {
        match self {
            #[cfg(feature = "with-base64")]
            Self::Bytes(vec) => base64::encode(vec),
            Self::Base64String(string) => string.to_owned(),
        }
    }
}

impl RetryableEndpoint for VerifyReceipt {
    type RetryReason = RetryReason;

    type RenderRequestError = VerifyReceiptError;

    type ParseResponseOutput = ResponseBody;
    type ParseResponseError = VerifyReceiptError;

    fn render_request(
        &self,
        retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = URL_PRODUCTION;
        if let Some(retry) = retry {
            match retry.reason {
                RetryReason::ServiceUnavailable => {}
                RetryReason::GotoSandbox => url = URL_SANDBOX,
                RetryReason::TryAgainWithStatus(_) => {}
            }
        }

        let receipt_data = self.receipt_data.data();

        let body = RequestBody::new(
            receipt_data.as_str(),
            &self.password,
            self.exclude_old_transactions,
        );

        let body_bytes =
            serde_json::to_vec(&body).map_err(VerifyReceiptError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .version(Version::HTTP_11)
            .header(USER_AGENT, "curl/7.80.0")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(body_bytes)
            .map_err(VerifyReceiptError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
        retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Result<Self::ParseResponseOutput, Self::RetryReason>, Self::ParseResponseError>
    {
        match response.status() {
            StatusCode::OK => {}
            /*
            503 <html><body><b>Http/1.1 Service Unavailable</b></body> </html>
            */
            StatusCode::SERVICE_UNAVAILABLE => {
                return Ok(Err(RetryReason::ServiceUnavailable));
            }
            status => {
                return Err(VerifyReceiptError::StatusMismatch(status));
            }
        }

        let body: ResponseBody = serde_json::from_slice(response.body())
            .map_err(VerifyReceiptError::DeResponseBodyFailed)?;

        match &body {
            ResponseBody::Success(_) => {}
            ResponseBody::Error(body) => match &body.status {
                Status::Error21007 => {
                    if let Some(retry) = retry {
                        match retry.reason {
                            RetryReason::ServiceUnavailable => {}
                            RetryReason::GotoSandbox => debug_assert!(false, "double goto sandbox"),
                            RetryReason::TryAgainWithStatus(_) => {}
                        }
                    }
                    return Ok(Err(RetryReason::GotoSandbox));
                }
                Status::Error21002 | Status::Error21005 | Status::Error21009 => {
                    if let Some(retry) = retry {
                        if retry.count < self.max_retry_count() {
                            return Ok(Err(RetryReason::TryAgainWithStatus(
                                body.status.to_owned(),
                            )));
                        }
                    } else {
                        return Ok(Err(RetryReason::TryAgainWithStatus(body.status.to_owned())));
                    }
                }
                Status::InternalDataAccessError(_) => {
                    if body.is_retryable == Some(true) {
                        if let Some(retry) = retry {
                            if retry.count < self.max_retry_count() {
                                return Ok(Err(RetryReason::TryAgainWithStatus(
                                    body.status.to_owned(),
                                )));
                            }
                        } else {
                            return Ok(Err(RetryReason::TryAgainWithStatus(
                                body.status.to_owned(),
                            )));
                        }
                    }
                }
                _ => {}
            },
        }

        Ok(Ok(body))
    }

    fn max_retry_count(&self) -> usize {
        3
    }
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum VerifyReceiptError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeJsonError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("StatusMismatch {0}")]
    StatusMismatch(StatusCode),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
