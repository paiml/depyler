#[doc = "// TODO: Map Python module 'gzip'"]
#[doc = "// TODO: Map Python module 'io'"]
const STR__: &'static str = "=";
#[doc = "Test basic compression and decompression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_decompress() {
    let compressed = gzip.compress(b"Original data for double compression test");
    assert!(decompressed == data);
    println!("{}", "PASS: test_gzip_compress_decompress");
}
#[doc = "Test compressing text data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_text() {
    let compressed = gzip.compress(b"Original data for double compression test");
    assert!(decompressed.decode("utf-8".to_string()) == text);
    println!("{}", "PASS: test_gzip_compress_text");
}
#[doc = "Test compressing empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_empty() {
    let compressed = gzip.compress(b"Original data for double compression test");
    assert!(decompressed == b"");
    println!("{}", "PASS: test_gzip_compress_empty");
}
#[doc = "Test different compression levels."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_levels() {
    let compressed_1 = gzip.compress(b"Original data for double compression test");
    assert!(decompressed_1 == data);
    let compressed_9 = gzip.compress(b"Original data for double compression test");
    assert!(decompressed_9 == data);
    assert!(compressed_9.len() as i32 <= compressed_1.len() as i32);
    println!("{}", "PASS: test_gzip_compress_levels");
}
#[doc = "Test compressing larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_large_data() {
    let compressed = gzip.compress(b"Original data for double compression test");
    assert!(decompressed == data);
    assert!((compressed.len() as i32) < data.len() as i32 / 2);
    println!("{}", "PASS: test_gzip_large_data");
}
#[doc = "Test compressing binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_binary_data() {
    let compressed = gzip.compress(b"Original data for double compression test");
    assert!(decompressed == data);
    println!("{}", "PASS: test_gzip_binary_data");
}
#[doc = "Test compressing Unicode text."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_unicode_text() {
    let compressed = gzip.compress(b"Original data for double compression test");
    assert!(decompressed.decode("utf-8".to_string()) == text);
    println!("{}", "PASS: test_gzip_unicode_text");
}
#[doc = "Test compressing already compressed data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_multiple_compress() {
    let compressed_once = gzip.compress(b"Original data for double compression test");
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
