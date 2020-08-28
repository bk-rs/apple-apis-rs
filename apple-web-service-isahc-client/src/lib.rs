pub use isahc;

//
//
//
use std::io;
use std::result;
use std::time::Duration;

pub use apple_web_service_client::Client;
use apple_web_service_client::{async_trait, Body, Request, Response};
use futures_lite::io::AsyncReadExt;
use isahc::{config::Configurable, HttpClient};

pub struct IsahcClient {
    http_client: HttpClient,
}

impl IsahcClient {
    pub fn new() -> result::Result<Self, isahc::Error> {
        Ok(Self::with(
            HttpClient::builder()
                .timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(5))
                .build()?,
        ))
    }

    pub fn with(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl Client for IsahcClient {
    async fn respond(&self, request: Request<Body>) -> io::Result<Response<Body>> {
        let req_uri = request.uri().to_owned();
        let req_body_len = request.body().len();

        let res = self.http_client.send_async(request).await?;
        let (head, mut body) = res.into_parts();

        let mut body_buf = Vec::with_capacity(body.len().unwrap_or_else(|| {
            if req_uri.to_string().contains("verifyReceipt") {
                req_body_len as u64 * 2
            } else {
                4 * 1024
            }
        }) as usize);

        body.read_to_end(&mut body_buf).await?;

        let res = Response::from_parts(head, body_buf);
        Ok(res)
    }
}
