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

| Section                                | v3   | -   | -   |
| -------------------------------------- | ---- | --- | --- |
| file_io::get_file_lines                | -    | -   | -   |
| file_io::write_line_to_file_true       | -    | -   | -   |
| parse_line_formatting::parse_all_lines | -    | -   | -   |
| Total                                  | .091 | -   | -   |

\* = shown by system as 0

v3: Changed process_italics to use

- using parse_text_formatting::process_bold
- using parse_text_formatting::process_italics

# Time data

calculating by running the execute_benchmark.sh, which runs `cargo run` 200 times and calculates the mean execution time

Times calculated in release mode

| Benchmark # | v1     | v2     | v3    |
| ----------- | ------ | ------ | ----- |
| 1           | 0.104s | 0.094s | v.094 |
| 2           | -      | -      | v2    |
