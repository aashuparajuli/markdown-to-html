# Benchmark 1

- 10 KB, 254 lines, 37672 characters
- Contains: h1, h2, h3, unordered lists, ordered lists, italics, bold
- Does not contain: nested lists

# Benchmark 2

- WIP

# Version info:

v1: Using a Finite State machine to move between states to parse bold. Using a stack to parse italics. Headers and lists were parsed by checking start the first n characters in the line

- using parse_text_formatting::process_bold
- using parse_text_formatting::process_italics

v2: Using a stack to parse italics

- using parse_text_formatting::process_bold
- using parse_text_formatting::process_italics

v3: Changed process_italics to use 
- using parse_text_formatting::process_bold
- using parse_text_formatting::process_italics

# Time data

calculating by running the execute_benchmark.sh, which runs `cargo run` 100 times and calculates the mean execution ti

| Benchmark # | v1    | v2   |
| ----------- | ----- | ---- |
| 1           | 0.10s | 0.10 |
| 2           | -     | -    |
