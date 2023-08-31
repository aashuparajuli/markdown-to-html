#!/bin/bash

total_execution_time=0
sample_size=100
for i in {1..100}
do
    output=$(command time cargo run 2>&1 >/dev/null)
    execution_time=$(echo "$output" | ggrep -Po "\d+.\d+\s(?=real)")
    total_execution_time=$(echo "$total_execution_time + $execution_time" | bc)
done
average_execution_time=$(echo "scale=2;$total_execution_time / $sample_size" | bc)
echo "$average_execution_time"
