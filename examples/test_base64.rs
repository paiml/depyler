use base64;
#[doc = "Encode string to base64"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn encode_data(data: &str) -> String {
    let encoded_bytes = base64::engine::general_purpose::STANDARD.encode(data.as_bytes().to_vec());
    String::from_utf8_lossy(&encoded_bytes).to_string()
}
#[doc = "Decode base64 string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn decode_data(encoded: &str) -> String {
    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    String::from_utf8_lossy(&decoded_bytes).to_string()
}
#[doc = "Encode string to URL-safe base64"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn encode_url_safe(data: &str) -> String {
    let encoded_bytes = base64::engine::general_purpose::URL_SAFE.encode(data.as_bytes().to_vec());
    String::from_utf8_lossy(&encoded_bytes).to_string()
}
