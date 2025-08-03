# Test base64 module mapping  
import base64

def encode_data(data: str) -> str:
    """Encode string to base64"""
    encoded_bytes = base64.b64encode(data.encode('utf-8'))
    return encoded_bytes.decode('utf-8')

def decode_data(encoded: str) -> str:
    """Decode base64 string"""
    decoded_bytes = base64.b64decode(encoded)
    return decoded_bytes.decode('utf-8')

def encode_url_safe(data: str) -> str:
    """Encode string to URL-safe base64"""
    encoded_bytes = base64.urlsafe_b64encode(data.encode('utf-8'))
    return encoded_bytes.decode('utf-8')