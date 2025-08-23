package main

import (
	"bytes"
	"encoding/csv"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sync"
	"testing"
	"time"
)

// CaseResult captures the outcome of a single benchmark case.
type CaseResult struct {
	Name        string        `json:"name"`
	Passed      bool          `json:"passed"`
	Error       string        `json:"error,omitempty"`
	Duration    time.Duration `json:"duration"`
	Allocations float64       `json:"allocs"`
}

// stats aggregates results for reporting.
type stats struct {
	Total    int     `json:"total"`
	Passed   int     `json:"passed"`
	Failed   int     `json:"failed"`
	PassRate float64 `json:"pass_rate"`
}

func main() {
	var (
		statsDir          = flag.String("stats", "", "summarize results for a directory")
		model             = flag.String("model", "", "LLM model name")
		editFormat        = flag.String("edit-format", "", "edit format")
		threads           = flag.Int("threads", runtime.NumCPU(), "number of parallel threads")
		numTests          = flag.Int("num-tests", 0, "limit number of tests to run")
		keywords          = flag.String("keywords", "", "only run tests containing keywords")
		readModelSettings = flag.String("read-model-settings", "", "path to model settings")
		exercisesDir      = flag.String("exercises-dir", "polyglot-benchmark", "path to exercises")
		reportURL         = flag.String("report-url", "", "POST JSON results to this URL")
	)
	flag.Parse()

	if *statsDir != "" {
		if err := statsMode(*statsDir); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		return
	}

	if flag.NArg() < 1 {
		fmt.Fprintln(os.Stderr, "run name required")
		os.Exit(1)
	}
	runName := flag.Arg(0)

	results, err := runBench(*exercisesDir, *threads, *numTests, *keywords)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	if err := writeResults(runName, results); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	if *reportURL != "" {
		_ = postResults(*reportURL, results)
	}

	_ = model
	_ = editFormat
	_ = readModelSettings
}

// runBench discovers test cases and executes them using the testing package.
func runBench(exercises string, threads, limit int, keywords string) ([]CaseResult, error) {
	testDirs := []string{}
	err := filepath.Walk(exercises, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			matches, _ := filepath.Glob(filepath.Join(path, "*_test.go"))
			if len(matches) > 0 {
				if keywords == "" || filepath.Base(path) == keywords {
					testDirs = append(testDirs, path)
				}
			}
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	if limit > 0 && len(testDirs) > limit {
		testDirs = testDirs[:limit]
	}
	if len(testDirs) == 0 {
		return nil, errors.New("no test directories found")
	}

	results := make([]CaseResult, 0, len(testDirs))
	var mu sync.Mutex
	var wg sync.WaitGroup

	sema := make(chan struct{}, threads)
	for _, dir := range testDirs {
		dir := dir
		name := filepath.Base(dir)
		wg.Add(1)
		go func() {
			defer wg.Done()
			sema <- struct{}{}
			defer func() { <-sema }()

			start := time.Now()
			cmd := exec.Command("go", "test", dir)
			var out []byte
			var err error
			allocs := testing.AllocsPerRun(1, func() {
				out, err = cmd.CombinedOutput()
			})
			duration := time.Since(start)
			passed := err == nil
			mu.Lock()
			results = append(results, CaseResult{Name: name, Passed: passed, Error: string(out), Duration: duration, Allocations: allocs})
			mu.Unlock()
		}()
	}
	wg.Wait()
	return results, nil
}

// writeResults writes CSV and JSON outputs to tmp.benchmarks.
func writeResults(runName string, results []CaseResult) error {
	dir := filepath.Join("tmp.benchmarks", time.Now().Format("2006-01-02-15-04-05")+"--"+runName)
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}

	// JSON
	jfile := filepath.Join(dir, "results.json")
	jf, err := os.Create(jfile)
	if err != nil {
		return err
	}
	defer jf.Close()
	if err := json.NewEncoder(jf).Encode(results); err != nil {
		return err
	}

	// CSV
	cfile := filepath.Join(dir, "results.csv")
	cf, err := os.Create(cfile)
	if err != nil {
		return err
	}
	defer cf.Close()
	w := csv.NewWriter(cf)
	defer w.Flush()
	w.Write([]string{"name", "passed", "error", "duration"})
	for _, r := range results {
		w.Write([]string{r.Name, fmt.Sprint(r.Passed), r.Error, r.Duration.String()})
	}
	return nil
}

// statsMode reads a results directory and prints summary stats.
func statsMode(dir string) error {
	data, err := os.ReadFile(filepath.Join(dir, "results.json"))
	if err != nil {
		return err
	}
	var results []CaseResult
	if err := json.Unmarshal(data, &results); err != nil {
		return err
	}
	s := aggregate(results)
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	return enc.Encode(s)
}

func aggregate(results []CaseResult) stats {
	s := stats{Total: len(results)}
	for _, r := range results {
		if r.Passed {
			s.Passed++
		} else {
			s.Failed++
		}
	}
	if s.Total > 0 {
		s.PassRate = float64(s.Passed) / float64(s.Total) * 100
	}
	return s
}

// postResults sends JSON results to a remote server.
func postResults(url string, results []CaseResult) error {
	body, err := json.Marshal(results)
	if err != nil {
		return err
	}
	resp, err := http.Post(url, "application/json", bytes.NewReader(body))
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	return nil
}
