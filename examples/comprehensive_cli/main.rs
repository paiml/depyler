use clap::Parser;
use serde_json as json;
use std::path::PathBuf;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    #[derive(clap::Parser)]
    #[command(about = "Comprehensive CLI application testing all ArgumentParser features")]
    #[command(after_help = "Example: ./cli input.txt --verbose -o output.txt")]
    struct Args {
        #[doc = "Input file to process"]
        input_file: String,
        #[arg(short = 'v')]
        #[doc = "Verbose mode(short only)"]
        v: Option<String>,
        #[arg(long)]
        #[arg(action = clap::ArgAction::SetTrue)]
        #[doc = "Debug mode(long only)"]
        debug: bool,
        #[arg(short = 'o', long)]
        #[doc = "Output file(short + long)"]
        output: Option<String>,
        #[arg(short = 'q', long)]
        #[arg(action = clap::ArgAction::SetTrue)]
        #[doc = "Quiet mode"]
        quiet: bool,
        #[arg(long = "no-color")]
        #[arg(action = clap::ArgAction::SetFalse)]
        #[arg(default_value_t = true)]
        #[doc = "Disable colors"]
        color: bool,
        #[arg(short = 'V')]
        #[arg(default_value = "0")]
        #[arg(action = clap::ArgAction::Count)]
        #[doc = "Verbosity level"]
        V: u8,
        #[arg(short = 'I', long)]
        #[doc = "Include paths"]
        include: Vec<String>,
        #[doc = "Extra files(1 or more)"]
        extras: Vec<String>,
        #[arg(long)]
        #[doc = "Optional args(0 or more)"]
        optional: Option<Vec<String>>,
        #[arg(long)]
        #[doc = "Config file(0 or 1)"]
        config: Option<String>,
        #[arg(long)]
        #[arg(num_args = 2)]
        #[doc = "X Y coordinates"]
        coords: Vec<f64>,
        #[arg(short = 'n', long)]
        #[doc = "Item count"]
        count: Option<i32>,
        #[arg(short = 'r', long)]
        #[doc = "Processing rate"]
        rate: Option<f64>,
        #[arg(short = 'm', long)]
        #[doc = "Message text"]
        message: Option<String>,
        #[arg(short = 'p', long)]
        #[doc = "Path argument"]
        path: Option<PathBuf>,
        #[arg(long)]
        #[arg(default_value = "30")]
        #[doc = "Timeout seconds"]
        timeout: i32,
        #[arg(long)]
        #[arg(default_value = "0.95")]
        #[doc = "Threshold"]
        threshold: f64,
        #[arg(long)]
        #[arg(default_value = "unnamed")]
        #[doc = "Project name"]
        name: String,
        #[arg(long)]
        #[doc = "API key(required)"]
        api_key: String,
        #[arg(long)]
        #[doc = "Token(optional)"]
        token: Option<String>,
        #[arg(long = "input-path")]
        #[doc = "Custom dest"]
        custom_input: Option<String>,
        #[arg(long)]
        #[arg(value_name = "FILE")]
        #[doc = "With metavar"]
        file: Option<String>,
        #[arg(long)]
        #[arg(value_parser = ["json", "yaml", "xml"])]
        #[doc = "Output format"]
        format: Option<String>,
        #[arg(long)]
        #[arg(default_value = "manual")]
        #[arg(default_missing_value = "auto", num_args = 0..= 1)]
        #[doc = "Processing mode"]
        mode: Option<String>,
        #[arg(long = "fast")]
        #[arg(default_value_t = false)]
        #[doc = "Fast mode"]
        speed: bool,
    }
    let args = Args::parse();
    let result = serde_json::json!({ "input_file": args.input_file, "debug": args.debug, "output": args.output, "quiet": args.quiet, "color": args.color, "verbosity": args.V, "includes": args.include, "extras": args.extras, "config": args.config, "count": args.count, "rate": args.rate, "timeout": args.timeout, "threshold": args.threshold, "name": args.name, "api_key": args.api_key, "format": args.format, "mode": args.mode, "speed": args.speed });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
