## Files

### salesReports__Subscription_Event_12345678_20230301_V1_3.txt.gz

```
curl -v -H 'Authorization: Bearer xxx' \                              
  'https://api.appstoreconnect.apple.com/v1/salesReports?filter[frequency]=DAILY&filter[reportDate]=2023-03-01&filter[reportSubType]=SUMMARY&filter[reportType]=SUBSCRIPTION_EVENT&filter[vendorNumber]=12345678&filter[version]=1_3' \
  -o /tmp/Subscription_Event_12345678_20230301_V1_3.txt.gz -g --http1.1
```

```
< HTTP/1.1 200 OK
< Server: daiquiri/3.0.0
< Date: Mon, 06 Mar 2023 07:38:54 GMT
< Content-Type: application/a-gzip
< Content-Length: 2025
< Connection: keep-alive
< requestId: 5c5ddedd-d4e4-4de6-a584-c6a4eb6c7ff6
< Content-Encoding: agzip
< Content-Disposition: attachment;filename=Subscription_Event_12345678_20230301_V1_3.txt.gz
< x-reports-filename: Subscription_Event_12345678_20230301_V1_3.txt.gz
< x-reports-download-version: 1_3
< Strict-Transport-Security: max-age=31536000; includeSubDomains
< X-Frame-Options: SAMEORIGIN
< Host: reportingitc-reporter-internal.corp.apple.com
< X-Content-Type-Options: nosniff
< X-XSS-Protection: 1; mode=block
< Content-Security-Policy: style-src  'self' blob: *.apple.com *.cdn-apple.com; child-src  'self' blob: *.apple.com; font-src  'self' data: *.apple.com *.cdn-apple.com; img-src  'self' data: *.apple.com *.cdn-apple.com; default-src  'self' *.apple.com;
< X-Rate-Limit: user-hour-lim:3600;user-hour-rem:3599;
< X-Apple-Jingle-Correlation-Key: DQP5APJ3KIDSA4YI44OPYD4B5A
< x-daiquiri-instance: daiquiri:18493001:mr85p00it-hyhk03154801:7987:23RELEASE10:daiquiri-amp-all-shared-ext-001-mr
< 
```
