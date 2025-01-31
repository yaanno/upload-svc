import io
import json
import pytest

from service_upload.core.file_processor import FileProcessor

def test_process_json_file_array():
    """Test processing a JSON file with an array of events"""
    test_data = [
        {"actor": {"id": 1, "login": "user1"}},
        {"actor": {"id": 2, "login": "user2"}}
    ]
    
    # Convert test data to bytes
    file_contents = json.dumps(test_data).encode('utf-8')
    
    # Process the file
    actors = FileProcessor.process_json_file("test.json", file_contents)
    
    # Assertions
    assert len(actors) == 2
    assert actors[0] == {"id": 1, "login": "user1"}
    assert actors[1] == {"id": 2, "login": "user2"}

def test_process_json_file_individual_events():
    """Test processing a JSON file with individual events"""
    test_data1 = {"actor": {"id": 1, "login": "user1"}}
    test_data2 = {"actor": {"id": 2, "login": "user2"}}
    
    # Create a file-like object with individual events
    file_contents = (
        json.dumps(test_data1) + "\n" + 
        json.dumps(test_data2)
    ).encode('utf-8')
    
    # Process the file
    actors = FileProcessor.process_json_file("test.json", file_contents)
    
    # Assertions
    assert len(actors) == 2
    assert actors[0] == {"id": 1, "login": "user1"}
    assert actors[1] == {"id": 2, "login": "user2"}

def test_process_json_file_empty():
    """Test processing an empty file"""
    file_contents = b""
    
    # Process the file
    actors = FileProcessor.process_json_file("empty.json", file_contents)
    
    # Assertions
    assert len(actors) == 0

def test_process_json_file_invalid():
    """Test processing an invalid JSON file"""
    file_contents = b"Invalid JSON data"
    
    # Process the file
    actors = FileProcessor.process_json_file("invalid.json", file_contents)
    
    # Assertions
    assert len(actors) == 0
