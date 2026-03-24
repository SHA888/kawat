#!/bin/bash
# Local benchmark runner with comparison capabilities

set -e

BENCHMARK_DIR="crates/kawat-xpath"
RESULTS_DIR="benchmark-results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "Running kawat benchmarks..."

# Create results directory
mkdir -p "$RESULTS_DIR"

# Run benchmarks
cd "$BENCHMARK_DIR"
cargo bench --all -- --output-format json > "../$RESULTS_DIR/benchmark_$TIMESTAMP.json"

# Process results
echo "Processing results..."
cat "../$RESULTS_DIR/benchmark_$TIMESTAMP.json" | jq -r '
  .report[] | 
  "\(.group.name)/\(.query.id): \(.query.mean_ns | tonumber | . / 1_000_000) ms ± \(.query.stddev_ns | tonumber | . / 1_000_000) ms"
' > "../$RESULTS_DIR/summary_$TIMESTAMP.txt"

# Create CSV for tracking
echo "benchmark,mean_ns,stddev_ns" > "../$RESULTS_DIR/results_$TIMESTAMP.csv"
cat "../$RESULTS_DIR/benchmark_$TIMESTAMP.json" | jq -r '
  .report[] | 
  "\(.group.name)/\(.query.id),\(.query.mean_ns),\(.query.stddev_ns)"
' >> "../$RESULTS_DIR/results_$TIMESTAMP.csv"

# Display summary
echo ""
echo "=== Benchmark Results ==="
cat "../$RESULTS_DIR/summary_$TIMESTAMP.txt"

# Compare with previous results if available
cd ..
PREVIOUS=$(find "$RESULTS_DIR" -name "benchmark_*.json" | sort | tail -n 2 | head -n 1)

if [ -n "$PREVIOUS" ] && [ "$PREVIOUS" != "$RESULTS_DIR/benchmark_$TIMESTAMP.json" ]; then
  echo ""
  echo "=== Comparison with Previous ==="
  
  # Parse current and previous
  cat "$RESULTS_DIR/benchmark_$TIMESTAMP.json" | jq -r '
    .report[] | 
    "\(.group.name)/\(.query.id),\(.query.mean_ns)"
  ' > "$RESULTS_DIR/current.csv"
  
  cat "$PREVIOUS" | jq -r '
    .report[] | 
    "\(.group.name)/\(.query.id),\(.query.mean_ns)"
  ' > "$RESULTS_DIR/previous.csv"
  
  # Compare
  echo "| Benchmark | Current (ms) | Previous (ms) | Change |"
  echo "|-----------|--------------|---------------|--------|"
  
  join -t, -1 1 -2 1 <(sort "$RESULTS_DIR/current.csv") <(sort "$RESULTS_DIR/previous.csv") | while IFS=, read -r name current previous; do
    current_ms=$(echo "$current / 1_000_000" | bc -l)
    prev_ms=$(echo "$previous / 1_000_000" | bc -l)
    change=$(echo "scale=2; ($current_ms - $prev_ms) / $prev_ms * 100" | bc -l)
    
    if (( $(echo "$change < -10" | bc -l) )); then
      status="🟢 $change%"
    elif (( $(echo "$change > 10" | bc -l) )); then
      status="🔴 $change%"
    else
      status="⚪ $change%"
    fi
    
    echo "| $name | $current_ms | $prev_ms | $status |"
  done
else
  echo "No previous benchmark data found for comparison"
fi

echo ""
echo "Results saved to $RESULTS_DIR/"
echo "Latest: benchmark_$TIMESTAMP.json"
