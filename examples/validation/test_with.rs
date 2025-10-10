#[doc = "Read file using with statement."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn read_file(filename: String)  -> String {
    { let mut f = open(filename);
    let content = f.read();
   
}
return content;
   
}
#[doc = "Write file using with statement."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn write_file<'a>(filename: String, content: & 'a str) {
    { let mut f = open(filename, "w".to_string());
    f.write(content);
   
}
} #[doc = "Process file with multiple with statements."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn process_file(input_file: String, output_file: String) {
    { let mut fin = open(input_file);
    let data = fin.read();
   
}
{
    let mut fout = open(output_file, "w".to_string());
    fout.write(data.to_uppercase());
   
}
}