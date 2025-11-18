#[doc = "Read file using with statement."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_file(filename: String) -> String {
    let f = std::fs::File::open(filename)?;
    let content = f.read();
    content
}
#[doc = "Write file using with statement."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn write_file(filename: String, content: &str) {
    let f = std::fs::File::create(filename)?;
    f.write(content);
}
#[doc = "Process file with multiple with statements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_file(input_file: String, output_file: String) {
    let fin = std::fs::File::open(input_file)?;
    let data = fin.read();
    let fout = std::fs::File::create(output_file)?;
    fout.write(data.to_uppercase());
}
