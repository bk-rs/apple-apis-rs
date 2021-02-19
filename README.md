## Examples

### async-std

* [in-app purchase - verify receipt](demos/async-std/src/iap_verify_receipt.rs)

## Dev

```
cargo clippy --all --all-features -- -D clippy::all
cargo +nightly clippy --all --all-features -- -D clippy::all

cargo fmt --all -- --check
```

```
cargo build-all-features

cargo test-all-features -- --nocapture
```
