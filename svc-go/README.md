# Golang Upload Service

## Overview

This is a high-performance, scalable microservice designed for processing ZIP file uploads containing JSON files. The service is built using Go (Golang) with the Gin web framework and provides robust file handling, processing, and error management.

## Features

- 🚀 Fast and efficient file upload processing
- 📦 ZIP file extraction with comprehensive validation
- 🔒 Secure file handling with size and type restrictions
- 📝 Detailed logging and error reporting
- 🔍 Performance profiling support
- 🌐 Graceful server shutdown

## Prerequisites

- Go 1.18+ 
- Docker (optional, for containerization)

## Configuration

The service can be configured using environment variables:

| Variable           | Description                   | Default Value    |
|--------------------|-------------------------------|-----------------|
| `SERVER_PORT`      | HTTP server port              | 8080            |
| `READ_TIMEOUT`     | HTTP read timeout             | 10s             |
| `WRITE_TIMEOUT`    | HTTP write timeout            | 10s             |
| `MAX_UPLOAD_SIZE`  | Maximum file upload size      | 300MB           |
| `TEMP_STORAGE_PATH`| Temporary file storage path   | ./tmp           |

## Local Development

### Setup

1. Clone the repository
```bash
git clone https://github.com/your-org/service-upload.git
cd svc-go
```

2. Install dependencies
```bash
go mod tidy
```

3. Run the service
```bash
go run cmd/server/main.go
```

### Testing

Run unit and integration tests:
```bash
go test ./...
```

## Docker Deployment

### Build Docker Image
```bash
docker build -t upload-service .
```

### Run Docker Container
```bash
docker run -p 8080:8080 upload-service
```

## API Endpoints

### Upload Endpoint
- **URL**: `/upload`
- **Method**: `POST`
- **Payload**: Multipart form-data with a ZIP file
- **Success Response**: 
  - Code: 200
  - Content: 
    ```json
    {
      "message": "Upload successful",
      "data": [...]
    }
    ```

### Error Responses

The service provides detailed error responses with the following structure:

```json
{
  "code": "ERROR_CODE",
  "message": "Human-readable error message",
  "details": "Specific error description"
}
```

Common Error Codes:
- `FILE_UPLOAD_ERROR`
- `FILE_SIZE_ERROR`
- `FILE_TYPE_ERROR`
- `FILE_EXTRACTION_ERROR`
- `FILE_PROCESSING_ERROR`

## Performance Profiling

The service includes `pprof` routes for performance analysis. Access profiling data at:
- `/debug/pprof/`
- `/debug/pprof/heap`
- `/debug/pprof/profile`

## Logging

Structured logging is implemented using `zap`. Log levels and formats can be customized in the logger configuration.

## Security Considerations

- File size limits prevent large file uploads
- File type validation
- Secure file path handling to prevent directory traversal
- Timeout-based context management

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Your Name - your.email@example.com

Project Link: [https://github.com/your-org/service-upload](https://github.com/your-org/service-upload)
