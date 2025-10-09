#[doc = "// Python import: sys"] #[doc = "Get command line arguments"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_command_args()  -> Vec<String>{
    return {
    let start  = (1).max(0) as usize;
    if start<std::env::args.len() {
    std::env::args [start..].to_vec()
}
else {
    Vec::new()
}
};
   
}
#[doc = "Print error to stderr and exit"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn exit_with_error(message: String, code: i32) {
    std::io::stderr.write(format !("Error: {}\n", message));
    std::process::exit(code);
   
}
#[doc = "Print message to stdout"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn print_to_stdout<'a>(message: & 'a str) {
    std::io::stdout.write(format !("{}{}", message, "\n".to_string()));
    std::io::stdout.flush();
   
}
#[doc = "Read all input from stdin"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn read_from_stdin ()  -> String {
    return std::io::stdin.read();
   
}
#[doc = "Check if Python version is at least 3.6"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn check_python_version()  -> bool {
    let _cse_temp_0 = sys.version_info>= (3, 6);
    return _cse_temp_0
}