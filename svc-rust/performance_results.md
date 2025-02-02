| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `curl -X POST http://localhost:8080/upload -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 325.7 ± 41.9 | 295.5 | 373.6 | 1.03 ± 0.14 |
| `curl -X POST http://localhost:8080/upload_large -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 316.3 ± 16.4 | 304.8 | 335.0 | 1.00 |
