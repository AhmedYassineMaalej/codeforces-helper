 # codeforces-helper
 
Tired of manually compiling, executing, copying and pasting input when solving competitives programming problems?

compile, run and test your code with 1 command.

# Installation

Simply clone the repository, compile the program and add the binary to your PATH.

# Features
- Intuitive and simple usage (see [usage](#usage))
- Automatic read test input and output from clipboard
- After sucessful test, automatically copy code to clipboard for submission
- Errors are conveyed clearly to the user and are always handled gracefully

# Usage

1. copy test input to clipboard and run `codeforces-helper input`
2. copy test output to clipboard and run `codeforces-helper output`
3. run `codeforces-helper run <SOURCE>`

> [!Tip]
> steps 1 & 2 only have to ran once at the start or if you wish to change the test input/output

For further help, run `codeforces-helper help` to show a list of subcommands
```help
Usage: codeforces-helper <COMMAND>

Commands:
  run     Run code with test input
  input   Set the test's input from the clipboard
  output  Set the test's expected output from the clipboard
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
you can also run `codeforces-helper help <SUBCOMMAND>` to get help about a specific subcommand
```help
> codeforces-helper help run
Run code with test input

Usage: codeforces-helper run [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  path to the C++ file to run

Options:
  -s, --show         Show execution output in terminal
  -i, --interactive  Take interactive input from user
  -c, --compare      Compare execution output with expected output
  -h, --help         Print help
```
# Contributing
Feel free to suggest features, user experience improvements as well as open issues and pull requests

