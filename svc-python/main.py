from fastapi import FastAPI, File, UploadFile, HTTPException
import zipfile
import os
import orjson
import io

app = FastAPI()

MAX_FILE_SIZE = 10 * 1024 * 1024  # 10 MB

def sanitize_filename(filename):
    return os.path.basename(filename)

@app.post("/upload")
async def upload(file: UploadFile = File(...)):
    if file.content_type != 'application/zip':
        raise HTTPException(status_code=400, detail="File must be a zip archive")
    contents = await file.read()

    if len(contents) > MAX_FILE_SIZE:
        raise HTTPException(status_code=400, detail="File size exceeds the limit")

    try:
        with zipfile.ZipFile(io.BytesIO(contents)) as zip_file:
            json_data = []
            for name in zip_file.namelist():
                sanitized_name = sanitize_filename(name)
                if sanitized_name.endswith('.json'):
                    with zip_file.open(name) as json_file:
                        data = orjson.loads(json_file.read())
                        json_data.append({sanitized_name: data})
    except zipfile.BadZipFile:
        raise HTTPException(status_code=400, detail="Invalid zip file")

    return {"json_files": orjson.dumps(json_data)}
