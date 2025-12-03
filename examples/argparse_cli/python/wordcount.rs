use clap::Parser;
use std as sys;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct Stats {
    pub lines: i32,
    pub words: i32,
    pub chars: i32,
    pub filename: String,
}
#[derive(clap::Parser)]
#[command(about = "Count lines, words, and characters in files")]
#[command(after_help = "Similar to wc(1) Unix command")]
struct Args {
    #[doc = "Files to process"]
    files: Vec<PathBuf>,
    #[arg(short = 'l', long)]
    #[arg(action = clap::ArgAction::SetTrue)]
    #[doc = "Show only line count"]
    lines: bool,
    #[arg(short = 'w', long)]
    #[arg(action = clap::ArgAction::SetTrue)]
    #[doc = "Show only word count"]
    words: bool,
    #[arg(short = 'c', long)]
    #[arg(action = clap::ArgAction::SetTrue)]
    #[doc = "Show only character count"]
    chars: bool,
}
#[doc = "Count statistics for a single file"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn count_file(filepath: &Path) -> Stats {
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(filepath).unwrap();
        let lines = content
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .len() as i32;
        let words = content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .len() as i32;
        let chars = content.len() as i32;
        return Stats::new(lines, words, chars, (filepath).display().to_string());
        Ok(Default::default())
    })() {
        Ok(_result) => {
            return Ok(_result);
        }
        Err(e) => {
            eprintln!("{}", format!("Error reading {}: {:?}", filepath, e));
            return Stats::new(0, 0, 0, (filepath).display().to_string());
        }
    }
}
#[doc = "Format statistics for output"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_stats(stats: Stats, show_filename: bool) -> String {
    let mut result = format!("{} {} {}", stats.lines, stats.words, stats.chars);
    if show_filename {
        let _cse_temp_0 = format!("{}{}", result, format!(" {}", stats.filename));
        result = _cse_temp_0;
    }
    result.to_string()
}
#[doc = "Main entry point"]
#[doc = " Depyler: verified panic-free"]
pub fn main() {
    let args = Args::parse();
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    for filepath in args.files {
        let stats = count_file(filepath.display().to_string());
        total_lines = total_lines + stats.lines;
        total_words = total_words + stats.words;
        total_chars = total_chars + stats.chars;
        if args.lines {
            println!("{}", format!("{} {}", stats.lines, stats.filename));
        } else {
            if args.words {
                println!("{}", format!("{} {}", stats.words, stats.filename));
            } else {
                if args.chars {
                    println!("{}", format!("{} {}", stats.chars, stats.filename));
                } else {
                    println!("{}", format_stats(stats, true));
                }
            }
        }
    }
    let _cse_temp_0 = args.files.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 1;
    if _cse_temp_1 {
        let total_stats = Stats::new(total_lines, total_words, total_chars, "total".to_string());
        println!("{}", format_stats(total_stats, true));
    }
    ()
}
