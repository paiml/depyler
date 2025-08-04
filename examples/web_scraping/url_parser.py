# @depyler: string_strategy = "zero_copy"
# @depyler: bounds_checking = "explicit"
from typing import Dict, Optional, Tuple

class URL:
    """Simple URL parser without external dependencies"""
    
    def __init__(self, url: str) -> None:
        self.original = url
        self.scheme = ""
        self.host = ""
        self.port: Optional[int] = None
        self.path = ""
        self.query_params: Dict[str, str] = {}
        self.fragment = ""
        
        self._parse(url)
    
    def _parse(self, url: str) -> None:
        """Parse URL into components"""
        remaining = url
        
        # Extract scheme (http://, https://, etc.)
        if "://" in remaining:
            scheme_end = remaining.find("://")
            self.scheme = remaining[:scheme_end]
            remaining = remaining[scheme_end + 3:]
        
        # Extract fragment (#section)
        if "#" in remaining:
            fragment_start = remaining.find("#")
            self.fragment = remaining[fragment_start + 1:]
            remaining = remaining[:fragment_start]
        
        # Extract query parameters (?key=value&key2=value2)
        if "?" in remaining:
            query_start = remaining.find("?")
            query_string = remaining[query_start + 1:]
            remaining = remaining[:query_start]
            self._parse_query(query_string)
        
        # Extract path
        if "/" in remaining:
            path_start = remaining.find("/")
            self.path = remaining[path_start:]
            remaining = remaining[:path_start]
        else:
            self.path = "/"
        
        # Remaining should be host[:port]
        if ":" in remaining:
            colon_pos = remaining.find(":")
            self.host = remaining[:colon_pos]
            port_str = remaining[colon_pos + 1:]
            try:
                self.port = int(port_str)
            except:
                self.port = None
        else:
            self.host = remaining
    
    def _parse_query(self, query_string: str) -> None:
        """Parse query parameters"""
        if not query_string:
            return
        
        pairs = query_string.split("&")
        for pair in pairs:
            if "=" in pair:
                eq_pos = pair.find("=")
                key = pair[:eq_pos]
                value = pair[eq_pos + 1:]
                self.query_params[key] = value
            else:
                self.query_params[pair] = ""
    
    def get_query_param(self, key: str) -> Optional[str]:
        """Get query parameter value"""
        if key in self.query_params:
            return self.query_params[key]
        return None
    
    def is_secure(self) -> bool:
        """Check if URL uses HTTPS"""
        return self.scheme.lower() == "https"
    
    def get_base_url(self) -> str:
        """Get base URL without path, query, or fragment"""
        result = ""
        if self.scheme:
            result += self.scheme + "://"
        
        result += self.host
        
        if self.port is not None:
            result += ":" + str(self.port)
        
        return result

def extract_domain(url: str) -> str:
    """Extract domain from URL string"""
    parsed = URL(url)
    return parsed.host

def is_valid_email(email: str) -> bool:
    """Basic email validation"""
    if "@" not in email or email.count("@") != 1:
        return False
    
    at_pos = email.find("@")
    local_part = email[:at_pos]
    domain_part = email[at_pos + 1:]
    
    # Basic checks
    if len(local_part) == 0 or len(domain_part) == 0:
        return False
    
    if "." not in domain_part:
        return False
    
    # Check for consecutive dots
    if ".." in email:
        return False
    
    # Check start and end
    if email.startswith(".") or email.endswith("."):
        return False
    
    return True

def normalize_url(url: str) -> str:
    """Normalize URL by removing trailing slash, converting to lowercase"""
    normalized = url.strip().lower()
    
    # Remove trailing slash unless it's the root path
    if normalized.endswith("/") and len(normalized) > 1:
        # Don't remove if it's just the scheme + domain
        if normalized.count("/") > 2:
            normalized = normalized[:-1]
    
    return normalized