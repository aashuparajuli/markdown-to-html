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
- using stack::process_italics

v3: Changed stack::process_italics to store string indices instead of building strings by appending char. implemented on optimize/process-italics

- using parse_text_formatting::process_bold
- using stack::process_italics

Section-specific time data: as of v3

# Time data

calculating by running the execute_benchmark.sh, which runs `cargo run --release` 200 times and calculates the mean execution time

### Info

- all times are in milliseconds
- Times calculated using `cargo run --release`

| Section                      | v1    | v3    |
| ---------------------------- | ----- | ----- |
| reading lines from the file  | 0.153 | 1.151 |
| converting markdown to html  | 1.446 | 1.335 |
| writing the html to the file | 2.150 | 2.134 |
| Total                        | 3.749 | 3.621 |
