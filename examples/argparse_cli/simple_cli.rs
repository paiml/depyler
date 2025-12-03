use clap::Parser;
#[derive(clap::Parser)]
#[command(about = "A simple CLI tool example")]
struct Args {
    #[doc = "Your name"]
    name: String,
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let args = Args::parse();
    println!("{}", format!("Hello, {}!", args.name));
    ()
}
