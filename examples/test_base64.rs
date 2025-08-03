#[doc = "// Python import: base64"] #[doc = "Encode string to base64"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn encode_data<'a>(data: & 'a str)  -> String {
    let mut encoded_bytes = std::encode(data.encode("utf-8".to_string()));
    return encoded_bytes.decode("utf-8".to_string());
   
}
#[doc = "Decode base64 string"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn decode_data<'a>(encoded: & 'a str)  -> String {
    let mut decoded_bytes = std::decode(encoded);
    return decoded_bytes.decode("utf-8".to_string());
   
}
#[doc = "Encode string to URL-safe base64"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn encode_url_safe<'a>(data: & 'a str)  -> String {
    let mut encoded_bytes = std::encode_config(data.encode("utf-8".to_string()));
    return encoded_bytes.decode("utf-8".to_string())
}