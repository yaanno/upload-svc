#!/bin/bash

# Exit on first error
set -e

# Benchmark configuration
ITERATIONS=10
SIZES=(10 100 1000 10000)
SERVICES=("svc-go" "svc-rust" "svc-python")

# Create results directory
mkdir -p results

# Function to run load test
run_load_test() {
    local service=$1
    local size=$2
    local iteration=$3
    
    echo "Running benchmark for $service with $size events (Iteration $iteration)"
    
    # Run load test using Python script
    python load_test.py \
        --service $service \
        --data "../data/test_data/github_events_${size}.json.zip" \
        --output "results/${service}_${size}_${iteration}.json"
}

# Main benchmarking loop
for service in "${SERVICES[@]}"; do
    for size in "${SIZES[@]}"; do
        for ((i=1; i<=ITERATIONS; i++)); do
            run_load_test $service $size $i
        done
    done
done

# Analyze results
python analyze_results.py results/
