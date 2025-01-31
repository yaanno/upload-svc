package service

import (
	"context"
	"fmt"
	"os"
	"sync"

	jsoniter "github.com/json-iterator/go"
	"go.uber.org/zap"

	"service-upload/svc-go/pkg/models"
)

var json = jsoniter.ConfigCompatibleWithStandardLibrary

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
		go fp.processFile(ctx, filename, &wg, results, errChan)
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
		return nil, fmt.Errorf("errors processing files: %v", processingErrors)
	}

	return actors, nil
}

func (fp *FileProcessor) processFile(ctx context.Context, filename string, wg *sync.WaitGroup, results chan<- models.Actor, errChan chan<- error) {
	defer wg.Done()

	select {
	case <-ctx.Done():
		return
	default:
		file, err := os.Open(filename)
		if err != nil {
			fp.logger.Errorw("Failed to open file", "filename", filename, "error", err)
			errChan <- err
			return
		}
		defer file.Close()

		var events []models.GithubEvent
		decoder := json.NewDecoder(file)
		if err := decoder.Decode(&events); err != nil {
			fp.logger.Errorw("Failed to decode JSON", "filename", filename, "error", err)
			errChan <- err
			return
		}

		for _, event := range events {
			results <- event.Actor
		}
	}
}
