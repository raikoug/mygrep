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

mygrep --help
Usage: mygrep.exe [OPTIONS] <PATTERN> [PATH]

Arguments:
  <PATTERN>  The pattern to look for
  [PATH]     The path to the file to read

Options:
  -c, --color <COLOR>  Color to use [default: red] [possible values: red, green, blue, yellow, magenta, cyan, white, black, bright-red, bright-green, bright-blue, bright-yellow, bright-magenta, bright-cyan, bright-white]
  -b, --bold           Bold
  -u, --underline      Underline
  -i, --italic         Italic
  -s, --strike         StrikeThrough
  -l, --line-numbers   Show LineNumbers
  -R, --regex          Pattern is a Regex
  -I, --insensitive    Case Insensitive
  -d, --debug          Debug
  -h, --help           Print help
  -V, --version        Print version
```
## Setting up MyGrep with Windows Environment Variables

To use `mygrep` from any location in the command prompt, you need to add it to your Windows environment variables. Here's how you can do it:

1. Locate the directory where `mygrep` is installed. For example, it might be `C:\Users\YourUsername\mygrep`.

2. Right-click on 'This PC' (or 'My Computer') and choose 'Properties'.

3. Click on 'Advanced system settings'.

4. In the System Properties window that appears, click on the 'Environment Variables...' button.

5. In the Environment Variables window, under 'System variables', find the 'Path' variable, select it, and click 'Edit...'.

6. In the Edit Environment Variable window, click 'New' and then add the path to the `mygrep` directory.

7. Click 'OK' in all windows to apply the changes.

Now, you should be able to use `mygrep` from any location in the command prompt. Just type `mygrep` followed by your commands.

# Exit Codes
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


