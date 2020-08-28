// ref https://developer.apple.com/documentation/appstorereceipts/verifyreceipt

use http::{
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
    Method, StatusCode, Version,
};
use std::io;

use crate::endpoints::endpoint_prelude::*;
use crate::{
    objects::{request_body::RequestBody, response_body::ResponseBody},
    types::status::Status,
};

const URL_PRODUCTION: &str = "https://buy.itunes.apple.com/verifyReceipt";
const URL_SANDBOX: &str = "https://sandbox.itunes.apple.com/verifyReceipt";

#[derive(Debug, PartialEq, Eq)]
pub enum RetryReason {
    ServiceUnavailable,
    GotoSandbox,
    TryAgainWithStatus(Status),
}

pub struct VerifyReceipt {
    password: String,
    receipt_data: ReceiptData,
    exclude_old_transactions: Option<bool>,
    //
    goto_sandbox: bool,
    retry_count: u8,
    max_retry_count: u8,
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
            goto_sandbox: false,
            retry_count: 0,
            max_retry_count: 3,
        }
    }
}

pub enum ReceiptData {
    Bytes(Vec<u8>),
    Base64String(String),
}
impl ReceiptData {
    pub fn data(&self) -> String {
        match self {
            Self::Bytes(vec) => base64::encode(vec),
            Self::Base64String(string) => string.to_owned(),
        }
    }
}

impl Endpoint for VerifyReceipt {
    type ParseResponseOutput = ResponseBody;
    type RetryReason = RetryReason;

    fn render_request(&self) -> io::Result<Request<Body>> {
        let url = if self.goto_sandbox {
            URL_SANDBOX
        } else {
            URL_PRODUCTION
        };
        let receipt_data = self.receipt_data.data();

        let body = RequestBody::new(
            receipt_data.as_str(),
            &self.password,
            self.exclude_old_transactions,
        );

        let body_bytes = serde_json::to_vec(&body)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .version(Version::HTTP_11)
            .header(USER_AGENT, "curl/7.72.0")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(body_bytes)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(request)
    }

    fn parse_response(
        &mut self,
        response: Response<Body>,
    ) -> io::Result<EndpointParseResponseOutput<Self::ParseResponseOutput, Self::RetryReason>> {
        match response.status() {
            StatusCode::OK => {}
            /*
            503 <html><body><b>Http/1.1 Service Unavailable</b></body> </html>
            */
            StatusCode::SERVICE_UNAVAILABLE => {
                return Ok(EndpointParseResponseOutput::Retryable(
                    RetryReason::ServiceUnavailable,
                ));
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("status [{}] mismatch", response.status()),
                ));
            }
        }

        let body: ResponseBody = serde_json::from_slice(response.body())?;

        match &body {
            ResponseBody::Success(_) => {}
            ResponseBody::Error(body) => match &body.status {
                Status::Error21007 => {
                    if self.goto_sandbox {
                        debug_assert!(false, "double goto sandbox");
                    } else {
                        self.goto_sandbox = true;
                        return Ok(EndpointParseResponseOutput::Retryable(
                            RetryReason::GotoSandbox,
                        ));
                    }
                }
                Status::Error21002 | Status::Error21005 | Status::Error21009 => {
                    self.retry_count += 1;
                    if self.retry_count <= self.max_retry_count {
                        return Ok(EndpointParseResponseOutput::Retryable(
                            RetryReason::TryAgainWithStatus(body.status.to_owned()),
                        ));
                    }
                }
                Status::InternalDataAccessError(_) => {
                    if body.is_retryable == Some(true) {
                        self.retry_count += 1;
                        if self.retry_count <= self.max_retry_count {
                            return Ok(EndpointParseResponseOutput::Retryable(
                                RetryReason::TryAgainWithStatus(body.status.to_owned()),
                            ));
                        }
                    }
                }
                _ => {}
            },
        }

        Ok(EndpointParseResponseOutput::Done(body))
    }
}
