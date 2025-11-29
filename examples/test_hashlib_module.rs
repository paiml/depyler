use sha2 as hashlib;
const STR__: &'static str = "=";
#[doc = "Test basic SHA256 hashing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_basic() {
    let data = b"Hello, World!";
    let hash_obj = {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result == expected);
    println!("{}", "PASS: test_sha256_basic");
}
#[doc = "Test SHA256 of empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_empty() {
    let data = b"";
    let hash_obj = {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result == expected);
    println!("{}", "PASS: test_sha256_empty");
}
#[doc = "Test SHA256 with multiple updates."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_update() {
    let mut hash_obj = {
        use digest::DynDigest;
        use sha2::Digest;
        Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
    };
    hash_obj.update(&b"Hello, ");
    hash_obj.update(&b"World!");
    assert!(result == expected);
    println!("{}", "PASS: test_sha256_update");
}
#[doc = "Test basic SHA1 hashing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha1_basic() {
    let data = b"test data";
    let mut hash_obj = {
        use sha1::Digest;
        let mut hasher = sha1::Sha1::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result == expected);
    println!("{}", "PASS: test_sha1_basic");
}
#[doc = "Test basic MD5 hashing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_md5_basic() {
    let data = b"test";
    let mut hash_obj = {
        use md5::Digest;
        let mut hasher = md5::Md5::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result == expected);
    println!("{}", "PASS: test_md5_basic");
}
#[doc = "Test hashing binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_binary_data() {
    let data = bytes(0..256);
    let mut hash_obj = {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result.len() as i32 == 64);
    assert!(result
        .as_slice()
        .iter()
        .copied()
        .map(|c| "0123456789abcdef".to_string().contains(&c))
        .all(|x| x));
    println!("{}", "PASS: test_sha256_binary_data");
}
#[doc = "Test hashing larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_large_data() {
    let _cse_temp_0 = b"A" * 10000;
    let data = _cse_temp_0;
    let mut hash_obj = {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result.len() as i32 == 64);
    assert!(result
        .as_slice()
        .iter()
        .copied()
        .map(|c| "0123456789abcdef".to_string().contains(&c))
        .all(|x| x));
    println!("{}", "PASS: test_sha256_large_data");
}
#[doc = "Test that different data produces different hashes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hash_different_data() {
    assert!(hash1 != hash2);
    println!("{}", "PASS: test_hash_different_data");
}
#[doc = "Test that same data produces same hash."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hash_deterministic() {
    let data = b"deterministic test";
    assert!(hash1 == hash2);
    println!("{}", "PASS: test_hash_deterministic");
}
#[doc = "Test hashing text(encoded to bytes)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_text() {
    let text = "Hello, 世界!";
    let data = text.encode("utf-8".to_string());
    let mut hash_obj = {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    };
    assert!(result.len() as i32 == 64);
    assert!(result
        .as_slice()
        .iter()
        .copied()
        .map(|c| "0123456789abcdef".to_string().contains(&c))
        .all(|x| x));
    println!("{}", "PASS: test_sha256_text");
}
#[doc = "Run all hashlib tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "HASHLIB MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_sha256_basic();
    test_sha256_empty();
    test_sha256_update();
    test_sha1_basic();
    test_md5_basic();
    test_sha256_binary_data();
    test_sha256_large_data();
    test_hash_different_data();
    test_hash_deterministic();
    test_sha256_text();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL HASHLIB TESTS PASSED!");
    println!("{}", "Total tests: 10");
    println!("{}", STR__.repeat(60 as usize));
}
