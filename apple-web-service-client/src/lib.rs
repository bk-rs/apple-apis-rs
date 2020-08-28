use std::io;

pub use apple_web_service_endpoint::{
    Body, Endpoint, EndpointParseResponseOutput, Request, Response,
};
pub use async_trait::async_trait;

#[async_trait]
pub trait Client {
    async fn respond(&self, request: Request<Body>) -> io::Result<Response<Body>>;

    async fn respond_endpoint<E: Endpoint + Send + Sync>(
        &self,
        endpoint: &mut E,
    ) -> io::Result<EndpointParseResponseOutput<E::ParseResponseOutput, E::RetryReason>> {
        let request = endpoint.render_request()?;
        let response = self.respond(request).await?;
        endpoint.parse_response(response)
    }

    async fn respond_endpoint_until_done<E: Endpoint + Send + Sync>(
        &self,
        endpoint: &mut E,
        max_retry_count: Option<u8>,
    ) -> io::Result<E::ParseResponseOutput> {
        let max_retry_count = max_retry_count.unwrap_or(3);
        let mut retry_count = 0;
        loop {
            match self.respond_endpoint(endpoint).await? {
                EndpointParseResponseOutput::Retryable(_) => {
                    retry_count += 1;
                    if retry_count > max_retry_count {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "reached max retry count",
                        ));
                    }
                    continue;
                }
                EndpointParseResponseOutput::Done(output) => {
                    break Ok(output);
                }
            }
        }
    }
}
