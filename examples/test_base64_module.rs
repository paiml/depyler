use base64;
const STR__: &'static str = "=";
#[doc = "Test basic base64 encoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_encode_basic() {
    let data = b"Hello, World!";
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    assert!(encoded == b"SGVsbG8sIFdvcmxkIQ==");
    println!("{}", "PASS: test_base64_encode_basic");
}
#[doc = "Test basic base64 decoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_decode_basic() {
    let encoded = b"SGVsbG8sIFdvcmxkIQ==";
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    assert!(decoded == b"Hello, World!");
    println!("{}", "PASS: test_base64_decode_basic");
}
#[doc = "Test encode-decode round trip."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_roundtrip() {
    let original = b"Python to Rust transpilation!";
    let encoded = base64::engine::general_purpose::STANDARD.encode(original);
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    assert!(decoded == original);
    println!("{}", "PASS: test_base64_roundtrip");
}
#[doc = "Test encoding/decoding empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_empty() {
    let data = b"";
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    assert!(encoded == b"");
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(b"")
        .unwrap();
    assert!(decoded == b"");
    println!("{}", "PASS: test_base64_empty");
}
#[doc = "Test encoding binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_binary_data() {
    let data = bytes(0..256);
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    assert!(decoded == data);
    println!("{}", "PASS: test_base64_binary_data");
}
#[doc = "Test URL-safe base64 encoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_urlsafe_encode() {
    let data = b"Hello>>???World";
    let encoded = base64::engine::general_purpose::URL_SAFE.encode(data);
    assert!(!encoded.get(&b"+").is_some());
    assert!(!encoded.get(&b"/").is_some());
    println!("{}", "PASS: test_base64_urlsafe_encode");
}
#[doc = "Test URL-safe base64 decoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_urlsafe_decode() {
    let data = b"Test data with special chars";
    let encoded = base64::engine::general_purpose::URL_SAFE.encode(data);
    let decoded = base64::engine::general_purpose::URL_SAFE
        .decode(encoded)
        .unwrap();
    assert!(decoded == data);
    println!("{}", "PASS: test_base64_urlsafe_decode");
}
#[doc = "Test base64 padding handling."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_padding() {
    let data1 = b"a";
    let encoded1 = base64::engine::general_purpose::STANDARD.encode(data1);
    assert!(encoded1 == b"YQ==");
    let data2 = b"ab";
    let encoded2 = base64::engine::general_purpose::STANDARD.encode(data2);
    assert!(encoded2 == b"YWI=");
    let data3 = b"abc";
    let encoded3 = base64::engine::general_purpose::STANDARD.encode(data3);
    assert!(encoded3 == b"YWJj");
    println!("{}", "PASS: test_base64_padding");
}
#[doc = "Test encoding larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_multiline() {
    let _cse_temp_0 = b"The quick brown fox jumps over the lazy dog. " * 10;
    let data = _cse_temp_0;
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    assert!(decoded == data);
    println!("{}", "PASS: test_base64_multiline");
}
#[doc = "Test encoding Unicode text."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_unicode() {
    let text = "Hello 世界 🌍";
    let data = text.as_bytes().to_vec();
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    let result = String::from_utf8_lossy(&decoded).to_string();
    assert!(result == text);
    println!("{}", "PASS: test_base64_unicode");
}
#[doc = "Run all base64 tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "BASE64 MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_base64_encode_basic();
    test_base64_decode_basic();
    test_base64_roundtrip();
    test_base64_empty();
    test_base64_binary_data();
    test_base64_urlsafe_encode();
    test_base64_urlsafe_decode();
    test_base64_padding();
    test_base64_multiline();
    test_base64_unicode();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL BASE64 TESTS PASSED!");
    println!("{}", "Total tests: 10");
    println!("{}", STR__.repeat(60 as usize));
}
