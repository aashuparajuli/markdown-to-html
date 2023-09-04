#!/bin/bash

#the problem: when a time is displayed with no decimal places: eg: 360ms, the grep does not match, causing an error
cargo build --release
cargo run --release &>/dev/null

total_runtime=0
total_read_file_runtime=0
total_process_text_runtime=0
total_write_file_runtime=0
sample_size=500
for i in $(seq 1 $sample_size);
do
    #run the program, collect the output
    output=$(cargo run --release 2>/dev/null)
    #get the runtime of eachc section
    
    read_file_runtime=$(echo "$output" | ggrep -Po "(?<=read:)\d+[\.]?[\d]*")
    process_text_runtime=$(echo "$output" | ggrep -Po "(?<=parse:)\d+[\.]?[\d]*")
    write_file_runtime=$(echo "$output" |ggrep -Po "(?<=write:)\d+[\.]?[\d]*")
    runtime=$(echo "$output" | ggrep -Po "(?<=total:)\d+[\.]?[\d]*")
    
    #echo "$total_read_file_runtime:$read_file_runtime"
    total_read_file_runtime=$(echo "$total_read_file_runtime + $read_file_runtime" | bc)
    #echo "processed"
    #:'
    total_process_text_runtime=$(echo "$total_process_text_runtime + $process_text_runtime" | bc)
    
    total_write_file_runtime=$(echo "$total_write_file_runtime + $write_file_runtime" | bc)
    total_runtime=$(echo "$total_runtime + $runtime" | bc)
    #'
    #delete the file that was generated
    rm benchmarks/benchmark1/output.html
done

average_read_file_runtime=$(echo "scale=6;$total_read_file_runtime / $sample_size" / 1000 | bc)
average_process_text_runtime=$(echo "scale=4;$total_process_text_runtime / $sample_size" | bc)
average_write_file_runtime=$(echo "scale=4;$total_write_file_runtime / $sample_size" | bc)
average_runtime=$(echo "scale=4;$total_runtime / $sample_size" | bc)

echo "Average runtimes"
echo "$average_read_file_runtime ms"
echo "$average_process_text_runtime ms"
echo "$average_write_file_runtime ms"
echo "$average_runtime ms"
