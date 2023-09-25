# `simil` (the reverse `diff`, but better)

## A command line tool to compare two files and print the lines which are similar in both files, featuring extensive configuration options.

`simil` was built to compare two code files of any language to each other with configuration options to make it fit your needs.<br>
However, it is able to compare any type of text files (`.txt`, `.py`, `.c`, `.cpp`, `.js`, `.ts`, `.toml`, ... ). 

It says similar instead of the same, bc. it can be configured to only compare certain aspects of a line, e.g. ignoring any leading or trailing whitespace.
This comes in handy when comparing code since code style can substancially differ with different coding styles (you can still configure the program to analyze files as is).

## Configuration

### simil.toml

The tool can be configured through a `simil.toml` file.

### TODO: Where must the file be placed? How does simil look for the .toml file?

**Structure:**<br>
*Example for analyzing Python files according to my preferences*
```
[config]
# statements to ignore
ignore = [
    "",  # ignore empty lines
    "else:",
    "\"\"\"",
    "\"\"\")",
    "'''",
    "(", ")",
    "{", "}",
    "[", "]",
    "})",
    "continue",
    "break",
    "return",
    "print()",
    "print(",
    "print(\"\"\"",
]
# ignore lines beginning with ...
ignore_beginning = [
    "import",
    "from"
]
# remove any leading and/or trailing whitespace
trim_whitespace = true
```
### optional command line args

### TODO: provide usage examples

- `--abspath`<br>
  This flag allows you to provide absolute paths to the files you want to analyze.<br>
  By default, the filename arguments provided will be treated as relative paths (relative to the current working directory).
