#!/bin/bash

# Performance Testing Script for svc-rust

# Exit on first error
set -e

# Directories
PROJECT_DIR="/Users/A200246910/workspace/service-upload/svc-rust"
TEST_ARCHIVE="$PROJECT_DIR/ArchiveLarge.zip"

# Kill any existing processes using port 8080
lsof -ti:8080 | xargs -r kill -9 || true

# Ensure test archive exists
if [ ! -f "$TEST_ARCHIVE" ]; then
    echo "Generating test archive..."
    python3 "$PROJECT_DIR/generate_test_data.py"
fi

# Build the Rust service
echo "Building Rust service..."
cd "$PROJECT_DIR"
cargo build --release

# Start the service in the background
echo "Starting Rust service..."
cargo run --release &
SERVICE_PID=$!

# Wait for service to start
sleep 2

# Performance testing with hyperfine
echo "Running performance benchmarks..."
hyperfine \
    --warmup 3 \
    --min-runs 5 \
    --max-runs 50 \
    --show-output \
    --export-markdown performance_results.md \
    --export-json performance_results.json \
    'curl -v -X POST http://localhost:8080/upload -F "file=@'"$TEST_ARCHIVE"'"'

# Capture system resources
echo "Capturing system resources..."
top -l 1 -n 5 > system_resources.txt

# Kill the service
kill $SERVICE_PID || true

# Display results
echo "Performance testing complete."
echo "Results saved in:"
echo "- performance_results.md"
echo "- performance_results.json"
echo "- system_resources.txt"

# Optional: Show markdown results
cat performance_results.md