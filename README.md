# `simil` (the reverse `diff`, but better)

## A command line tool to compare two files and print the lines which are similar in both files.

It says similar instead of the same, bc. it only takes the contents of the file, ignoring any leading or trailing whitespace.<br>
This makes it easy to compare code since code style can substancially differ with different coding styles (you can still configure the program to not ignore whitespace).

## Configuration

### The tool can be configured through a `simil.toml` file, placed in the same directory as the executable.

