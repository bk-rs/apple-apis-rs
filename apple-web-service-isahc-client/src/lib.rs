pub use isahc;

//
//
//
use std::{io, time::Duration};

pub use apple_web_service_client::Client;
use apple_web_service_client::{async_trait, Body, Request, Response};
use isahc::{config::Configurable, AsyncReadResponseExt, HttpClient};

pub struct IsahcClient {
    http_client: HttpClient,
}

impl IsahcClient {
    pub fn new() -> Result<Self, isahc::Error> {
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

        let resp = self.http_client.send_async(request).await?;
        let (head, body) = resp.into_parts();

        let mut body_buf = Vec::with_capacity(body.len().unwrap_or_else(|| {
            if req_uri.to_string().contains("verifyReceipt") {
                req_body_len as u64 * 2
            } else {
                4 * 1024
            }
        }) as usize);

        let mut resp = Response::from_parts(head, body);
        resp.copy_to(&mut body_buf).await?;

        let (head, _) = resp.into_parts();
        let resp = Response::from_parts(head, body_buf);

        Ok(resp)
    }
}
