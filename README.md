# Jones CLI

[![pipeline-jones](https://github.com/Fremen-Solutions/jonescli/actions/workflows/pipeline-jones.yml/badge.svg)](https://github.com/Fremen-Solutions/jonescli/actions/workflows/pipeline-jones.yml)

# Context

> Note: 🌶 This is a first attempt to Rust programming.

This project is a pathfinder/inspector for Python classes. A very minimal implementation of a CLI tool that helps you find Python classes.
It aims to help you find classes without changing files or view to understand classes and it shows you the methods and arguments of it very
beautiful in the terminal

# Usage

Obviously for better understanding of `jones` capabilities you can always type:

```bash
$ jones --help
```

This will show you all the flags and arguments that jones has


## Search/grep for classes


The `--grep` flag will find all the Python classes containing a keyword given for search. The keyword should be used in cased format as Python classes are usually written in camel case.
For example:

```bash
$ jones -g Tool
```
This would return all classes that contain the Tool word and would show you in which files are found exactly as the usual grep.

```
> [FOUND MATCHES]
:: class Tool: -> ~/project/src/band.py
:: class ToolMind: -> ~/project/src/golden_ratio.py
```

> Note: This is still in development as it should be renamed to smart search. The smart search will be used to find classes based on a keyword and the context in which the keyword is used
## Showing classes features

To display the class methods and arguments just use `jones` without any flag. For example:

```bash
$ jones Tool
```

This will search for the class tool in the current directory on which you called jones (if the directory path is not specified). If the directory path is specified `jones` will search there. For example:

```bash
$ jones Tool ~/band_project
```
