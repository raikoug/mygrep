use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use colored::*;
use std::io::{self, BufRead};
use regex::Regex;
use std::error::Error;
use std::process;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug, Clone)]
#[command(name = "MyGrep")]
#[command(author = "Riccardo Bella <raikoug@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Grep Equivalent", long_about = None)]
#[command(author, version, about, long_about = None)]
#[command(about, version, after_help = 
    "A regex like implementation for windows.\n\
    \n\
    The pattern is a string to search for in the file.\n\
    The path is the file to search in.\n\
    If no path is given, the program will read from stdin.\n\
    \n\
    The color is the color to use for the pattern found.\n\
    The bold, underline, italic and strike options are used to format the pattern found.\n\
    The line_numbers option is used to show the line number of the pattern found.\n\
    The regex option is used to search for a regex pattern.\n\
    The insensitive option is used to search for a case insensitive pattern.\n\
    The debug option is used to print all the args for debug.\n\
    \n\
    Example:\n\
    \n\
    mygrep pattern file.txt\n\
    mygrep regex_pattern file.txt -R\n\
    mygrep pattern file.txt -I -c magenta\n\
    cat file.txt | mygrep pattern \n\
    cat file.txt | mygrep regex_pattern -R\n\
    \n\
    Exit Codes:
       0: Success
       1: Generic Error
       2: Invalid Regex Pattern
    "
)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: Option<std::path::PathBuf>,

    /// Color to use
    #[arg(short, long, value_enum, default_value_t = Colors::Red)]
    color: Colors,
    
    /// Bold
    #[arg(short, long, default_value_t = true)]
    bold: bool,

    /// Underline
    #[arg(short, long, default_value_t = false)]
    underline: bool,

    /// Italic
    #[arg(short, long, default_value_t = false)]
    italic: bool,

    /// StrikeThrough
    #[arg(short, long, default_value_t = false)]
    strike: bool,

    /// Show LineNumbers
    #[arg(short, long, default_value_t = false)]
    line_numbers: bool,

    /// Pattern is a Regex
    #[arg(short = 'R', long, default_value_t = false)]
    regex: bool,

    /// Case Insensitive
    #[arg(short = 'I', long, default_value_t = false)]
    insensitive: bool,

    /// Debug
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Colors{
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
    BrightRed,
    BrightGreen,
    BrightBlue,
    BrightYellow,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}

fn line_contains_pattern(line: &str, pattern: &str) -> Result<bool, Box<dyn Error>> {
    // return all start indices where pattern is found
    Ok(line.contains(pattern))
}

fn regex_match_line(line: &str, regex_pattern: &str) -> Result<bool, Box<dyn Error>> {
    let re = Regex::new(regex_pattern)?;
    Ok(re.is_match(line))
    
}

fn get_normal_indexes(line: &str, pattern: &str) -> Vec<(usize, usize)> {
    // return a Vec of (start,end) indices where pattern is found
    let mut indexes : Vec<(usize,usize)> = Vec::new();
    line.match_indices(pattern).for_each(|start| {
        indexes.push((start.0, start.0 + pattern.len()));
    });
    indexes
}

fn get_grep_indexes(line: &str, regex_pattern: &str) -> Vec<(usize, usize)>{
    // return a Vec of (start,end) indices where regex pattern match

    let mut indexes : Vec<(usize,usize)> = Vec::new();
    let re = Regex::new(regex_pattern).unwrap();
    re.find_iter(line).for_each(|m| {
        indexes.push((m.start(), m.end()));
    });
    
    indexes
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // print all args for debug
    if args.debug {
        println!("----------------------------");
        println!("args.pattern:      {}", args.pattern);
        println!("args.path:         {:?}", args.path);
        println!("args.color:        {:?}", args.color);
        println!("args.bold:         {}", args.bold);
        println!("args.underline:    {}", args.underline);
        println!("args.italic:       {}", args.italic);
        println!("args.strike:       {}", args.strike);
        println!("args.line_numbers: {}", args.line_numbers);
        println!("args.regex:        {}", args.regex);
        println!("args.insensitive:  {}", args.insensitive);
        println!("args.debug:        {}", args.debug);
        println!("----------------------------");
        println!();
    }

    // init content as mut empty
    let mut content = String::new();
    // if path is not None, do the following
    if let Some(path) = args.path {
        content = std::fs::read_to_string(&path)
            .with_context(|| format!("could not read file `{}`", path.display()))?;
    }
    else{
        // if no path is give we hope to have a stdin as content
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            content.push_str(&line?);
            content.push_str("\n");
        }
    }

    // color must be string to use it in replace
    let color = format!("{:?}", args.color);

    //for (index , line) in content.lines().enumerate() {
    for (index, line) in content.lines().enumerate() {

        let mut search_line = line.to_string().clone();
        let mut pattern_to_search = args.pattern.clone();
        let mut get_indexes_f: fn(&str, &str) -> Vec<(usize, usize)> = get_normal_indexes;

        if args.insensitive {
            search_line = search_line.to_lowercase();
            pattern_to_search = pattern_to_search.to_lowercase();
        }

        let mut search_f: fn(&str, &str) -> Result<bool, Box<dyn Error>> = line_contains_pattern;

        if args.regex {
            search_f = regex_match_line;
            get_indexes_f = get_grep_indexes;
        }
        
        match search_f(&search_line, &pattern_to_search) {
            Ok(match_found) => {
                if match_found {
                    let mut colored_line: String = line.to_string().clone();
                    let indexes = get_indexes_f(&search_line, &pattern_to_search);
                    for (index,end) in indexes {
                        let colored_pattern = line[index..end].color(color.clone()).to_string();
                        if args.bold {
                            colored_line = colored_line.replace(&line[index..end], &colored_pattern.bold().to_string());
                        }
                        if args.underline {
                            colored_line = colored_line.replace(&line[index..end], &colored_pattern.underline().to_string());
                        }
                        if args.italic {
                            colored_line = colored_line.replace(&line[index..end], &colored_pattern.italic().to_string());
                        }
                        if args.strike {
                            colored_line = colored_line.replace(&line[index..end], &colored_pattern.strikethrough().to_string());
                        }
                    }
                    if args.line_numbers {
                        colored_line = format!("{}: {}", index+1, colored_line);
                    }
                    println!("{}", colored_line);
                }
            },
            Err(_) => {
                let error_message = "Pattern is not a valid regex: ".color("red").bold().to_string();
                eprintln!("{error_message} {}", pattern_to_search.color("magenta").bold());
                process::exit(2);
            }
        }
    }

    Ok(())
}
