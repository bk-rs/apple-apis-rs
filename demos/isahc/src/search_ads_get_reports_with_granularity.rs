/*
cargo run -p appleapis-demo-isahc --bin search_ads_get_reports_with_granularity campaign MONTHLY demo_certs/search_ads.pem demo_certs/search_ads.key ORG_ID 2020-01-01 2020-04-01

cargo run -p appleapis-demo-isahc --bin search_ads_get_reports_with_granularity campaign WEEKLY demo_certs/search_ads.pem demo_certs/search_ads.key ORG_ID 2020-01-01 2020-02-01

cargo run -p appleapis-demo-isahc --bin search_ads_get_reports_with_granularity ad_group DAILY demo_certs/search_ads.pem demo_certs/search_ads.key ORG_ID 2020-01-01 2020-02-01 CAMPAIGN_ID
cargo run -p appleapis-demo-isahc --bin search_ads_get_reports_with_granularity keyword HOURLY demo_certs/search_ads.pem demo_certs/search_ads.key ORG_ID 2020-01-31 2020-02-01 CAMPAIGN_ID
cargo run -p appleapis-demo-isahc --bin search_ads_get_reports_with_granularity search_term DAILY demo_certs/search_ads.pem demo_certs/search_ads.key ORG_ID 2020-01-31 2020-02-01 CAMPAIGN_ID
*/

use std::{env, error};

use apple_search_ads::v3::{
    endpoints::{
        get_ad_group_level_reports::GetAdGroupLevelReports,
        get_campaign_level_reports::GetCampaignLevelReports,
        get_keyword_level_reports::GetKeywordLevelReports,
        get_search_term_level_reports::GetSearchTermLevelReports,
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
use apple_web_service_isahc_client::{
    isahc::{
        config::{ClientCertificate, Configurable, PrivateKey},
        HttpClient,
    },
    Client as _, IsahcClient,
};
use chrono::NaiveDate;
use futures_lite::future::block_on;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let level = env::args().nth(1).unwrap();
    let granularity = env::args().nth(2).unwrap();
    let cert_pem_file_path = env::args().nth(3).unwrap();
    let private_key_pem_file_path = env::args().nth(4).unwrap();
    let org_id: u64 = env::args().nth(5).unwrap().parse().unwrap();
    let start_time: NaiveDate = env::args().nth(6).unwrap().parse().unwrap();
    let end_time: NaiveDate = env::args().nth(7).unwrap().parse().unwrap();
    let campaign_id: Option<u64> = env::args()
        .nth(8)
        .map(|campaign_id| campaign_id.parse().unwrap());

    let granularity = match granularity.as_str() {
        "HOURLY" => ReportingRequestGranularity::HOURLY,
        "DAILY" => ReportingRequestGranularity::DAILY,
        "WEEKLY" => ReportingRequestGranularity::WEEKLY,
        "MONTHLY" => ReportingRequestGranularity::MONTHLY,
        _ => return Err("Unknown granularity".into()),
    };

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

    let http_client = HttpClient::builder()
        .ssl_client_certificate(ClientCertificate::pem_file(
            cert_pem_file_path,
            PrivateKey::pem_file(private_key_pem_file_path, None),
        ))
        .build()?;
    let isahc_client = IsahcClient::with(http_client);

    match level.as_str() {
        "campaign" => {
            let mut get_campaign_level_reports =
                GetCampaignLevelReports::new(org_id, reporting_request);

            let (response_body, response_status) = isahc_client
                .respond_endpoint_until_done(&mut get_campaign_level_reports, None)
                .await?;

            println!("{:?}", response_body);
            println!("{:?}", response_status);
        }
        "ad_group" => {
            let mut get_ad_group_level_reports =
                GetAdGroupLevelReports::new(org_id, campaign_id.unwrap(), reporting_request);

            let (response_body, response_status) = isahc_client
                .respond_endpoint_until_done(&mut get_ad_group_level_reports, None)
                .await?;

            println!("{:?}", response_body);
            println!("{:?}", response_status);
        }
        "keyword" => {
            let mut get_keyword_level_reports =
                GetKeywordLevelReports::new(org_id, campaign_id.unwrap(), reporting_request);

            let (response_body, response_status) = isahc_client
                .respond_endpoint_until_done(&mut get_keyword_level_reports, None)
                .await?;

            println!("{:?}", response_body);
            println!("{:?}", response_status);
        }
        "search_term" => {
            let mut get_search_term_level_reports =
                GetSearchTermLevelReports::new(org_id, campaign_id.unwrap(), reporting_request);

            let (response_body, response_status) = isahc_client
                .respond_endpoint_until_done(&mut get_search_term_level_reports, None)
                .await?;

            println!("{:?}", response_body);
            println!("{:?}", response_status);
        }
        _ => return Err("Unknown level".into()),
    }

    Ok(())
}
