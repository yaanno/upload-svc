import logging
from fastapi import FastAPI

from service_upload.api.routes import router
from service_upload.core.config import LOG_LEVEL, LOG_FORMAT

# Configure logging
logging.basicConfig(
    level=getattr(logging, LOG_LEVEL),
    format=LOG_FORMAT
)

# Create FastAPI application
app = FastAPI(
    title="Service Upload",
    description="High-performance file upload and processing service",
    version="0.1.0"
)

# Include routes
app.include_router(router, prefix="/api/v1")

# Optional: Add startup and shutdown events
@app.on_event("startup")
async def startup_event():
    logging.info("Application is starting up...")

@app.on_event("shutdown")
async def shutdown_event():
    logging.info("Application is shutting down...")
