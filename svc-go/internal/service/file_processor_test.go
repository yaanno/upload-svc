package service

import (
	"context"
	"encoding/json"
	"fmt"
	"math/rand"
	"os"
	"path/filepath"
	"runtime"
	"runtime/pprof"
	"sync"
	"testing"
	"time"

	"service-upload/svc-go/pkg/models"

	"go.uber.org/zap"
)

func generateTestEvents(count int) []models.GithubEvent {
	events := make([]models.GithubEvent, count)
	for i := 0; i < count; i++ {
		events[i] = models.GithubEvent{
			Actor: models.Actor{
				ID:    rand.Int(),
				Login: fmt.Sprintf("user_%d", i),
			},
			Type: "TestEvent",
		}
	}
	return events
}

func BenchmarkEventProcessing(b *testing.B) {
	// Create a logger
	logger, _ := zap.NewProduction()
	sugaredLogger := logger.Sugar()
	defer logger.Sync()

	// Benchmark configurations
	eventCounts := []int{100, 1000, 10000, 100000}

	for _, eventCount := range eventCounts {
		// Generate test events
		events := generateTestEvents(eventCount)

		b.Run(fmt.Sprintf("Sequential_%d_events", eventCount), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				b.StopTimer()
				fp := NewFileProcessor(sugaredLogger)
				results := make([]models.Actor, 0, len(events))
				b.StartTimer()

				for _, event := range events {
					results = append(results, fp.ProcessEvent(event))
				}
			}
		})

		b.Run(fmt.Sprintf("ChannelParallel_%d_events", eventCount), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				b.StopTimer()
				fp := NewFileProcessor(sugaredLogger)
				results := make(chan models.Actor, len(events))
				b.StartTimer()

				var wg sync.WaitGroup
				numWorkers := runtime.NumCPU()

				eventChan := make(chan models.GithubEvent, len(events))
				for _, event := range events {
					eventChan <- event
				}
				close(eventChan)

				for w := 0; w < numWorkers; w++ {
					wg.Add(1)
					go func() {
						defer wg.Done()
						for event := range eventChan {
							results <- fp.ProcessEvent(event)
						}
					}()
				}

				wg.Wait()
				close(results)
			}
		})

		b.Run(fmt.Sprintf("WorkerPool_%d_events", eventCount), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				b.StopTimer()
				fp := NewFileProcessor(sugaredLogger)
				ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
				defer cancel()

				results := make(chan models.Actor, len(events))
				errChan := make(chan error, 1)
				var wg sync.WaitGroup
				b.StartTimer()

				numWorkers := runtime.NumCPU()
				eventChan := make(chan models.GithubEvent, len(events))

				// Start worker goroutines
				for w := 0; w < numWorkers; w++ {
					wg.Add(1)
					go func() {
						defer wg.Done()
						for event := range eventChan {
							select {
							case <-ctx.Done():
								return
							case results <- fp.ProcessEvent(event):
							}
						}
					}()
				}

				// Send events to workers
				for _, event := range events {
					eventChan <- event
				}
				close(eventChan)

				// Wait for all event workers to complete
				wg.Wait()
				close(results)
				close(errChan)
			}
		})
	}
}

func BenchmarkMemoryConsumption(b *testing.B) {
	// Setup logger
	logger, _ := zap.NewProduction()
	defer logger.Sync()
	sugaredLogger := logger.Sugar()

	// Define event count scenarios
	eventCounts := []int{100, 1000, 10000, 100000}

	for _, eventCount := range eventCounts {
		b.Run(fmt.Sprintf("Sequential_%d_events", eventCount), func(b *testing.B) {
			// Enable memory profiling
			b.ReportAllocs()

			// Generate test events
			events := generateTestEvents(eventCount)

			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				fp := NewFileProcessor(sugaredLogger)
				results := make([]models.Actor, 0, len(events))

				for _, event := range events {
					results = append(results, fp.ProcessEvent(event))
				}
			}
		})

		b.Run(fmt.Sprintf("ChannelParallel_%d_events", eventCount), func(b *testing.B) {
			// Enable memory profiling
			b.ReportAllocs()

			// Generate test events
			events := generateTestEvents(eventCount)

			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				fp := NewFileProcessor(sugaredLogger)
				results := make(chan models.Actor, len(events))
				var wg sync.WaitGroup

				for _, event := range events {
					wg.Add(1)
					go func(evt models.GithubEvent) {
						defer wg.Done()
						results <- fp.ProcessEvent(evt)
					}(event)
				}

				go func() {
					wg.Wait()
					close(results)
				}()

				// Consume results
				for range results {
				}
			}
		})
	}
}

