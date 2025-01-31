# Service Upload - Python Implementation

## 🚀 Project Overview

A high-performance, memory-efficient file upload and processing service built with FastAPI, designed to handle large JSON files with optimal concurrency and minimal resource consumption.

## ✨ Features

- **Concurrent File Processing**: Utilizes Python's multiprocessing for parallel JSON file handling
- **Memory-Efficient Parsing**: Streaming JSON decoder to minimize memory overhead
- **Flexible Input Handling**: Supports both array and individual event JSON formats
- **Robust Error Handling**: Comprehensive error logging and graceful error management
- **Performance Monitoring**: Built-in health check endpoint

## 🛠 Tech Stack

- **Framework**: FastAPI
- **JSON Parsing**: orjson
- **Concurrency**: `concurrent.futures`
- **Dependency Management**: Poetry
- **Testing**: pytest, httpx

## 📦 Prerequisites

- Python 3.13+
- Poetry

## 🔧 Installation

1. Clone the repository
2. Install dependencies:
```bash
poetry install
```

## 🚀 Running the Server

```bash
poetry run uvicorn service_upload.main:app --host 0.0.0.0 --port 8000
```

## 🧪 Running Tests

```bash
poetry run pytest tests/
```

## 📊 Performance Characteristics

### Concurrency Strategy
- Uses `ProcessPoolExecutor` for true parallel processing
- Dynamically sets worker count based on CPU cores
- Minimizes goroutine/thread overhead

### Memory Management
- Streaming JSON parsing
- Minimal memory allocations
- Efficient actor/event extraction

## 🔍 Endpoint Details

### `/api/v1/upload` (POST)
- Accepts ZIP files containing JSON
- Processes multiple JSON files concurrently
- Returns processed actors/events

### `/api/v1/health` (GET)
- Returns server health information
- Provides configuration details

## 🛡 Error Handling

- Validates file type and size
- Handles malformed ZIP and JSON files
- Provides descriptive error messages

## 📈 Benchmarking

Benchmark results available in `BENCH_NOTES.md`

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## 📄 License

[Insert License Information]

## 👥 Authors

- Janos Hardi <janos-karoly.hardi@telekom.com>
