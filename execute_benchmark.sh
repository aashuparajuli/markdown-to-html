#!/bin/bash

total_execution_time=0
for i in {1..100}
do
    output=$(command time cargo run 2>&1 >/dev/null)
    execution_time=$(echo "$output" | ggrep -Po "\d+.\d+\s(?=real)" out.txt)
    total_execution_time=$(echo "$total_execution_time + $execution_time" | bc)
done
average_execution_time=$(echo "scale=2;$total_execution_time / 50" | bc)
echo "$average_execution_time"
