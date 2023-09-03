cargo run -- input/input.txt output/output.txt 1> /dev/null
diff output/output.txt output/correct.txt | wc -l
