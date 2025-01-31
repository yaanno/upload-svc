#!/bin/bash

# Define the target directory
TARGET_DIR="./tmp"

# Check if the directory exists
if [ -d "$TARGET_DIR" ]; then
  # Remove all files and directories under the target directory
  rm -rf "$TARGET_DIR"/*
  echo "All files and directories under $TARGET_DIR have been deleted."
else
  echo "Directory $TARGET_DIR does not exist."
fi

# Simulate file upload
curl -X POST http://localhost:8080/upload \
  -F "file=@upload.zip" \
  -H "Content-Type: multipart/form-data"