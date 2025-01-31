package config

import (
	"os"
	"strconv"
	"time"
)

type ServerConfig struct {
	Port            int
	ReadTimeout     time.Duration
	WriteTimeout    time.Duration
	MaxUploadSize   int64
	TempStoragePath string
}

func NewServerConfig() *ServerConfig {
	return &ServerConfig{
		Port:            getEnvInt("SERVER_PORT", 8080),
		ReadTimeout:     getEnvDuration("READ_TIMEOUT", 10*time.Second),
		WriteTimeout:    getEnvDuration("WRITE_TIMEOUT", 10*time.Second),
		MaxUploadSize:   getEnvInt64("MAX_UPLOAD_SIZE", 300*1024*1024), // 300MB
		TempStoragePath: getEnvString("TEMP_STORAGE_PATH", "./tmp"),
	}
}

func getEnvString(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

func getEnvInt(key string, defaultValue int) int {
	valueStr := os.Getenv(key)
	if value, err := strconv.Atoi(valueStr); err == nil {
		return value
	}
	return defaultValue
}

func getEnvInt64(key string, defaultValue int64) int64 {
	valueStr := os.Getenv(key)
	if value, err := strconv.ParseInt(valueStr, 10, 64); err == nil {
		return value
	}
	return defaultValue
}

func getEnvDuration(key string, defaultValue time.Duration) time.Duration {
	valueStr := os.Getenv(key)
	if value, err := time.ParseDuration(valueStr); err == nil {
		return value
	}
	return defaultValue
}