// Optional: Memory Profile Helper Function
func writeMemProfile(b *testing.B) {
	f, err := os.Create("mem.prof")
	if err != nil {
		b.Fatal("could not create memory profile: ", err)
	}
	defer f.Close()

	runtime.GC() // get up-to-date statistics
	if err := pprof.WriteHeapProfile(f); err != nil {
		b.Fatal("could not write memory profile: ", err)
	}
}

// Additional test to validate the processing logic
func TestFileProcessor_ProcessEvents(t *testing.T) {
	logger, _ := zap.NewProduction()
	sugaredLogger := logger.Sugar()
	defer logger.Sync()

	fp := NewFileProcessor(sugaredLogger)
	events := generateTestEvents(1000)

	ctx := context.Background()

	// Create a temporary file with test events
	tempFile, err := os.CreateTemp("", "events*.json")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	defer os.Remove(tempFile.Name())

	// Write events to temp file
	if err := json.NewEncoder(tempFile).Encode(events); err != nil {
		t.Fatalf("Failed to write events to temp file: %v", err)
	}
	tempFile.Close()

	// Process the file
	processedActors, err := fp.ProcessFiles(ctx, []string{tempFile.Name()})
	if err != nil {
		t.Fatalf("Failed to process file: %v", err)
	}

	// Validate results
	if len(processedActors) != len(events) {
		t.Errorf("Expected %d actors, got %d", len(events), len(processedActors))
	}
}

func TestFileProcessor_ProcessFilesOptimized(t *testing.T) {
	logger, _ := zap.NewProduction()
	sugaredLogger := logger.Sugar()
	defer logger.Sync()

	// Create temporary test files
	tempDir, err := os.MkdirTemp("", "fileprocessor-test-")
	if err != nil {
		t.Fatalf("Failed to create temp directory: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Generate test files
	testFiles := make([]string, 5)
	for i := 0; i < 5; i++ {
		filename := filepath.Join(tempDir, fmt.Sprintf("test_file_%d.json", i))
		events := generateTestEvents(100)

		file, err := os.Create(filename)
		if err != nil {
			t.Fatalf("Failed to create test file: %v", err)
		}

		encoder := json.NewEncoder(file)
		for _, event := range events {
			if err := encoder.Encode(event); err != nil {
				file.Close()
				t.Fatalf("Failed to write test event: %v", err)
			}
		}
		file.Close()

		testFiles[i] = filename
	}

	// Create file processor
	fp := NewFileProcessor(sugaredLogger)

	// Test optimized processing
	ctx := context.Background()
	results, err := fp.ProcessFilesOptimized(ctx, testFiles, 0)

	if err != nil {
		t.Fatalf("ProcessFilesOptimized returned error: %v", err)
	}

	// Verify results
	expectedTotalEvents := 100 * 5
	if len(results) != expectedTotalEvents {
		t.Errorf("Expected %d total actors, got %d", expectedTotalEvents, len(results))
	}

	// Verify context cancellation
	cancelCtx, cancel := context.WithCancel(context.Background())
	cancel() // Immediately cancel

	_, err = fp.ProcessFilesOptimized(cancelCtx, testFiles, 0)
	if err == nil {
		t.Error("Expected error from cancelled context, got nil")
	}
}
