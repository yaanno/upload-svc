import time
import json
import zipfile
import io
import concurrent.futures
import pytest

from service_upload.core.file_processor import FileProcessor
from service_upload.core.config import MAX_WORKERS

def generate_test_events(num_events):
    """Generate a list of test events"""
    return [
        {"actor": {"id": i, "login": f"user{i}"}} 
        for i in range(num_events)
    ]

def create_test_zip(events):
    """Create a test ZIP file with JSON events"""
    zip_buffer = io.BytesIO()
    with zipfile.ZipFile(zip_buffer, 'w') as zip_file:
        # Use UTF-8 encoding explicitly
        json_content = json.dumps(events).encode('utf-8')
        print(f"JSON Content Length: {len(json_content)}")
        print(f"First 100 bytes: {json_content[:100]}")
        zip_file.writestr('events.json', json_content)
    zip_buffer.seek(0)
    return zip_buffer

def test_performance_sequential():
    """Test sequential processing performance"""
    events = generate_test_events(1000)
    zip_file = create_test_zip(events)
    
    # Debug: print raw contents
    raw_contents = zip_file.getvalue()
    print(f"Raw Contents Length: {len(raw_contents)}")
    print(f"First 100 bytes: {raw_contents[:100]}")
    
    start_time = time.time()
    actors = FileProcessor.process_json_file('events.json', raw_contents)
    end_time = time.time()
    
    print(f"Actors found: {len(actors)}")
    
    assert len(actors) == 1000
    processing_time = end_time - start_time
    print(f"\nSequential Processing Time: {processing_time:.4f} seconds")
    assert processing_time < 1.0  # Should complete under 1 second

def test_performance_parallel():
    """Test parallel processing performance"""
    # Create multiple test files
    test_files = [
        ('events1.json', generate_test_events(500)),
        ('events2.json', generate_test_events(500)),
        ('events3.json', generate_test_events(500))
    ]
    
    start_time = time.time()
    with concurrent.futures.ProcessPoolExecutor(max_workers=MAX_WORKERS) as executor:
        # Submit processing tasks
        file_tasks = [
            executor.submit(
                FileProcessor.process_json_file, 
                name, 
                json.dumps(events).encode('utf-8')
            ) for name, events in test_files
        ]
        
        # Collect results
        all_actors = []
        for future in concurrent.futures.as_completed(file_tasks):
            all_actors.extend(future.result())
    
    end_time = time.time()
    
    assert len(all_actors) == 1500
    processing_time = end_time - start_time
    print(f"\nParallel Processing Time: {processing_time:.4f} seconds")
    assert processing_time < 0.5  # Should complete under 0.5 seconds

def test_memory_efficiency():
    """Test memory efficiency of file processing"""
    import tracemalloc
    
    events = generate_test_events(10000)
    zip_file = create_test_zip(events)
    
    # Start memory tracking
    tracemalloc.start()
    
    actors = FileProcessor.process_json_file('events.json', zip_file.getvalue())
    
    # Get memory usage
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    
    assert len(actors) == 10000
    print(f"\nMemory Usage:")
    print(f"Current Memory: {current / 10**6:.2f} MB")
    print(f"Peak Memory: {peak / 10**6:.2f} MB")
    
    # Ensure memory usage is reasonable
    assert peak < 50 * 10**6  # Less than 50 MB
