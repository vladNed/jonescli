# Jones CLI

[![pipeline-jones](https://github.com/Fremen-Solutions/jonescli/actions/workflows/pipeline-jones.yml/badge.svg)](https://github.com/Fremen-Solutions/jonescli/actions/workflows/pipeline-jones.yml)
![Crates.io](https://img.shields.io/crates/v/jones)

# Context

> Note: 🌶 This is a first attempt to Rust programming.

This project is a pathfinder/inspector for Python classes. A very minimal implementation of a CLI tool that helps you find Python classes.
It aims to help you find classes without changing files or view to understand classes and it shows you the methods and arguments of it very
beautiful in the terminal

# Installation

Installing `jones` requires having the latest rust and cargo. 

After that, by simply running this command the `jones` CLI would be installed:
```bash
cargo install jones
```

## Supported Python Projects
| Version | Supported    |
| :---:   | :---:        |
| >=3.5   | True         |
| ==2.7   | False        |

# Usage

Obviously for a better understanding of `jones` capabilities you can always type:

```bash
jones --help
```

This will show you all the flags and arguments that `jones` has


## Search/grep for classes


The `--grep` flag will find all the Python classes containing a keyword given for search. The keyword should be used in cased format as Python classes are usually written in camel case.
For example:

```bash
jones -g Tool
```
This would return all classes that contain the Tool word and would show you in which files are found exactly as the usual grep.

```
> [FOUND MATCHES]
:: class Tool: -> ~/project/src/band.py
:: class ToolMind: -> ~/project/src/golden_ratio.py
```

> Note: This is still in development as it should be renamed to smart search. The smart search will be used to find classes based on a keyword and the context in which the keyword is used
## Showing class features

To display the class methods and arguments just use `jones` without any flag. For example:

```bash
jones Tool
```

This will search for the class tool in the current directory which you called `jones` (if the directory path is not specified). If the directory path is specified `jones` will search there. For example:

```bash
jones Tool ~/band_project
```

Output:
```bash
# Class [Board]
-------
*docstring: Tic Tac Toe board
* inherit -> Tool

# Methods
-------
:: [__init__] -> None
  * self: None
:: [__getitem__] -> ndarray
  * self: None
  * x: int
:: [display] -> None
  * self: None
:: [mark] -> None
  * self: None
  * tag: Markers
  * x: int
  * y: int
```
