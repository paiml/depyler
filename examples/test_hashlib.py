# Test hashlib module mapping
import hashlib

def hash_password(password: str) -> str:
    """Hash a password using SHA256"""
    hasher = hashlib.sha256()
    hasher.update(password.encode('utf-8'))
    return hasher.hexdigest()

def compute_file_checksum(data: bytes) -> str:
    """Compute MD5 checksum of file data"""
    hasher = hashlib.md5()
    hasher.update(data)
    return hasher.hexdigest()

def verify_integrity(data: str, expected_hash: str) -> bool:
    """Verify data integrity using SHA512"""
    hasher = hashlib.sha512()
    hasher.update(data.encode('utf-8'))
    actual_hash = hasher.hexdigest()
    return actual_hash == expected_hash