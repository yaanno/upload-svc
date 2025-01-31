package service

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"runtime"
	"sync"

	"go.uber.org/zap"

	"service-upload/svc-go/pkg/models"
)

type FileProcessor struct {
	logger *zap.SugaredLogger
}

func NewFileProcessor(logger *zap.SugaredLogger) *FileProcessor {
	return &FileProcessor{logger: logger}
}

func (fp *FileProcessor) ProcessFiles(ctx context.Context, files []string) ([]models.Actor, error) {
	var wg sync.WaitGroup
	results := make(chan models.Actor, len(files))
	errChan := make(chan error, len(files))

	for _, filename := range files {
		wg.Add(1)
		go func(file string) {
			defer wg.Done()
			
			select {
			case <-ctx.Done():
				return
			default:
				actors, err := fp.processFile(ctx, file)
				if err != nil {
					fp.logger.Errorw("Failed to process file", "filename", file, "error", err)
					errChan <- err
					return
				}

				for _, actor := range actors {
					select {
					case <-ctx.Done():
						return
					case results <- actor:
					}
				}
			}
		}(filename)
	}

	go func() {
		wg.Wait()
		close(results)
		close(errChan)
	}()

	var actors []models.Actor
	var processingErrors []error

	for {
		select {
		case actor, ok := <-results:
			if !ok {
				results = nil
			} else {
				actors = append(actors, actor)
			}
		case err, ok := <-errChan:
			if !ok {
				errChan = nil
			} else {
				processingErrors = append(processingErrors, err)
			}
		}

		if results == nil && errChan == nil {
			break
		}
	}

	if len(processingErrors) > 0 {
		return actors, fmt.Errorf("errors during file processing: %v", processingErrors)
	}

	return actors, nil
}

func (fp *FileProcessor) ProcessFilesOptimized(ctx context.Context, files []string, maxConcurrency int) ([]models.Actor, error) {
	// Validate input
	if len(files) == 0 {
		return nil, nil
	}

	// Limit concurrency to prevent excessive goroutine creation
	if maxConcurrency <= 0 {
		maxConcurrency = runtime.NumCPU()
	}

	// Use a buffered channel for controlled concurrency
	semaphore := make(chan struct{}, maxConcurrency)
	
	// Prepare thread-safe result collection
	var results []models.Actor
	var resultsMutex sync.Mutex
	var wg sync.WaitGroup
	
	// Error handling channel
	errChan := make(chan error, len(files))

	// Process files with controlled concurrency
	for _, filename := range files {
		select {
		case <-ctx.Done():
			// Context cancelled, stop processing
			return nil, ctx.Err()
		default:
			// Acquire semaphore slot
			semaphore <- struct{}{}
			
			wg.Add(1)
			go func(file string) {
				defer func() {
					// Release semaphore slot
					<-semaphore
					wg.Done()
				}()

				// Recover from potential panics in file processing
				defer func() {
					if r := recover(); r != nil {
						fp.logger.Errorf("Panic in file processing: %v", r)
						errChan <- fmt.Errorf("panic processing file %s: %v", file, r)
					}
				}()

				// Process single file
				actors, err := fp.processFile(ctx, file)
				if err != nil {
					errChan <- fmt.Errorf("error processing file %s: %v", file, err)
					return
				}

				// Thread-safe result collection
				resultsMutex.Lock()
				results = append(results, actors...)
				resultsMutex.Unlock()
			}(filename)
		}
	}

	// Wait for all processing to complete
	wg.Wait()
	close(errChan)
	close(semaphore)

	// Collect and return any errors
	var processingErrors []error
	for err := range errChan {
		processingErrors = append(processingErrors, err)
	}

	if len(processingErrors) > 0 {
		return results, fmt.Errorf("errors during file processing: %v", processingErrors)
	}

	return results, nil
}

func (fp *FileProcessor) processFile(ctx context.Context, filename string) ([]models.Actor, error) {
	// Open file with context support
	file, err := os.OpenFile(filename, os.O_RDONLY, 0644)
	if err != nil {
		return nil, fmt.Errorf("failed to open file %s: %w", filename, err)
	}
	defer file.Close()

	var actors []models.Actor
	decoder := json.NewDecoder(file)

	// Try to decode as an array first
	var events []models.GithubEvent
	if err := decoder.Decode(&events); err == nil {
		// Successfully decoded an array
		for _, event := range events {
			actors = append(actors, event.Actor)
		}
		return actors, nil
	}

	// If array decoding fails, reset decoder
	file.Seek(0, 0)
	decoder = json.NewDecoder(file)

	// Streaming JSON decoder to handle individual events
	for {
		select {
		case <-ctx.Done():
			return actors, ctx.Err()
		default:
			var event models.GithubEvent
			if err := decoder.Decode(&event); err != nil {
				if err == io.EOF {
					return actors, nil
				}
				return actors, fmt.Errorf("JSON decoding error in %s: %w", filename, err)
			}

			// Process and collect actors
			actors = append(actors, event.Actor)
		}
	}
}

func (fp *FileProcessor) ProcessEvent(event models.GithubEvent) models.Actor {
	return event.Actor
}
