package handler

import (
	"archive/zip"
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"

	"service-upload/svc-go/internal/config"
	"service-upload/svc-go/internal/service"
	"service-upload/svc-go/pkg/models"
)

// Add these constants at the top of the file
const (
	maxFileSize = 300 * 1024 * 1024 // 300 MB
)

// Custom error types for more granular error handling
var (
	ErrFileUpload = errors.New("file upload error")
	ErrFileSize   = errors.New("file size error")
	ErrFileType   = errors.New("file type error")
	ErrExtraction = errors.New("file extraction error")
	ErrProcessing = errors.New("file processing error")
)

// createErrorResponse generates a standardized error response
func createErrorResponse(err error, defaultCode string, defaultMessage string) ErrorResponse {
	switch {
	case errors.Is(err, ErrFileUpload):
		return ErrorResponse{
			Code:    "FILE_UPLOAD_ERROR",
			Message: "Failed to upload file",
			Details: err.Error(),
		}
	case errors.Is(err, ErrFileSize):
		return ErrorResponse{
			Code:    "FILE_SIZE_ERROR",
			Message: "File size exceeds limit",
			Details: err.Error(),
		}
	case errors.Is(err, ErrFileType):
		return ErrorResponse{
			Code:    "FILE_TYPE_ERROR",
			Message: "Invalid file type",
			Details: err.Error(),
		}
	case errors.Is(err, ErrExtraction):
		return ErrorResponse{
			Code:    "FILE_EXTRACTION_ERROR",
			Message: "Failed to extract files",
			Details: err.Error(),
		}
	case errors.Is(err, ErrProcessing):
		return ErrorResponse{
			Code:    "FILE_PROCESSING_ERROR",
			Message: "Failed to process files",
			Details: err.Error(),
		}
	default:
		return ErrorResponse{
			Code:    defaultCode,
			Message: defaultMessage,
			Details: err.Error(),
		}
	}
}

// ErrorResponse provides a structured error response
type ErrorResponse struct {
	Code    string `json:"code"`
	Message string `json:"message"`
	Details string `json:"details,omitempty"`
}

type UploadHandler struct {
	logger        *zap.SugaredLogger
	config        *config.ServerConfig
	fileProcessor *service.FileProcessor
}

func NewUploadHandler(logger *zap.SugaredLogger, config *config.ServerConfig, fileProcessor *service.FileProcessor) *UploadHandler {
	return &UploadHandler{
		logger:        logger,
		config:        config,
		fileProcessor: fileProcessor,
	}
}

func (h *UploadHandler) HandleUpload(c *gin.Context) {
	// Detailed file upload validation
	file, err := c.FormFile("file")
	if err != nil {
		wrappedErr := fmt.Errorf("%w: %v", ErrFileUpload, err)
		h.logger.Errorw("File upload validation failed",
			"error", wrappedErr,
			"method", "HandleUpload",
		)
		c.JSON(http.StatusBadRequest, createErrorResponse(wrappedErr, "UPLOAD_ERROR", "File upload failed"))
		return
	}

	// Validate file size
	if file.Size > h.config.MaxUploadSize {
		wrappedErr := fmt.Errorf("%w: file size %d exceeds maximum allowed %d",
			ErrFileSize, file.Size, h.config.MaxUploadSize)
		h.logger.Errorw("File size validation failed",
			"fileSize", file.Size,
			"maxSize", h.config.MaxUploadSize,
			"method", "HandleUpload",
		)
		c.JSON(http.StatusBadRequest, createErrorResponse(wrappedErr, "SIZE_ERROR", "File too large"))
		return
	}

	// Validate file type (optional - you can expand this)
	if !strings.HasSuffix(file.Filename, ".zip") {
		wrappedErr := fmt.Errorf("%w: expected .zip file, got %s",
			ErrFileType, filepath.Ext(file.Filename))
		h.logger.Errorw("File type validation failed",
			"filename", file.Filename,
			"method", "HandleUpload",
		)
		c.JSON(http.StatusBadRequest, createErrorResponse(wrappedErr, "TYPE_ERROR", "Invalid file type"))
		return
	}

	// Create unique temporary directory
	timestamp := time.Now().UnixNano()
	path := filepath.Join(h.config.TempStoragePath, fmt.Sprintf("%d", timestamp))
	if err := os.MkdirAll(path, os.ModePerm); err != nil {
		wrappedErr := fmt.Errorf("failed to create temporary directory: %w", err)
		h.logger.Errorw("Temporary directory creation failed",
			"error", wrappedErr,
			"path", path,
			"method", "HandleUpload",
		)
		c.JSON(http.StatusInternalServerError, createErrorResponse(wrappedErr, "DIRECTORY_ERROR", "Failed to create storage directory"))
		return
	}

	// Save uploaded file
	zipFilePath := filepath.Join(path, file.Filename)
	if err := c.SaveUploadedFile(file, zipFilePath); err != nil {
		wrappedErr := fmt.Errorf("%w: unable to save uploaded file", ErrFileUpload)
		h.logger.Errorw("File save failed",
			"error", wrappedErr,
			"path", zipFilePath,
			"method", "HandleUpload",
		)
		c.JSON(http.StatusInternalServerError, createErrorResponse(wrappedErr, "SAVE_ERROR", "Failed to save uploaded file"))
		return
	}

	// Extract files
	files, err := unzip(zipFilePath, path)
	if err != nil {
		wrappedErr := fmt.Errorf("%w: %v", ErrExtraction, err)
		h.logger.Errorw("ZIP extraction failed",
			"error", wrappedErr,
			"zipFile", zipFilePath,
			"method", "HandleUpload",
		)
		c.JSON(http.StatusInternalServerError, createErrorResponse(wrappedErr, "EXTRACTION_ERROR", "Failed to extract files"))
		return
	}

	// Process files with context and timeout
	ctx, cancel := context.WithTimeout(c.Request.Context(), 2*time.Minute)
	defer cancel()

	actors, err := h.fileProcessor.ProcessFiles(ctx, files)
	if err != nil {
		wrappedErr := fmt.Errorf("%w: %v", ErrProcessing, err)
		h.logger.Errorw("File processing failed",
			"error", wrappedErr,
			"fileCount", len(files),
			"method", "HandleUpload",
		)
		c.JSON(http.StatusInternalServerError, createErrorResponse(wrappedErr, "PROCESSING_ERROR", "Failed to process files"))
		return
	}

	// Successful response
	c.JSON(http.StatusOK, models.GenericResponse{
		Message: "Upload successful",
		Data:    actors,
	})
}

