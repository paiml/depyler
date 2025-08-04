# @depyler: thread_safety = "required"
# @depyler: optimization_level = "size"
from typing import Dict, Optional, Tuple

class HTTPRequest:
    """Simple HTTP request representation"""
    
    def __init__(self, method: str, url: str, headers: Optional[Dict[str, str]] = None) -> None:
        self.method = method.upper()
        self.url = url
        self.headers = headers if headers else {}
        self.body = ""
    
    def add_header(self, name: str, value: str) -> None:
        """Add HTTP header"""
        self.headers[name] = value
    
    def set_body(self, body: str) -> None:
        """Set request body"""
        self.body = body
    
    def to_string(self) -> str:
        """Convert request to HTTP string format"""
        lines: List[str] = []
        
        # Request line
        lines.append(f"{self.method} {self.url} HTTP/1.1")
        
        # Headers
        for name, value in self.headers.items():
            lines.append(f"{name}: {value}")
        
        # Empty line
        lines.append("")
        
        # Body
        if self.body:
            lines.append(self.body)
        
        return "\n".join(lines)

class HTTPResponse:
    """Simple HTTP response representation"""
    
    def __init__(self, status_code: int, reason: str) -> None:
        self.status_code = status_code
        self.reason = reason
        self.headers: Dict[str, str] = {}
        self.body = ""
    
    def add_header(self, name: str, value: str) -> None:
        """Add HTTP header"""
        self.headers[name] = value
    
    def set_body(self, body: str) -> None:
        """Set response body"""
        self.body = body
    
    def is_success(self) -> bool:
        """Check if response indicates success (2xx status)"""
        return 200 <= self.status_code < 300
    
    def is_client_error(self) -> bool:
        """Check if response indicates client error (4xx status)"""
        return 400 <= self.status_code < 500
    
    def is_server_error(self) -> bool:
        """Check if response indicates server error (5xx status)"""
        return 500 <= self.status_code < 600

class HTTPClient:
    """Simple HTTP client simulation (without actual networking)"""
    
    def __init__(self) -> None:
        self.default_headers: Dict[str, str] = {
            "User-Agent": "SimpleHTTPClient/1.0",
            "Accept": "*/*"
        }
    
    def create_request(self, method: str, url: str) -> HTTPRequest:
        """Create HTTP request with default headers"""
        request = HTTPRequest(method, url, self.default_headers.copy())
        return request
    
    def parse_response(self, response_text: str) -> HTTPResponse:
        """Parse HTTP response from text"""
        lines = response_text.split('\n')
        if not lines:
            return HTTPResponse(500, "Invalid Response")
        
        # Parse status line
        status_line = lines[0]
        status_parts = status_line.split(' ', 2)
        if len(status_parts) < 3:
            return HTTPResponse(500, "Invalid Status Line")
        
        try:
            status_code = int(status_parts[1])
        except:
            status_code = 500
        
        reason = status_parts[2] if len(status_parts) > 2 else "Unknown"
        response = HTTPResponse(status_code, reason)
        
        # Parse headers
        i = 1
        while i < len(lines) and lines[i].strip():
            header_line = lines[i].strip()
            if ':' in header_line:
                colon_pos = header_line.find(':')
                name = header_line[:colon_pos].strip()
                value = header_line[colon_pos + 1:].strip()
                response.add_header(name, value)
            i += 1
        
        # Parse body (everything after empty line)
        body_start = i + 1
        if body_start < len(lines):
            body_lines = lines[body_start:]
            response.set_body('\n'.join(body_lines))
        
        return response

def build_query_string(params: Dict[str, str]) -> str:
    """Build URL query string from parameters"""
    if not params:
        return ""
    
    pairs: List[str] = []
    for key, value in params.items():
        # Simple URL encoding (just replace spaces)
        encoded_key = key.replace(' ', '%20')
        encoded_value = value.replace(' ', '%20')
        pairs.append(f"{encoded_key}={encoded_value}")
    
    return "?" + "&".join(pairs)

def parse_url_components(url: str) -> Tuple[str, str, str, str]:
    """Parse URL into scheme, host, path, and query"""
    scheme = ""
    host = ""
    path = "/"
    query = ""
    
    remaining = url
    
    # Extract scheme
    if "://" in remaining:
        scheme_end = remaining.find("://")
        scheme = remaining[:scheme_end]
        remaining = remaining[scheme_end + 3:]
    
    # Extract query
    if "?" in remaining:
        query_start = remaining.find("?")
        query = remaining[query_start + 1:]
        remaining = remaining[:query_start]
    
    # Extract path
    if "/" in remaining:
        path_start = remaining.find("/")
        path = remaining[path_start:]
        host = remaining[:path_start]
    else:
        host = remaining
        path = "/"
    
    return scheme, host, path, query

def format_json_response(response: HTTPResponse) -> str:
    """Format response as JSON-like string (simplified)"""
    # Simple JSON formatting without actual JSON parsing
    result = "{\n"
    result += f'  "status": {response.status_code},\n'
    result += f'  "reason": "{response.reason}",\n'
    result += '  "headers": {\n'
    
    header_items: List[str] = []
    for name, value in response.headers.items():
        header_items.append(f'    "{name}": "{value}"')
    
    result += ",\n".join(header_items)
    result += '\n  },\n'
    result += f'  "body": "{response.body}"\n'
    result += "}"
    
    return result