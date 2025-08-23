# Aider Go Benchmark Harness

This directory contains a Go-based benchmark harness for Aider.

## Running Benchmarks

```
go run ./benchmark <run-name> --model gpt-3.5 --edit-format diff --threads 4 \
  --exercises-dir ../polyglot-benchmark
```

Results are stored under `tmp.benchmarks` and include both `results.json` and
`results.csv` files. Use `--stats` with a benchmark directory to summarize a
previous run:

```
go run ./benchmark --stats tmp.benchmarks/2024-07-04-14-32-08--example
```

Passing `--report-url` posts JSON results to a remote server using `net/http`.
