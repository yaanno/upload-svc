FROM python:3.9-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy benchmark scripts
COPY scripts/ ./scripts/
COPY data/ ./data/

# Generate test data
RUN python data/generate_test_data.py

# Set the entrypoint to the benchmark script
ENTRYPOINT ["bash", "scripts/run_benchmarks.sh"]
