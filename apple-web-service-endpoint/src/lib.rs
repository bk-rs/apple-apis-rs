pub use http;
pub use http::{Request, Response};

pub type Body = Vec<u8>;

//
//
//
use std::io;

pub trait Endpoint {
    type ParseResponseOutput;
    type RetryReason;

    fn render_request(&self) -> io::Result<Request<Body>>;

    fn parse_response(
        &mut self,
        response: Response<Body>,
    ) -> io::Result<EndpointParseResponseOutput<Self::ParseResponseOutput, Self::RetryReason>>;
}

pub enum EndpointParseResponseOutput<ParseResponseOutput, RetryReason> {
    Retryable(RetryReason),
    Done(ParseResponseOutput),
}
