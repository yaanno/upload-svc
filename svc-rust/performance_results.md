| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `curl -X POST http://localhost:8080/upload -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 249.6 ± 9.0 | 239.2 | 255.4 | 1.00 |
| `curl -X POST http://localhost:8080/upload_large -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 474.1 ± 2.5 | 471.9 | 476.8 | 1.90 ± 0.07 |
