# apple-app-store-receipts

* [Apple Doc](https://developer.apple.com/documentation/appstorereceipts)
* [Cargo package](https://crates.io/crates/apple-app-store-receipts)

## Examples

* [verify receipt](demo/src/iap_verify_receipt.rs)

## Dev

```
echo -n 'YOUR_APPLE_IAP_PASSWORD' > tests/verify_receipt_files/password

ls tests/verify_receipt_files/*.base64
```

```
ls tests/response_body_json_files/*.json

rg ':210' tests/response_body_json_files/

rg ':211' tests/response_body_json_files/
```
