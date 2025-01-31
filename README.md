# Service Upload Performance Comparison

## Project Overview

This project is a comprehensive performance benchmark of file upload and processing services implemented in three different programming languages: Go, Rust, and Python.

### Objective

The primary goal is to compare and analyze the performance characteristics of each language implementation for a common file upload and processing task.

## Service Workflow

1. **File Upload Endpoint**
   - Expose a REST API endpoint for file uploads
   - Accept ZIP files containing JSON data

2. **File Handling**
   - Receive uploaded file
   - Save file to a temporary directory
   - Unzip the uploaded file
   - Extract and process JSON files

## Implementation Languages

### 1. Go (Golang)
- Web Framework: Gin
- Performance-oriented implementation
- Concurrent processing
- Structured logging

### 2. Rust
- Web Framework: Actix Web
- Zero-cost abstractions
- High performance and memory safety
- Efficient JSON processing

### 3. Python
- Web Framework: FastAPI
- Async processing
- Type hints and validation
- Modern Python async capabilities

## Performance Metrics

Each implementation will be evaluated on:
- Request Throughput
- Memory Usage
- CPU Utilization
- Latency
- Concurrency Handling

### Benchmarking Methodology

- Consistent test dataset
- Identical hardware/environment
- Multiple test runs
- Metrics collected using:
  - Profiling tools
  - System monitoring
  - Detailed logging

## Running Benchmarks

### Prerequisites
- Docker
- Docker Compose
- Performance monitoring tools

### Benchmark Steps
1. Prepare test data
2. Run each service
3. Execute load tests
4. Collect and analyze performance data

## Example Benchmark Command
```bash
docker-compose up -d
./run-benchmarks.sh