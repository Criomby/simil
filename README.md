# `simil` (the reverse `diff`, but better)

## A command line tool to compare two files and print the lines which are similar in both files, featuring extensive configuration options.

`simil` was built to compare two code files of any programming language to each other with configuration options to make it fit your needs.<br>
However, it can compare any type of text files (`.txt`, `.py`, `.c`, `.cpp`, `.js`, `.ts`, `.toml`, ... ). 

It says similar instead of the same, bc. it can be configured to only compare certain aspects of a line, e.g. ignoring any leading or trailing whitespace.
This comes in handy when comparing code since code style can substancially differ with different coding styles (you can still configure the program to analyze files as-is).

It is comparable to the `comm` command on *nix systems with options -12 (only print lines found in both files),<br>
but `simil` allows you to configure the comparison extensively for more accurate results with complex text (like code files) and the output gives you more info about the findings (line numbers of where matches are found in each file and matching blocks of text).

## Configuration

### command line args

```
Usage: simil [--abspath] [--noconf [[--ignore-empty] [--trim]] [...] file1 file2

positional arguments:
    file

options:
    -h, --help      Show this help message and exit
    -v, --version   Show version number and exit
    --abspath       Using absolute filepaths (relative to cwd by default)
    --global-conf   Use global config (ignore local)
    --local-conf    Use local config (ignore global)
    --noconf        Do not use simil.toml config
        + --ignore-empty  Omit empty lines in output
        + --trim          Trim whitespace
```

- `--abspath`<br>
This flag allows you to provide absolute paths to the files you want to analyze.<br>
By default, the filename arguments provided will be treated as relative paths (relative to the current working directory).
- `--global-conf`<br>
Use the global config (in exe dir), ignoring the local config file (cwd or parents)
- `--local-conf`<br>
Use the local config (in cwd or any parent dir), ignoring the global config (in exe dir)
- `--noconf`<br>
Ignore any simil.toml file and configure simil from the command line (or use to compare files as-is)
    - `--ignore-empty`<br>
    Ignore empty lines in output
    - `--trim`<br>
    Trim any leading & trailing whitespace from lines for the comparison

<br>

### simil.toml

The tool can be configured through a `simil.toml` file.

___
*If you do not want to use a config file, you **HAVE** to set the option `--noconfig` (or an error will be shown).<br>
You can then set additional options to configure simil from the command line.*
___

You have two options setting a simil.toml:
1. Place toml file in same dir as the exe to set a *system-wide config* for simil
2. Place toml file in the project dir (cwd, dir from where you will invoke simil from)<br>
or any parent dir to set a config on a *per-project (per-dir & sub-dirs)* basis.

If you set both, the global config file (in the exe dir) will be used<br>
OR you can set which config to use if both exist with `--global-conf` or `--local-conf`.

**Structure:**<br>

Required structure for `simil.toml`:
```
[config]

# lines/statements to ignore
ignore = []

# ignore lines beginning with ...
ignore_beginning = []

# remove any leading & trailing whitespace
trim_whitespace = false
```

***Example for analyzing Python files according to my preferences:***
```
[config]

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

ignore_beginning = [
    "import",
    "from"
]

trim_whitespace = true
```