func unzip(source, destination string) ([]string, error) {
	// Open the zip file
	reader, err := zip.OpenReader(source)
	if err != nil {
		return nil, fmt.Errorf("failed to open zip file: %w", err)
	}
	defer reader.Close()

	// Create the destination directory if it doesn't exist
	if err := os.MkdirAll(destination, os.ModePerm); err != nil {
		return nil, fmt.Errorf("failed to create destination directory: %w", err)
	}

	var extractedFiles []string
	var errors []error

	// Process each file in the zip archive
	for _, file := range reader.File {
		// Skip directories
		if file.FileInfo().IsDir() {
			continue
		}

		// Validate file path to prevent zip slip vulnerability
		filePath := filepath.Join(destination, file.Name)
		if !strings.HasPrefix(filePath, filepath.Clean(destination)+string(os.PathSeparator)) {
			errors = append(errors, fmt.Errorf("invalid file path: %s", file.Name))
			continue
		}

		// Check file size
		if file.UncompressedSize64 > maxFileSize {
			errors = append(errors, fmt.Errorf("file too large: %s", file.Name))
			continue
		}

		// Only process JSON files
		if !strings.HasSuffix(strings.ToLower(file.Name), ".json") {
			continue
		}

		// Create parent directories if they don't exist
		if err := os.MkdirAll(filepath.Dir(filePath), os.ModePerm); err != nil {
			errors = append(errors, fmt.Errorf("failed to create directory for %s: %w", file.Name, err))
			continue
		}

		// Create the file
		outFile, err := os.OpenFile(filePath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
		if err != nil {
			errors = append(errors, fmt.Errorf("failed to create file %s: %w", file.Name, err))
			continue
		}
		defer outFile.Close()

		// Open the file in the zip
		rc, err := file.Open()
		if err != nil {
			errors = append(errors, fmt.Errorf("failed to open file in zip %s: %w", file.Name, err))
			continue
		}
		defer rc.Close()

		// Copy the file contents
		_, err = io.Copy(outFile, rc)
		if err != nil {
			errors = append(errors, fmt.Errorf("failed to extract file %s: %w", file.Name, err))
			continue
		}

		extractedFiles = append(extractedFiles, filePath)
	}

	// If there were any errors during extraction
	if len(errors) > 0 {
		errorMessages := make([]string, len(errors))
		for i, err := range errors {
			errorMessages[i] = err.Error()
		}
		return extractedFiles, fmt.Errorf("extraction errors: %v", strings.Join(errorMessages, "; "))
	}

	return extractedFiles, nil
}
