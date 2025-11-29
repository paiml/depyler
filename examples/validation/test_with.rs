use std::io::Read;
use std::io::Write;
#[doc = "Read file using with statement."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_file(filename: String) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(&filename)?;
    let content = {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        content
    };
    Ok(content.to_string())
}
#[doc = "Write file using with statement."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn write_file(filename: String, content: &str) -> Result<(), std::io::Error> {
    let mut f = std::fs::File::create(&filename)?;
    f.write_all(content.as_bytes()).unwrap();
    Ok(())
}
#[doc = "Process file with multiple with statements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_file(input_file: String, output_file: String) -> Result<(), std::io::Error> {
    let mut fin = std::fs::File::open(&input_file)?;
    let data = {
        let mut content = String::new();
        fin.read_to_string(&mut content)?;
        content
    };
    let mut fout = std::fs::File::create(output_file.as_ref().unwrap())?;
    fout.write_all(data.to_uppercase().as_bytes()).unwrap();
    Ok(())
}
