import io
import logging
import json
import zipfile
from typing import List, Dict, Any

import orjson

logger = logging.getLogger(__name__)

class FileProcessor:
    @staticmethod
    def process_json_file(file_path: str, file_contents: bytes) -> List[Dict[str, Any]]:
        """
        Process a single JSON file with memory-efficient parsing
        
        Args:
            file_path (str): Path of the JSON file
            file_contents (bytes): Bytes content of the file
        
        Returns:
            List of processed actors/events
        """
        logger.debug(f"Processing file: {file_path}")
        logger.debug(f"File contents length: {len(file_contents)} bytes")
        
        try:
            # Check if the file is a ZIP file
            try:
                with zipfile.ZipFile(io.BytesIO(file_contents)) as zip_file:
                    # Find the first JSON file in the ZIP
                    json_files = [
                        name for name in zip_file.namelist() 
                        if name.lower().endswith('.json')
                    ]
                    
                    if not json_files:
                        logger.error("No JSON files found in ZIP")
                        return []
                    
                    # Read the first JSON file
                    file_contents = zip_file.read(json_files[0])
            except zipfile.BadZipFile:
                # If not a ZIP, assume it's a direct JSON file
                pass
            
            # Try multiple parsing methods
            parsing_methods = [
                # Method 1: Whole file as JSON list
                lambda content: (
                    json.loads(content.decode('utf-8')) 
                    if isinstance(json.loads(content.decode('utf-8')), list) 
                    else [json.loads(content.decode('utf-8'))]
                ),
                
                # Method 2: Whole file as JSON array
                lambda content: (
                    orjson.loads(content) 
                    if isinstance(orjson.loads(content), list) 
                    else [orjson.loads(content)]
                ),
                
                # Method 3: Line-by-line parsing
                lambda content: [
                    json.loads(line.strip()) 
                    for line in content.decode('utf-8').split('\n') 
                    if line.strip()
                ]
            ]
            
            # Try multiple encodings
            encodings = ['utf-8', 'latin-1', 'utf-16', 'iso-8859-1']
            
            for encoding in encodings:
                try:
                    # Decode content
                    content_str = file_contents.decode(encoding)
                    logger.debug(f"Decoding successful with {encoding} encoding")
                    
                    # Try each parsing method
                    for method_idx, method in enumerate(parsing_methods, 1):
                        try:
                            logger.debug(f"Attempting parsing method {method_idx}")
                            events = method(file_contents)
                            
                            # Extract actors
                            actors = [
                                event.get('actor', {}) 
                                for event in events 
                                if isinstance(event, dict) and 'actor' in event
                            ]
                            
                            logger.debug(f"Found {len(actors)} actors")
                            
                            if actors:
                                return actors
                        except (json.JSONDecodeError, orjson.JSONDecodeError) as e:
                            logger.debug(f"Parsing method {method_idx} failed: {e}")
                            continue
                
                except UnicodeDecodeError as ude:
                    logger.debug(f"Decoding with {encoding} failed: {ude}")
                    continue
            
            # If no strategy works
            logger.error(f"Could not parse JSON file: {file_path}")
            return []
        
        except Exception as e:
            logger.error(f"Error processing {file_path}: {e}")
            return []
