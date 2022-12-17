/*
cargo run -p apple-search-ads-demo --bin search_ads_get_all_campaigns -- 'YOUR_ACCESS_TOKEN' ORG_ID
*/

use std::{env, error};

use apple_search_ads::{
    endpoints::{EndpointRet, GetAllCampaigns},
    objects::Pagination,
};
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_token = env::args().nth(1).unwrap();
    let org_id: u64 = env::args().nth(2).unwrap().parse().unwrap();

    let client = IsahcClient::new()?;

    let mut pagination = Pagination::new();
    pagination.set_limit(1000);
    let mut get_all_campaigns = GetAllCampaigns::new(org_id, access_token);
    get_all_campaigns.set_pagination(pagination);

    let ret = client.respond_endpoint(&get_all_campaigns).await?;

    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{ok_json:?}");
        }
        EndpointRet::Other(_) => {
            println!("{ret:?}");
        }
    }

    Ok(())
}
