use core::fmt;

use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body,
};
use serde_json::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

use crate::objects::api_error_response::ApiErrorResponse;

//
#[derive(Debug, Clone)]
pub enum EndpointRet<T> {
    Ok(T),
    Other((StatusCode, Result<ApiErrorResponse, Body>)),
}

//
#[derive(Debug)]
pub enum EndpointError {
    MakeRequestUrlFailed(UrlParseError),
    SerRequestBodyJsonFailed(SerdeJsonError),
    MakeRequestFailed(HttpError),
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl fmt::Display for EndpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for EndpointError {}
