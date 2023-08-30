# Current Features

- parse header tags from File
- parse unordered list from File
- parse ordered lists from File
- parse italics using asterisk from File
- parse bold using underscore from File

# Future features

## Short-term

- links
- italics using underscore
- pass in filepaths using command-line arguments

## Long-term

- create a bash script to run the file 100 times and average results, using command-line arguments for filepaths
  - for bash script: use `command time`, not time
  - automatically delete created files before and after testing
- images
- allow headers to be placed within list elements
- use 2 line breaks to separate elements
- nested bullet points
- bold using asterisks
- italics using underscores
- nested italics and bold
- begin a <p> tag directly after a header, end it at the next header. This could be used for css tagging or JavaScript
- add comments that when parsed, convert into html 'id' or 'class' attributes
- escape characters to treat next formatting token as plain text

## Implementation future work to add

- files that contain correct output
- create test functions that compare the generated file to the correct file
- time how long it takes to convert certain
- separate the testing modules to inside their respective modules
- automatically delete the file before/after tests
- consider using a stack instead of Finite State Machine to store elements
