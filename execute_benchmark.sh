#!/bin/bash

cargo build
total_execution_time=0
sample_size=200
for i in {1..200}
do
    touch benchmarks/benchmark1/output.html
    output=$(command time cargo run 2>&1 >/dev/null)
    execution_time=$(echo "$output" | ggrep -Po "\d+.\d+\s(?=real)")
    total_execution_time=$(echo "$total_execution_time + $execution_time" | bc)
    #delete the file that was generated
    rm benchmarks/benchmark1/output.html
done
average_execution_time=$(echo "scale=3;$total_execution_time / $sample_size" | bc)
echo "$average_execution_time"
