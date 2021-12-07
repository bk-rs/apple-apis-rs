pub mod verify_receipt;

//
//
//
pub(crate) mod endpoint_prelude {
    pub(crate) use http_api_client_endpoint::{
        http, Body, Request, Response, RetryableEndpoint, RetryableEndpointRetry,
    };
}
