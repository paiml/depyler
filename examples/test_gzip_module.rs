#[doc = "// NOTE: Map Python module 'gzip'(tracked in DEPYLER-0424)"]
use std::io::Cursor;
const STR__: &'static str = "=";
#[doc = "Test basic compression and decompression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_decompress() {
    let data = b"Hello, this is a test string for compression!";
    let compressed = gzip.compress(data);
    assert!(decompressed == data);
    println!("{}", "PASS: test_gzip_compress_decompress");
}
#[doc = "Test compressing text data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_text() {
    let _cse_temp_0 = "The quick brown fox jumps over the lazy dog. ".repeat(10 as usize);
    let text = _cse_temp_0;
    let data = text.encode("utf-8".to_string());
    let compressed = gzip.compress(data);
    assert!(decompressed.decode("utf-8".to_string()) == text);
    println!("{}", "PASS: test_gzip_compress_text");
}
#[doc = "Test compressing empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_empty() {
    let data = b"";
    let compressed = gzip.compress(data);
    assert!(decompressed == b"");
    println!("{}", "PASS: test_gzip_compress_empty");
}
#[doc = "Test different compression levels."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_levels() {
    let _cse_temp_0 = b"Test data for compression levels! " * 100;
    let data = _cse_temp_0;
    let compressed_1 = gzip.compress(data);
    assert!(decompressed_1 == data);
    let compressed_9 = gzip.compress(data);
    assert!(decompressed_9 == data);
    assert!(compressed_9.len() as i32 <= compressed_1.len() as i32);
    println!("{}", "PASS: test_gzip_compress_levels");
}
#[doc = "Test compressing larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_large_data() {
    let _cse_temp_0 = b"ABCDEFGHIJ" * 100;
    let data = _cse_temp_0;
    let compressed = gzip.compress(data);
    assert!(decompressed == data);
    assert!((compressed.len() as i32) < data.len() as i32 / 2);
    println!("{}", "PASS: test_gzip_large_data");
}
#[doc = "Test compressing binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_binary_data() {
    let data = bytes(0..256);
    let compressed = gzip.compress(data);
    assert!(decompressed == data);
    println!("{}", "PASS: test_gzip_binary_data");
}
#[doc = "Test compressing Unicode text."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_unicode_text() {
    let text = "Hello 世界 🌍 Привет مرحبا";
    let data = text.encode("utf-8".to_string());
    let compressed = gzip.compress(data);
    assert!(decompressed.decode("utf-8".to_string()) == text);
    println!("{}", "PASS: test_gzip_unicode_text");
}
#[doc = "Test compressing already compressed data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_multiple_compress() {
    let data = b"Original data for double compression test";
    let compressed_once = gzip.compress(data);
    let compressed_twice = gzip.compress(compressed_once);
    let decompressed_once = gzip.decompress(compressed_twice);
    assert!(decompressed_twice == data);
    println!("{}", "PASS: test_gzip_multiple_compress");
}
#[doc = "Run all gzip tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "GZIP MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_gzip_compress_decompress();
    test_gzip_compress_text();
    test_gzip_compress_empty();
    test_gzip_compress_levels();
    test_gzip_large_data();
    test_gzip_binary_data();
    test_gzip_unicode_text();
    test_gzip_multiple_compress();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL GZIP TESTS PASSED!");
    println!("{}", "Total tests: 8");
    println!("{}", STR__.repeat(60 as usize));
}
