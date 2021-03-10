## Examples

* [in-app purchase - verify receipt](demos/isahc/src/iap_verify_receipt.rs)
* [search_ads get all campaigns](demos/isahc/src/search_ads_get_all_campaigns.rs)
* [search_ads get reports with granularity](demos/isahc/src/search_ads_get_reports_with_granularity.rs)
* [search_ads get user acl](demos/isahc/src/search_ads_get_user_acl.rs)

## Dev

```
cargo clippy --all --all-features -- -D clippy::all
cargo +nightly clippy --all --all-features -- -D clippy::all

cargo fmt --all -- --check
```
