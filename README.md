# Current Features

- parse header tags from File
- parse unordered list from File
- parse ordered lists from File
- parse italics using asterisk from File
- parse bold using asterisks from File
- a bash script to run the file 100 times and average results
- Note: bash script only runs on MacOS. On MacOS: install grep using `brew install grep`

# Future features

## Short-term

- pass in filepaths using command-line arguments
- links
- italics using underscore
- strikethrough
- inline code blocks
- for bash script: automatically delete created files before and after testing

## Long-term

- images
- allow headers to be placed within list elements
- use 2 line breaks to separate elements
- nested bullet points
- multiline code blocks
- bold using underscore
- italics using underscores
- nested italics and bold
- begin a <p> tag directly after a header, end it at the next header. This could be used for css tagging or JavaScript
- add comments that when parsed, convert into html 'id' or 'class' attributes
- escape characters to treat next formatting token as plain text
- `command time -l cargo run ` may be useful to gather more detailed information

## Current implementation details

- use a stack instead of Finite State Machine to store elements
- convert stack::process_italics to use a buffer instead of pushing values to a stack repeatedly

## Future implementation details

- files that contain correct output
- create test functions that compare the generated file to the correct file
- separate the testing modules to inside their respective modules
- create more benchmark files

- in process_bold: replace `self.buffer = String::new()` with `self.buffer.clear()`
- check that double underscore are not being coverted into \*\* when parsing md - want to make sure they stay as underscore
