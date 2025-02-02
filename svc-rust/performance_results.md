| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `curl -X POST http://localhost:8080/upload -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 270.1 ± 30.7 | 242.4 | 303.2 | 1.00 |
| `curl -X POST http://localhost:8080/upload_large -F "file=@/Users/A200246910/workspace/service-upload/svc-rust/ArchiveLarge.zip"` | 473.4 ± 10.8 | 463.8 | 485.1 | 1.75 ± 0.20 |
