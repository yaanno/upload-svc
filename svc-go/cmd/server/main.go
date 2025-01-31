package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gin-contrib/pprof"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"

	"service-upload/svc-go/internal/config"
	"service-upload/svc-go/internal/handler"
	"service-upload/svc-go/internal/logger"
	"service-upload/svc-go/internal/service"
)

func main() {
	// Setup configuration
	serverConfig := config.NewServerConfig()

	// Setup logging
	sugaredLogger := logger.NewLogger()
	defer sugaredLogger.Sync()

	// Setup services and handlers
	fileProcessor := service.NewFileProcessor(sugaredLogger)
	uploadHandler := handler.NewUploadHandler(sugaredLogger, serverConfig, fileProcessor)

	// Setup router
	gin.SetMode(gin.TestMode)
	router := gin.New()

	// Add middleware
	router.Use(gin.Recovery())
	router.Use(loggingMiddleware(sugaredLogger))

	// Performance profiling
	pprof.Register(router)

	// Routes
	router.GET("/", func(c *gin.Context) {
		c.JSON(200, gin.H{"message": "Service is running"})
	})
	router.POST("/upload", uploadHandler.HandleUpload)

	// Server configuration
	server := &http.Server{
		Addr:           fmt.Sprintf(":%d", serverConfig.Port),
		Handler:        router,
		ReadTimeout:    serverConfig.ReadTimeout,
		WriteTimeout:   serverConfig.WriteTimeout,
		MaxHeaderBytes: 1 << 20, // 1 MB
	}

	// Start server in a goroutine
	go func() {
		sugaredLogger.Infow("Starting server",
			"port", serverConfig.Port,
			"readTimeout", serverConfig.ReadTimeout,
			"writeTimeout", serverConfig.WriteTimeout,
		)
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			sugaredLogger.Errorw("Server startup failed", "error", err)
			os.Exit(1)
		}
	}()

	// Graceful shutdown
	quit := make(chan os.Signal, 1)
	// Capture interrupt and terminate signals
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)

	// Block until a signal is received
	<-quit
	sugaredLogger.Info("Shutdown signal received, initiating graceful shutdown...")

	// Create a context with a timeout for shutdown
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Attempt to gracefully shutdown the server
	if err := server.Shutdown(ctx); err != nil {
		sugaredLogger.Errorw("Server shutdown failed",
			"error", err,
			"action", "force shutdown",
		)
		// Force shutdown if graceful shutdown fails
		server.Close()
	}

	// Additional cleanup can be added here
	sugaredLogger.Info("Server exiting")
}

// loggingMiddleware creates a middleware for request logging
func loggingMiddleware(logger *zap.SugaredLogger) gin.HandlerFunc {
	return func(c *gin.Context) {
		// Start timer
		start := time.Now()

		// Process request
		c.Next()

		// Log request details
		logger.Infow("HTTP Request",
			"method", c.Request.Method,
			"path", c.Request.URL.Path,
			"status", c.Writer.Status(),
			"latency", time.Since(start),
			"client_ip", c.ClientIP(),
		)
	}
}
