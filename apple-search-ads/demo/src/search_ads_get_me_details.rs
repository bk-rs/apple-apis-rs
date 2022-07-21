/*
cargo run -p apple-search-ads-demo --bin search_ads_get_me_details -- 'YOUR_ACCESS_TOKEN'
*/

use std::{env, error};

use apple_search_ads::endpoints::{EndpointRet, GetMeDetails};
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_token = env::args().nth(1).unwrap();

    let client = IsahcClient::new()?;

    let get_me_details = GetMeDetails::new(access_token);

    let ret = client.respond_endpoint(&get_me_details).await?;

    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);
        }
        EndpointRet::Other(_) => {
            println!("{:?}", ret);
        }
    }

    Ok(())
}
