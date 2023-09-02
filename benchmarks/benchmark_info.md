# Benchmark 1

- 38 KB, 1014 lines, 37672 characters
- Contains: h1, h2, h3, unordered lists, ordered lists, italics, bold
- Does not contain: nested lists

# Benchmark 2

- WIP

# Version info:

v1: Using a Finite State machine to move between states to parse bold. Using a stack to parse italics. Headers and lists were parsed by checking start the first n characters in the line

- using parse_text_formatting::process_bold
- using parse_text_formatting::process_italics

v2: Changes: Using a stack to parse italics

- using parse_text_formatting::process_bold
- using stack::process_italics

v3: Changes: stack::process_italics stores string indices instead of building strings by appending `char` repeatedly. implemented on optimize/process-italics

- using parse_text_formatting::process_bold
- using stack::process_italics

Section-specific time data: as of v3

v4:
Before: parse_line_formatting_parse_lines processed every line and stored them in a Vec<String>. Then the entire Vec<String> was written to a file
Change: parse_line_formatting_parse_lines will processs one line, write it to a file, then process the next line

# Time data

calculating by running the execute_benchmark.sh, which runs `cargo run` 200 times and calculates the mean execution time

### Info

- all times are in milliseconds
- Times calculated using `./execute_benchmark.sh`. which uses `cargo run --release`

| Section                      | v1  | v3  | v4    |
| ---------------------------- | --- | --- | ----- |
| reading lines from the file  | -   | -   | .154  |
| converting markdown to html  | -   | -   | 3.308 |
| writing the html to the file | -   | -   | -     |
| Total                        | -   | -   | 3.462 |

Note: For v4, converting markdown to html and writing html to file are done in a single step
