/*
cargo run -p apple-search-ads-demo --bin search_ads_get_all_campaigns -- demo_certs/search_ads.pem demo_certs/search_ads.key ORG_ID
*/

use std::{env, error};

use apple_search_ads::v3::{
    endpoints::get_all_campaigns::GetAllCampaigns, objects::pagination::Pagination,
};
use futures_lite::future::block_on;
use http_api_isahc_client::{
    isahc::{
        config::{ClientCertificate, Configurable, PrivateKey},
        HttpClient,
    },
    Client as _, IsahcClient,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let cert_pem_file_path = env::args().nth(1).unwrap();
    let private_key_pem_file_path = env::args().nth(2).unwrap();
    let org_id: u64 = env::args().nth(3).unwrap().parse().unwrap();

    let mut pagination = Pagination::new();
    pagination.set_limit(1000);
    let mut get_all_campaigns = GetAllCampaigns::new(org_id);
    get_all_campaigns.set_pagination(pagination);

    let http_client = HttpClient::builder()
        .ssl_client_certificate(ClientCertificate::pem_file(
            cert_pem_file_path,
            PrivateKey::pem_file(private_key_pem_file_path, None),
        ))
        .build()?;
    let isahc_client = IsahcClient::with(http_client);

    let (response_body, response_status) =
        isahc_client.respond_endpoint(&get_all_campaigns).await?;

    println!("{:?}", response_body);
    println!("{:?}", response_status);

    Ok(())
}
