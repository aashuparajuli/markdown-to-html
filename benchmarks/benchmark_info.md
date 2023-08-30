# Benchmark 1

- 10 KB, 254 lines, 37672 characters
- Contains: h1, h2, h3, unordered lists, ordered lists, italics, bold
- Does not contain: nested lists

# Benchmark 2

- WIP

# Version info:

v1: Using a Finite State machine to move between states to parse bold. Using a stack to parse italics. Headers and lists were parsed by checking start the first n characters in the line

Time data

| Benchmark # | v1    | v2  |
| ----------- | ----- | --- |
| 1           | 0.15s | -   |
| 2           | -     | -   |
