import io
import json
import zipfile
import pytest
from fastapi.testclient import TestClient
import logging
from service_upload.main import app
from service_upload.core.config import MAX_FILE_SIZE

logger = logging.getLogger(__name__)
logger.setLevel(logging.DEBUG) 

client = TestClient(app)

def create_test_zip(events=None):
    """Helper function to create a test ZIP file"""
    if events is None:
        events = [
            {"actor": {"id": 1, "login": "user1"}},
            {"actor": {"id": 2, "login": "user2"}}
        ]
    
    # Create a temporary file
    zip_buffer = io.BytesIO()
    
    with zipfile.ZipFile(zip_buffer, 'w', compression=zipfile.ZIP_DEFLATED) as zip_file:
        # Write JSON file to ZIP
        zip_file.writestr('events.json', json.dumps(events))
    
    # Reset buffer position
    zip_buffer.seek(0)
    
    return zip_buffer

def test_health_endpoint():
    """Test the health endpoint"""
    response = client.get("/api/v1/health")
    
    assert response.status_code == 200
    data = response.json()
    
    assert data['status'] == 'healthy'
    assert 'max_workers' in data
    assert 'max_file_size' in data

def test_upload_endpoint_success():
    """Test successful file upload"""
    zip_file = create_test_zip()
    
    response = client.post(
        "/api/v1/upload", 
        files={'file': ('events.zip', zip_file, 'application/zip')}
    )
    
    assert response.status_code == 200
    data = response.json()
    
    assert data['total_files_processed'] == 1
    assert data['total_actors'] == 2
    assert len(data['actors']) == 2
    assert data['actors'][0]['id'] == 1
    assert data['actors'][1]['id'] == 2

def test_upload_endpoint_invalid_file_type():
    """Test upload with invalid file type"""
    # Create a text file instead of a ZIP
    file_buffer = io.BytesIO(b"Not a ZIP file")
    
    response = client.post(
        "/api/v1/upload", 
        files={'file': ('invalid.txt', file_buffer, 'text/plain')}
    )
    
    assert response.status_code == 400
    assert "File must be" in response.json()['detail']

def test_upload_endpoint_file_too_large():
    """Test upload of a file exceeding size limit"""
    # Create a large ZIP file
    large_events = [{"actor": {"id": i, "login": f"user{i}", "details": "x" * 1000}} for i in range(MAX_FILE_SIZE // 1000)]
    zip_buffer = io.BytesIO()
    
    with zipfile.ZipFile(zip_buffer, 'w', compression=zipfile.ZIP_DEFLATED) as zip_file:
        # Create a large payload that exceeds MAX_FILE_SIZE
        large_payload = json.dumps(large_events)
        zip_file.writestr('events.json', large_payload)
    
    # Reset buffer position
    zip_buffer.seek(0)
    
    response = client.post(
        "/api/v1/upload", 
        files={'file': ('large_events.zip', zip_buffer, 'application/zip')}
    )
    assert response.status_code == 400

def test_upload_endpoint_invalid_zip():
    """Test upload of an invalid ZIP file"""
    # Create an invalid ZIP file
    invalid_zip = io.BytesIO(b"Invalid ZIP content")
    
    response = client.post(
        "/api/v1/upload", 
        files={'file': ('invalid.zip', invalid_zip, 'application/zip')}
    )
    
    assert response.status_code == 400
    assert "Invalid zip file" in response.json()['detail']
