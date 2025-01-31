import os
import json
import statistics
import pandas as pd
import matplotlib.pyplot as plt

def analyze_benchmarks(results_dir):
    results = {}
    
    # Collect results
    for filename in os.listdir(results_dir):
        if filename.endswith('.json'):
            service, size, _ = filename.split('_')
            
            with open(os.path.join(results_dir, filename), 'r') as f:
                data = json.load(f)
                
                if service not in results:
                    results[service] = {}
                if size not in results[service]:
                    results[service][size] = []
                
                results[service][size].append(data)
    
    # Aggregate and analyze
    summary = {}
    for service, sizes in results.items():
        summary[service] = {}
        for size, measurements in sizes.items():
            summary[service][size] = {
                'response_time': {
                    'mean': statistics.mean(m['response_time'] for m in measurements),
                    'std': statistics.stdev(m['response_time'] for m in measurements)
                },
                'cpu_usage': {
                    'mean': statistics.mean(m['cpu_usage'] for m in measurements),
                    'std': statistics.stdev(m['cpu_usage'] for m in measurements)
                },
                'memory_usage': {
                    'mean': statistics.mean(m['memory_usage']['after'] for m in measurements),
                    'std': statistics.stdev(m['memory_usage']['after'] for m in measurements)
                }
            }
    
    # Generate visualizations
    plot_performance_comparison(summary)
    
    return summary

def plot_performance_comparison(summary):
    services = list(summary.keys())
    sizes = list(summary[services[0]].keys())
    
    # Response Time
    plt.figure(figsize=(15, 5))
    plt.subplot(131)
    plt.title('Response Time Comparison')
    for service in services:
        plt.plot(sizes, [summary[service][size]['response_time']['mean'] for size in sizes], label=service)
    plt.xlabel('Data Size')
    plt.ylabel('Response Time (s)')
    plt.legend()
    
    # CPU Usage
    plt.subplot(132)
    plt.title('CPU Usage Comparison')
    for service in services:
        plt.plot(sizes, [summary[service][size]['cpu_usage']['mean'] for size in sizes], label=service)
    plt.xlabel('Data Size')
    plt.ylabel('CPU Usage (%)')
    plt.legend()
    
    # Memory Usage
    plt.subplot(133)
    plt.title('Memory Usage Comparison')
    for service in services:
        plt.plot(sizes, [summary[service][size]['memory_usage']['mean'] / 1024 / 1024 for size in sizes], label=service)
    plt.xlabel('Data Size')
    plt.ylabel('Memory Usage (MB)')
    plt.legend()
    
    plt.tight_layout()
    plt.savefig('performance_comparison.png')

def main():
    results = analyze_benchmarks('results/')
    
    # Print detailed summary
    print(json.dumps(results, indent=2))

if __name__ == '__main__':
    main()
