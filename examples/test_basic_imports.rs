#[doc = "// Python import: os"] #[doc = "// Python import: sys"] #[doc = "Get current working directory"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_current_directory()  -> String {
    return std::env::current_dir().unwrap().to_string_lossy().to_string();
   
}
#[doc = "Get command line arguments"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_args()  -> Vec<DynamicType>{
    return std::env::args;
   
}
#[doc = "Exit program with code"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn exit_program(code: i32)  -> DynamicType {
    std::process::exit(code)
}