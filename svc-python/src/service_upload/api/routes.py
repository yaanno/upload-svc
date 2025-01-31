import concurrent.futures
import logging
import traceback
import zipfile
import io
from typing import Dict, Any

from fastapi import APIRouter, File, UploadFile, HTTPException

from service_upload.core.config import MAX_FILE_SIZE, MAX_WORKERS, ALLOWED_FILE_TYPES
from service_upload.core.file_processor import FileProcessor

logger = logging.getLogger(__name__)
logger.setLevel(logging.DEBUG)  # Set to DEBUG for more detailed logging

router = APIRouter()

@router.post("/upload", response_model=Dict[str, Any])
async def upload_and_process(file: UploadFile = File(...)):
    """
    Optimized file upload and processing endpoint
    
    - Uses concurrent processing
    - Memory-efficient JSON parsing
    - Handles both array and streaming JSON
    """
    try:
        logger.debug(f"Received file upload: {file.filename}")
        logger.debug(f"File content type: {file.content_type}")
        
        # Validate file type
        if file.content_type not in ALLOWED_FILE_TYPES:
            logger.error(f"Invalid file type: {file.content_type}")
            raise HTTPException(status_code=400, detail=f"File must be one of {ALLOWED_FILE_TYPES}")
        
        # Validate file size before reading
        file.file.seek(0, 2)  # Go to end of file
        file_size = file.file.tell()
        file.file.seek(0)  # Reset file pointer
        
        if file_size > MAX_FILE_SIZE:
            logger.error(f"File size exceeds limit: {file_size} > {MAX_FILE_SIZE}")
            raise HTTPException(
                status_code=400, 
                detail=f"File size exceeds the limit of {MAX_FILE_SIZE} bytes. Current file size is {file_size} bytes."
            )
        
        # Process ZIP file with concurrent processing
        try:
            contents = await file.read()
            
            # Validate ZIP file
            try:
                zip_file_obj = zipfile.ZipFile(io.BytesIO(contents))
            except zipfile.BadZipFile:
                logger.error("Invalid ZIP file")
                raise HTTPException(
                    status_code=400,
                    detail="Invalid zip file"
                )
            except Exception as e:
                logger.error(f"Error reading ZIP file: {e}")
                raise HTTPException(
                    status_code=400,
                    detail="Invalid zip file"
                )
            
            # Validate total uncompressed size
            try:
                total_uncompressed_size = sum(
                    zip_file_obj.getinfo(name).file_size 
                    for name in zip_file_obj.namelist()
                )
                
                if total_uncompressed_size > MAX_FILE_SIZE:
                    logger.error(f"Uncompressed file size exceeds limit: {total_uncompressed_size} > {MAX_FILE_SIZE}")
                    raise HTTPException(
                        status_code=400, 
                        detail=f"Total uncompressed file size exceeds the limit of {MAX_FILE_SIZE} bytes."
                    )
                
                json_files = [
                    name for name in zip_file_obj.namelist() 
                    if name.endswith('.json')
                ]
                
                if not json_files:
                    logger.error("No JSON files found in ZIP")
                    raise HTTPException(
                        status_code=400,
                        detail="No JSON files found in the uploaded ZIP"
                    )
                
                logger.debug(f"Found JSON files in ZIP: {json_files}")
                
                # Use ProcessPoolExecutor for true parallelism
                with concurrent.futures.ProcessPoolExecutor(max_workers=MAX_WORKERS) as executor:
                    # Prepare file processing tasks
                    file_tasks = [
                        executor.submit(
                            FileProcessor.process_json_file, 
                            name, 
                            zip_file_obj.read(name)
                        ) for name in json_files
                    ]
                    
                    # Collect results
                    processed_actors = []
                    for future in concurrent.futures.as_completed(file_tasks):
                        try:
                            actors = future.result()
                            processed_actors.extend(actors)
                            # logger.debug(f"Processed actors: {actors}")
                        except Exception as task_error:
                            logger.error(f"Task processing error: {task_error}")
                            logger.error(traceback.format_exc())
                
                logger.debug(f"Total processed actors: {len(processed_actors)}")
                
                return {
                    "total_files_processed": len(json_files),
                    "total_actors": len(processed_actors),
                    "actors": processed_actors
                }
            except HTTPException:
                # Re-raise HTTPException to preserve its details
                raise
            except Exception as e:
                logger.error(f"Unexpected error processing ZIP: {e}")
                raise HTTPException(
                    status_code=400,
                    detail="Error processing the uploaded ZIP file"
                )
        except HTTPException:
            # Re-raise HTTPException to preserve its details
            raise
        except Exception as e:
            logger.error(f"Unexpected error: {e}")
            raise HTTPException(
                status_code=400,
                detail="Invalid zip file"
            )
    
    except HTTPException:
        # Re-raise HTTPException to preserve its status code and detail
        raise
    except Exception as unexpected_error:
        logger.error(f"Unexpected global error: {unexpected_error}")
        logger.error(traceback.format_exc())
        raise HTTPException(status_code=500, detail="Unexpected server error")

@router.get("/health")
async def health_check():
    """
    Performance monitoring endpoint
    """
    return {
        "status": "healthy",
        "max_workers": MAX_WORKERS,
        "max_file_size": MAX_FILE_SIZE
    }
