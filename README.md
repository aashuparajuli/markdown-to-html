To use the program:

> `cargo run -- input_file output_file`

ex:

> `cargo run -- input/input.txt output/output.txt 1> /dev/null`

to check that the program generates the correct file compared to some reference file

> `chmod +x ./validate.sh` //optional
> `./validate.sh test`

to run benchmarks to see the performance of the file

> `chmod +x ./execute_benchmark.sh` //optional
> `./execute_benchmark.sh`

# Current Features

- provide input and output files using command-line arguments
- parse header tags from File
- parse unordered list from File
- parse ordered lists from File
- parse italics using asterisk from File
- parse bold using asterisks from File
- a bash script to run the file 100 times and average results
- Note: bash script only runs on MacOS. On MacOS: install grep using `brew install grep`
- italics using underscore
- strikethrough
- inline code blocks
- bold using underscore
- italics using underscores
- for bash script: automatically delete created files before and after testing

# Future features

## Short-term

- fix bug in italics with underscore: ' a_a b_a' should remain as plain text and should not be converted
- pass in a folder, process every md folder in the file
- links

## Long-term

- images
- allow headers to be placed within list elements
- use 2 line breaks to separate elements
- nested bullet points
- multiline code blocks
- nested italics and bold
- begin a <p> tag directly after a header, end it at the next header. This could be used for css tagging or JavaScript
- add comments that when parsed, convert into html 'id' or 'class' attributes
- escape characters to treat next formatting token as plain text

## Current implementation details

- expand the lexer and parser to work with italics using underscore, as well as bold, and strikethrough

## Future implementation details

- create test functions that compare the generated file to the correct file
- create more benchmark files
