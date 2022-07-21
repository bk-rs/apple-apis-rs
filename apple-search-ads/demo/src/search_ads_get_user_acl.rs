/*
cargo run -p apple-search-ads-demo --bin search_ads_get_user_acl -- 'YOUR_ACCESS_TOKEN'
*/

use std::{env, error};

use apple_search_ads::endpoints::{EndpointRet, GetUserAcl};
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_token = env::args().nth(1).unwrap();

    let client = IsahcClient::new()?;

    let get_user_acl = GetUserAcl::new(access_token);

    let ret = client.respond_endpoint(&get_user_acl).await?;

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
