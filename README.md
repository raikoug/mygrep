# MyGrep

MyGrep is a command-line tool written in Rust that mimics the functionality of the Unix `grep` command. It searches for a pattern in a file and displays the lines that contain it.

## Features

- Search for a string pattern in a file or from stdin
- Use regex patterns for advanced searches
- Case insensitive search
- Customize the color and formatting of the pattern found in the output
- Display line numbers of the pattern found
- Debug mode to print all the args for debugging

## Usage

```bash
mygrep pattern file.txt
mygrep regex_pattern file.txt -R
mygrep pattern file.txt -I -c magenta
cat file.txt | mygrep pattern 
cat file.txt | mygrep regex_pattern -R
```

Exit Codes
```
0: Success
1: Generic Error
2: Invalid Regex Pattern
```
Author
```
Riccardo Bella raikoug@gmail.com
```

Version
1.0


