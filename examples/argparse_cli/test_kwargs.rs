use clap::Parser;
#[derive(clap::Parser)]
#[command(about = "Test kwargs preservation")]
struct Args {
    #[doc = "Files to process"]
    files: Vec<String>,
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let args = Args::parse();
    println!("{}", format!("Files: {:?}", args.files));
    ()
}
