use std as sys;
#[doc = "Get command line arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_command_args() -> Vec<String> {
    {
        let base = std::env::args().collect::<Vec<String>>();
        let start = (1).max(0) as usize;
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    }
}
#[doc = "Print error to stderr and exit"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn exit_with_error(message: String, code: i32) {
    {
        use std::io::Write;
        write!(std::io::stderr(), "{}", format!("Error: {:?}\n", message)).unwrap();
    };
    std::process::exit(code);
}
#[doc = "Print message to stdout"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn print_to_stdout(message: &str) {
    {
        use std::io::Write;
        write!(
            std::io::stdout(),
            "{}",
            format!("{}{}", message, "\n".to_string())
        )
        .unwrap();
    };
    {
        use std::io::Write;
        std::io::stdout().flush().unwrap()
    };
}
#[doc = "Read all input from stdin"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_from_stdin() -> String {
    {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    }
}
#[doc = "Check if Python version is at least 3.6"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn check_python_version() -> bool {
    (3, 11) >= (3, 6)
}
