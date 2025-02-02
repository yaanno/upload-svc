| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `curl -X POST http://localhost:8080/upload -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 262.6 ± 4.6 | 259.5 | 268.0 | 1.00 |
| `curl -X POST http://localhost:8080/upload_large -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 458.7 ± 12.8 | 445.5 | 471.1 | 1.75 ± 0.06 |
