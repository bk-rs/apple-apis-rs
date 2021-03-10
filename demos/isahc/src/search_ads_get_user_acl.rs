/*
cargo run -p appleapis-demo-isahc --bin search_ads_get_user_acl demo_certs/search_ads.pem demo_certs/search_ads.key
*/

use std::{env, error};

use apple_search_ads::v3::endpoints::get_user_acl::GetUserAcl;
use apple_web_service_isahc_client::{
    isahc::{
        config::{ClientCertificate, Configurable, PrivateKey},
        HttpClient,
    },
    Client as _, IsahcClient,
};
use futures_lite::future::block_on;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let cert_pem_file_path = env::args().nth(1).unwrap();
    let private_key_pem_file_path = env::args().nth(2).unwrap();

    let mut get_user_acl = GetUserAcl::new();

    let http_client = HttpClient::builder()
        .ssl_client_certificate(ClientCertificate::pem_file(
            cert_pem_file_path,
            PrivateKey::pem_file(private_key_pem_file_path, None),
        ))
        .build()?;
    let isahc_client = IsahcClient::with(http_client);

    let (response_body, response_status) = isahc_client
        .respond_endpoint_until_done(&mut get_user_acl, None)
        .await?;

    println!("{:?}", response_body);
    println!("{:?}", response_status);

    Ok(())
}
