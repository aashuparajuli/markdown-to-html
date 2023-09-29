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

v3: Changes: Use an enum to mark formatted and plaintext on the stack. Store string indices instead of building strings by appending `char` repeatedly. implemented on optimize/process-italics.

- using parse_text_formatting::process_bold
- using stack::process_italics

Section-specific time data: as of v3

v4:
Before: parse_line_formatting_parse_lines processed every line and stored them in a Vec<String>. Then the entire Vec<String> was written to a file
Change: parse_line_formatting_parse_lines will processs one line, write it to a file, then process the next line. file_io::FileAccess is a struct that stores the file to access so that the file is only opened once

v5: Can parse italics with underscore, italics with asterisks , bold with underscore, bold with asterisks, strikethrough, blockquotes, and inline code blocks. Each different parse is performed with a separate function

v6: v5, but using command line arguments to pass in filepathss

# Time data

calculating by running the execute_benchmark.sh, which runs `cargo run --release` 200 times and calculates the mean execution time

### Info

- all times are in milliseconds
- Times calculated using `./execute_benchmark.sh`. which uses `cargo run --release`

| Section                      | v1    | v3    | v4     | v5    | v6    |
| ---------------------------- | ----- | ----- | ------ | ----- | ----- |
| reading lines from the file  | 0.153 | 1.151 | 0.126  | .134  | .284  |
| converting markdown to html  | 1.446 | 1.335 | 2.776  | 4.426 | 4.060 |
| writing the html to the file | 2.150 | 2.134 | -      | -     |
| Total                        | 3.749 | 3.621 | 2.9021 | 4.564 | 4.345 |

Note: For v4, converting markdown to html and writing html to file are done in a single step

# Part 2
v7: original at start of ref-str 
v8: passing in a &str instead of String
- implemented for: italics using underscore


| Section                      | v7  | v8   | -      | -     | v6    |
| ---------------------------- | --- | --- | ------ | ----- | ----- |
| reading lines from the file  | .587   | .537   | -      | -     | .284  |
| converting markdown to html  | 8.417   | 10.114   | 2.776  | 4.426 | 4.060 |
| writing the html to the file | -   | -   | -      | -     |
| Total                        | 9.045   | 10.715   | 2.9021 | 4.564 | 4.345 |

## Adding features

- the progression from v4 to v5

- the below times are averaged over 500 iterations, instead of 200

| Section                              | v4 w/ no additions | + inline code blocks | + strikethrough |
| ------------------------------------ | ------------------ | -------------------- | --------------- |
| reading lines from the file          | .124               | 0.139                | .133            |
| converting to html + writing to file | 2.666              | 2.941                | 3.378           |
| Total                                | 2.790              | 3.080                | 3.511           |

| Section                              | + italics using underscore | + bold using underscore(aka v5) |
| ------------------------------------ | -------------------------- | ------------------------------- |
| reading lines from the file          | 0.129                      | .134                            |
| converting to html + writing to file | 3.756                      | 4.426                           |
| Total                                | 3.885                      | 4.564                           |
