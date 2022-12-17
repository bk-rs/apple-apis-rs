/*
cargo run -p apple-search-ads-demo --bin search_ads_get_reports_with_granularity -- campaign MONTHLY 'YOUR_ACCESS_TOKEN' ORG_ID 2020-01-01 2020-04-01

cargo run -p apple-search-ads-demo --bin search_ads_get_reports_with_granularity -- campaign WEEKLY 'YOUR_ACCESS_TOKEN' ORG_ID 2020-01-01 2020-02-01

cargo run -p apple-search-ads-demo --bin search_ads_get_reports_with_granularity -- ad_group DAILY 'YOUR_ACCESS_TOKEN' ORG_ID 2020-01-01 2020-02-01 CAMPAIGN_ID
cargo run -p apple-search-ads-demo --bin search_ads_get_reports_with_granularity -- keyword HOURLY 'YOUR_ACCESS_TOKEN' ORG_ID 2020-01-31 2020-02-01 CAMPAIGN_ID
cargo run -p apple-search-ads-demo --bin search_ads_get_reports_with_granularity -- search_term DAILY 'YOUR_ACCESS_TOKEN' ORG_ID 2020-01-31 2020-02-01 CAMPAIGN_ID
*/

use std::{env, error};

use apple_search_ads::{
    endpoints::{
        EndpointRet, GetAdGroupLevelReports, GetCampaignLevelReports, GetKeywordLevelReports,
        GetSearchTermLevelReports,
    },
    objects::{
        pagination::Pagination,
        reporting_request::{
            ReportingRequest, ReportingRequestGranularity, ReportingRequestGroupBy,
            ReportingRequestTimeZone,
        },
        selector::Selector,
        sorting::{Sorting, SortingSortOrder},
    },
};
use chrono::NaiveDate;
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let level = env::args().nth(1).unwrap();
    let granularity = env::args().nth(2).unwrap();
    let access_token = env::args().nth(3).unwrap();
    let org_id: u64 = env::args().nth(4).unwrap().parse().unwrap();
    let start_time: NaiveDate = env::args().nth(5).unwrap().parse().unwrap();
    let end_time: NaiveDate = env::args().nth(6).unwrap().parse().unwrap();
    let campaign_id: Option<u64> = env::args()
        .nth(7)
        .map(|campaign_id| campaign_id.parse().unwrap());

    let granularity = match granularity.as_str() {
        "HOURLY" => ReportingRequestGranularity::HOURLY,
        "DAILY" => ReportingRequestGranularity::DAILY,
        "WEEKLY" => ReportingRequestGranularity::WEEKLY,
        "MONTHLY" => ReportingRequestGranularity::MONTHLY,
        _ => return Err("Unknown granularity".into()),
    };

    let client = IsahcClient::new()?;

    let mut pagination = Pagination::new();
    pagination.set_limit(1000);
    let mut selector = Selector::new(vec![Sorting::new(
        "localSpend",
        SortingSortOrder::DESCENDING,
    )]);
    selector.set_pagination(pagination);
    let mut reporting_request = ReportingRequest::new(start_time, end_time, selector);
    reporting_request
        .set_granularity(granularity)
        .set_group_by(vec![ReportingRequestGroupBy::CountryOrRegion])
        .set_time_zone(ReportingRequestTimeZone::UTC)
        .set_return_records_with_no_metrics(false);

    match level.as_str() {
        "campaign" => {
            let get_campaign_level_reports =
                GetCampaignLevelReports::new(org_id, reporting_request, access_token);

            let ret = client.respond_endpoint(&get_campaign_level_reports).await?;

            match &ret {
                EndpointRet::Ok(ok_json) => {
                    println!("{ok_json:?}");
                }
                EndpointRet::Other(_) => {
                    println!("{ret:?}");
                }
            }
        }
        "ad_group" => {
            let get_ad_group_level_reports = GetAdGroupLevelReports::new(
                org_id,
                campaign_id.unwrap(),
                reporting_request,
                access_token,
            );

            let ret = client.respond_endpoint(&get_ad_group_level_reports).await?;

            match &ret {
                EndpointRet::Ok(ok_json) => {
                    println!("{ok_json:?}");
                }
                EndpointRet::Other(_) => {
                    println!("{ret:?}");
                }
            }
        }
        "keyword" => {
            let get_keyword_level_reports = GetKeywordLevelReports::new(
                org_id,
                campaign_id.unwrap(),
                reporting_request,
                access_token,
            );

            let ret = client.respond_endpoint(&get_keyword_level_reports).await?;

            match &ret {
                EndpointRet::Ok(ok_json) => {
                    println!("{ok_json:?}");
                }
                EndpointRet::Other(_) => {
                    println!("{ret:?}");
                }
            }
        }
        "search_term" => {
            let get_search_term_level_reports = GetSearchTermLevelReports::new(
                org_id,
                campaign_id.unwrap(),
                reporting_request,
                access_token,
            );

            let ret = client
                .respond_endpoint(&get_search_term_level_reports)
                .await?;

            match &ret {
                EndpointRet::Ok(ok_json) => {
                    println!("{ok_json:?}");
                }
                EndpointRet::Other(_) => {
                    println!("{ret:?}");
                }
            }
        }
        _ => return Err("Unknown level".into()),
    }

    Ok(())
}
