use clap::Parser;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> i32 {
    #[derive(clap::Parser)]
    struct Args {
        #[doc = "Files to process"]
        files: Vec<String>,
    }
    let args = Args::parse();
    println!("{}", format!("Files: {}", args.files));
    0
}
