import os
from typing import Final

# Server Configuration
MAX_FILE_SIZE: Final[int] = 100 * 1024 * 1024  # 100 MB
MAX_WORKERS: Final[int] = os.cpu_count() or 4

# Logging Configuration
LOG_LEVEL: Final[str] = os.getenv('LOG_LEVEL', 'INFO')
LOG_FORMAT: Final[str] = '%(asctime)s - %(name)s - %(levelname)s - %(message)s'

# Allowed File Types
ALLOWED_FILE_TYPES: Final[list[str]] = ['application/zip']


