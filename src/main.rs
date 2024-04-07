use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use colored::*;
use std::io::{self, BufRead};
use regex::Regex;
use std::error::Error;
use std::{process, vec};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug, Clone)]
#[command(name = "MyGrep")]
#[command(author = "Riccardo Bella <raikoug@gmail.com>")]
#[command(version = "1.1.0")]
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
    The after option is used to print the number of lines after the match (not compatible with section).\n\
    The before option is used to print the number of lines before the match (not compatible with section).\n\
    The section option is used to print the section (same indentation or more) of the file where the pattern is found. (Not compatible with after and or before)\n\
    The tabs_c option is used to set the number of spaces for a tab. Default is 4.\n\
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
    #[arg(short = 'B', long, default_value_t = true)]
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

    /// After
    #[arg(short, long, default_value_t = 0)]
    after: usize,

    /// Before
    #[arg(short, long, default_value_t = 0)]
    before: usize,

    /// Section
    #[arg(short = 'S', long, default_value_t = false)]
    section: bool,

    /// Tabs count
    #[arg(short, long, default_value_t = 4)]
    tabs_c: usize,

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

fn indentation(line: &str, tabs_c: usize) -> usize {
    // return the number of spaces at the beginning of the line
    let spaces = line.chars().take_while(|&c| c == ' ').count();
    let tabs = line.chars().take_while(|&c| c == '\t').count() * tabs_c;

    spaces + tabs

}

fn main() -> Result<()> {
    let mut args = Cli::parse();

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
        println!("args.after:        {}", args.after);
        println!("args.before:       {}", args.before);
        println!("args.section:      {}", args.section);
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

    // If section is true and before and after are != 0, print warning and set after and before to 0
    if args.section && (args.before != 0 || args.after != 0) {
        eprintln!("Section is not compatible with after and or before. Ignoring After and or Before.");
        args.after = 0;
        args.before = 0;
    }
    // the section behaviour could print multiple time the same section, this avoid this behavior.
    let mut sections_to_print: Vec<usize> = vec![];
    // init a found_rows to keep colored lines to be printed with found pattern, a tuple of index and string
    let mut found_rows: Vec<(usize, String)> = vec![];

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

                    // Before lines, args.before is the number of lines to print before the match, default 0
                    if args.before > 0 {
                        if index >= 1 {
                            let mut before_indexes = vec![];
                            for i in 1..=args.before {
                                if index < i {
                                    break;
                                }
                                let tmp_index = index - i;

                                before_indexes.push(tmp_index);
                            }
                            before_indexes.reverse();
                            for i in before_indexes {
                                let before_line = content.lines().nth(i).unwrap();
                                println!("{}", before_line);
                            }
                        }
                    }

                    if !args.section{
                        println!("{}", colored_line);
                    }

                    // After lines, args.after is the number of lines to print after the match, default 0
                    if args.after > 0 {
                        for i in 1..=args.after {
                            if index + i < content.lines().count() {
                                let after_line = content.lines().nth(index + i).unwrap();
                                println!("{}", after_line);
                            }
                        }
                    }

                    // Section, if true print the section.
                    //   Section is the same indentation or more of the line where the pattern is found.
                    //   Section can start before the finding
                    if args.section {
                        // init indentation
                        let starting_indentation = indentation(&line, args.tabs_c);
                        sections_to_print.push(index);
                        // keep the index where pattern is found along with colored string
                        found_rows.push((index, colored_line));
                        
                        // scroll backwards indexes, break if indentation is <= starting_indentation
                        for i in (0..index).rev() {
                            let tmp_line = content.lines().nth(i).unwrap();
                            let tmp_indentation = indentation(&tmp_line, args.tabs_c);
                            if tmp_indentation < starting_indentation {
                                // if indentation is <= starting_indentation, break
                                //   get the index as head of the section
                                sections_to_print.push(i);
                                break;
                            }
                            sections_to_print.push(i);
                        }

                        // scroll forward indexes, break if indentation is < starting_indentation
                        for i in index..content.lines().count() {
                            let tmp_line = content.lines().nth(i).unwrap();
                            let tmp_indentation = indentation(&tmp_line, args.tabs_c);
                            if tmp_indentation < starting_indentation {
                                // if indentation is < starting_indentation, break
                                break;
                            }
                            sections_to_print.push(i);
                        }

                    }
                }

            },
            Err(_) => {
                let error_message = "Pattern is not a valid regex: ".color("red").bold().to_string();
                eprintln!("{error_message} {}", pattern_to_search.color("magenta").bold());
                process::exit(2);
            }
        }
    }

    if args.section{
        // sections_to_print is a list olf indexes to print, but are unordered and maybe duplicate.
        //   we need to sort and remove duplicates
        sections_to_print.sort();
        sections_to_print.dedup();

        // print all indexes in sections_to_print
        for index in sections_to_print {
            // if index in found_rows, print the colored line instead of the normal line
            if found_rows.iter().any(|x| x.0 == index) {
                let colored_line = found_rows.iter().find(|&x| x.0 == index).unwrap().1.clone();
                println!("{}", colored_line);
                continue;
            }

            let line = content.lines().nth(index).unwrap();
            if args.line_numbers {
                println!("{}: {}", index+1, line);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
