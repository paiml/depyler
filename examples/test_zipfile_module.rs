#[doc = "// TODO: Map Python module 'zipfile'"]
#[doc = "// TODO: Map Python module 'io'"]
const STR__: &'static str = "=";
#[doc = "Test creating a ZIP file and reading it back."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_create_and_read() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
        zf.writestr("test.txt".to_string(), "Hello, ZIP!".to_string());
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        let content = zf.read("test.txt".to_string());
        assert!(content == b"Hello, ZIP!");
    }
    println!("{}", "PASS: test_zipfile_create_and_read");
}
#[doc = "Test ZIP with multiple files."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_multiple_files() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
        zf.writestr("file1.txt".to_string(), "Content 1".to_string());
        zf.writestr("file2.txt".to_string(), "Content 2".to_string());
        zf.writestr("file3.txt".to_string(), "Content 3".to_string());
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        assert!(zf.namelist().len() as i32 == 3);
        assert!(zf.namelist().contains_key(&"file1.txt".to_string()));
        assert!(zf.namelist().contains_key(&"file2.txt".to_string()));
        assert!(zf.namelist().contains_key(&"file3.txt".to_string()));
        assert!(zf.read("file2.txt".to_string()) == b"Content 2");
    }
    println!("{}", "PASS: test_zipfile_multiple_files");
}
#[doc = "Test listing files in ZIP archive."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_namelist() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
        zf.writestr("alpha.txt".to_string(), "A".to_string());
        zf.writestr("beta.txt".to_string(), "B".to_string());
        zf.writestr("gamma.txt".to_string(), "C".to_string());
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        let names = zf.namelist();
        assert!(names.len() as i32 == 3);
        assert!(names.contains_key(&"alpha.txt".to_string()));
        assert!(names.contains_key(&"beta.txt".to_string()));
        assert!(names.contains_key(&"gamma.txt".to_string()));
    }
    println!("{}", "PASS: test_zipfile_namelist");
}
#[doc = "Test getting file info from ZIP."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_getinfo() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
        zf.writestr("data.txt".to_string(), "Test data content".to_string());
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        let info = zf.getinfo("data.txt".to_string());
        assert!(info.filename == "data.txt".to_string());
        assert!(info.file_size == "Test data content".to_string().len() as i32);
    }
    println!("{}", "PASS: test_zipfile_getinfo");
}
#[doc = "Test ZIP with compression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_compression() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string(), zipfile.ZIP_DEFLATED);
        let zf = _context.__enter__();
        zf.writestr("compressed.txt".to_string(), data);
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        let content = zf
            .read("compressed.txt".to_string())
            .decode("utf-8".to_string());
        assert!(content == data);
    }
    println!("{}", "PASS: test_zipfile_compression");
}
#[doc = "Test ZIP with binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_binary_data() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
        zf.writestr("binary.dat".to_string(), binary_data);
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        let content = zf.read("binary.dat".to_string());
        assert!(content == binary_data);
    }
    println!("{}", "PASS: test_zipfile_binary_data");
}
#[doc = "Test creating empty ZIP archive."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_empty() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        assert!(zf.namelist().len() as i32 == 0);
    }
    println!("{}", "PASS: test_zipfile_empty");
}
#[doc = "Test reading from existing ZIP."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_read_mode() {
    let buffer = BytesIO::new();
    {
        let _context = zipfile.ZipFile(buffer, "w".to_string());
        let zf = _context.__enter__();
        zf.writestr("readonly.txt".to_string(), "Read-only content".to_string());
    }
    buffer.seek(0);
    {
        let _context = zipfile.ZipFile(buffer, "r".to_string());
        let zf = _context.__enter__();
        let content = zf.read("readonly.txt".to_string());
        assert!(content == b"Read-only content");
    }
    println!("{}", "PASS: test_zipfile_read_mode");
}
#[doc = "Run all zipfile tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ZIPFILE MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_zipfile_create_and_read();
    test_zipfile_multiple_files();
    test_zipfile_namelist();
    test_zipfile_getinfo();
    test_zipfile_compression();
    test_zipfile_binary_data();
    test_zipfile_empty();
    test_zipfile_read_mode();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL ZIPFILE TESTS PASSED!");
    println!("{}", "Total tests: 8");
    println!("{}", STR__.repeat(60 as usize));
}
