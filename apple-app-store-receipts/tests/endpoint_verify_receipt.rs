use std::error;

use apple_app_store_receipts::{
    endpoints::verify_receipt::{ReceiptData, RetryReason, VerifyReceipt, VerifyReceiptError},
    objects::response_body::ResponseBody,
    types::status::Status,
};
use http_api_client_endpoint::{
    http::{Method, StatusCode, Version},
    Response, RetryableEndpoint, RetryableEndpointRetry,
};

#[test]
fn render_request() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let req = verify_receipt.render_request(None)?;

    assert_eq!(req.method(), Method::POST);
    assert_eq!(req.uri(), "https://buy.itunes.apple.com/verifyReceipt");
    assert_eq!(req.version(), Version::HTTP_11);

    let headers = req.headers();

    assert_eq!(
        headers.get("User-Agent").map(|x| x.to_str().unwrap()),
        Some("curl/7.80.0")
    );
    assert_eq!(
        headers.get("Content-Type").map(|x| x.to_str().unwrap()),
        Some("application/json")
    );
    assert_eq!(
        headers.get("Accept").map(|x| x.to_str().unwrap()),
        Some("application/json")
    );

    assert_eq!(
        req.body(),
        &br#"{"receipt-data":"foo","password":"pw"}"#.to_vec()
    );

    Ok(())
}

#[test]
fn parse_response_with_http_503() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .body(b"HTML".to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::ServiceUnavailable)
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn parse_response_with_http_400() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(b"HTML".to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Err(VerifyReceiptError::StatusMismatch(status)) => {
            assert_eq!(status, StatusCode::BAD_REQUEST);
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn parse_response_with_21007() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21007}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::GotoSandbox)
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
#[should_panic(expected = "double goto sandbox")]
fn parse_response_with_double_21007() {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21007}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::GotoSandbox)
        }
        _ => panic!(),
    }

    // again
    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21007}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(
        res,
        Some(&RetryableEndpointRetry::new(1, RetryReason::GotoSandbox)),
    ) {
        _ => panic!(),
    }
}

#[test]
fn parse_response_with_21002() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21002}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::TryAgainWithStatus(Status::Error21002))
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn parse_response_with_many_times_21002() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21002}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::TryAgainWithStatus(Status::Error21002))
        }
        _ => panic!(),
    }

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21002}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(
        res,
        Some(&RetryableEndpointRetry::new(
            1,
            RetryReason::TryAgainWithStatus(Status::Error21002),
        )),
    ) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::TryAgainWithStatus(Status::Error21002))
        }
        _ => panic!(),
    }

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21002}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(
        res,
        Some(&RetryableEndpointRetry::new(
            2,
            RetryReason::TryAgainWithStatus(Status::Error21002),
        )),
    ) {
        Ok(Err(reason)) => {
            assert_eq!(reason, RetryReason::TryAgainWithStatus(Status::Error21002))
        }
        _ => panic!(),
    }

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21002}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(
        res,
        Some(&RetryableEndpointRetry::new(
            3,
            RetryReason::TryAgainWithStatus(Status::Error21002),
        )),
    ) {
        Ok(Ok(ResponseBody::Error(body))) => {
            assert_eq!(body.status, Status::Error21002)
        }
        _ => {
            panic!()
        }
    }

    Ok(())
}

#[test]
fn parse_response_with_21104_and_retryable() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21104,"is_retryable":true}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Err(reason)) => assert_eq!(
            reason,
            RetryReason::TryAgainWithStatus(Status::InternalDataAccessError(21104))
        ),
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn parse_response_with_21104_and_not_retryable() -> Result<(), Box<dyn error::Error>> {
    let verify_receipt = VerifyReceipt::new(
        "pw".to_owned(),
        ReceiptData::Base64String("foo".to_owned()),
        None,
    );

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(br#"{"status":21104,"is_retryable":false}"#.to_vec())
        .unwrap();
    match verify_receipt.parse_response(res, None) {
        Ok(Ok(ResponseBody::Error(body))) => {
            assert_eq!(body.status, Status::InternalDataAccessError(21104))
        }
        _ => panic!(),
    }

    Ok(())
}
