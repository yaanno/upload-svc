# Upload Service

## Requirements

- Secure & Authenticated endpoint
- OpenAPI documentation

## Data flow

- expose REST api to upload zip file
- save zip file to `tmp` directory
- unzip file
- validate file
- process file
- save processed data
