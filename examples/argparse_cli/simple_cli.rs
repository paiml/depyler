use clap::Parser;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> i32 {
    #[derive(clap::Parser)]
    struct Args {
        name: String,
    }
    let args = Args::parse();
    println!("{}", format!("Hello, {}!", args.name));
    0
}
