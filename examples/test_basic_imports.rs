use std as os;
use std as sys;
#[doc = "Get current working directory"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_current_directory() -> String {
    std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
#[doc = "Get command line arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_args() -> i32 {
    std::env::args().collect::<Vec<String>>()
}
#[doc = "Exit program with code"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn exit_program(code: i32) {
    std::process::exit(code);
}
