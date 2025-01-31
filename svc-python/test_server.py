import httpx
import asyncio
import os
import zipfile
import json
import logging
import traceback

logging.basicConfig(level=logging.DEBUG)

BASE_URL = "http://127.0.0.1:8000/api/v1"

async def test_health_endpoint():
    """Test the health endpoint"""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{BASE_URL}/health")
        print("Health Endpoint Response:")
        print(f"Status Code: {response.status_code}")
        print(f"Response: {response.json()}")
        assert response.status_code == 200, "Health endpoint failed"

async def create_test_zip():
    """Create a test ZIP file with sample JSON data"""
    test_data = [
        {"actor": {"id": 1, "login": "user1"}},
        {"actor": {"id": 2, "login": "user2"}}
    ]
    
    # Ensure test directory exists
    os.makedirs("/tmp/test_uploads", exist_ok=True)
    
    # Create test JSON files
    json_files = [
        "/tmp/test_uploads/events1.json",
        "/tmp/test_uploads/events2.json"
    ]
    
    for json_file in json_files:
        with open(json_file, "w") as f:
            json.dump(test_data, f)
        print(f"Created JSON file: {json_file}")
        with open(json_file, 'r') as f:
            print(f"File contents: {f.read()}")
    
    # Create ZIP file
    zip_path = "/tmp/test_uploads/events.zip"
    with zipfile.ZipFile(zip_path, 'w') as zipf:
        for json_file in json_files:
            zipf.write(json_file, arcname=os.path.basename(json_file))
    
    # Verify ZIP contents
    with zipfile.ZipFile(zip_path, 'r') as zipf:
        print("ZIP file contents:")
        for name in zipf.namelist():
            print(f"File: {name}")
            print(f"Content: {zipf.read(name).decode('utf-8')}")
    
    return zip_path

async def test_upload_endpoint():
    """Test the file upload endpoint"""
    # Create test ZIP file
    zip_path = await create_test_zip()
    
    # Prepare file for upload
    async with httpx.AsyncClient() as client:
        try:
            # Get file size using os.path
            file_size = os.path.getsize(zip_path)
            print(f"File size: {file_size} bytes")
            
            with open(zip_path, 'rb') as f:
                files = {
                    'file': (
                        'events.zip', 
                        f, 
                        'application/zip'  # Explicitly set content type
                    )
                }
                response = await client.post(
                    f"{BASE_URL}/upload", 
                    files=files
                )
            
            print("\nUpload Endpoint Response:")
            print(f"Status Code: {response.status_code}")
            print(f"Response Headers: {response.headers}")
            print(f"Response Text: {response.text}")
            
            # Assertions
            assert response.status_code == 200, f"Upload endpoint failed. Response: {response.text}"
            
            try:
                response_data = response.json()
                print(f"Response JSON: {response_data}")
                assert response_data.get('total_files_processed') == 2, "Incorrect number of files processed"
                assert len(response_data.get('actors', [])) == 4, "Incorrect number of actors processed"
            except json.JSONDecodeError:
                print("Could not parse JSON response. Check server implementation.")
                print(f"Full response content: {response.text}")
                raise
        except Exception as e:
            print(f"Error during upload test: {e}")
            print(traceback.format_exc())
            raise

async def main():
    tasks = asyncio.gather(test_health_endpoint(), test_upload_endpoint())
    await tasks
    print("\n✅ All tests passed successfully!")

if __name__ == "__main__":
    asyncio.run(main())
