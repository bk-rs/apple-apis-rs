## Examples

* [in-app purchase - verify receipt](apple-app-store-receipts/demo/src/iap_verify_receipt.rs)
* [search ads - get all campaigns](apple-search-ads/demo/src/search_ads_get_all_campaigns.rs)
* [search ads - get reports with granularity](apple-search-ads/demo/src/search_ads_get_reports_with_granularity.rs)
* [search ads - get user acl](apple-search-ads/demo/src/search_ads_get_user_acl.rs)

## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```
