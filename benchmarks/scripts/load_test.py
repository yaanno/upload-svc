import argparse
import requests
import time
import json
import os
import psutil
import tracemalloc

def measure_performance(url, file_path):
    start_time = time.time()
    
    # Start memory tracing
    tracemalloc.start()
    
    # Measure CPU and memory before request
    process = psutil.Process()
    cpu_before = process.cpu_percent()
    mem_before = process.memory_info().rss
    
    # Perform file upload
    with open(file_path, 'rb') as file:
        files = {'file': file}
        response = requests.post(url, files=files)
    
    # Measure CPU and memory after request
    cpu_after = process.cpu_percent()
    mem_after = process.memory_info().rss
    
    # Get memory snapshot
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    
    end_time = time.time()
    
    return {
        'response_time': end_time - start_time,
        'status_code': response.status_code,
        'cpu_usage': (cpu_after + cpu_before) / 2,
        'memory_usage': {
            'before': mem_before,
            'after': mem_after,
            'peak_traced': peak
        },
        'response_size': len(response.content)
    }

def main():
    parser = argparse.ArgumentParser(description='Performance Benchmark')
    parser.add_argument('--service', required=True, help='Service to test')
    parser.add_argument('--data', required=True, help='Path to test data')
    parser.add_argument('--output', required=True, help='Output results file')
    
    args = parser.parse_args()
    
    # Map service to URL
    service_urls = {
        'svc-go': 'http://localhost:8080/upload',
        'svc-rust': 'http://localhost:8081/upload',
        'svc-python': 'http://localhost:8082/upload'
    }
    
    results = measure_performance(service_urls[args.service], args.data)
    
    # Save results
    with open(args.output, 'w') as f:
        json.dump(results, f)

if __name__ == '__main__':
    main()
