# @depyler: optimization_level = "aggressive"
# @depyler: string_strategy = "zero_copy"
from typing import Dict, List, Optional, Tuple

class LogEntry:
    """Represents a single log entry"""
    
    def __init__(self, timestamp: str, level: str, message: str) -> None:
        self.timestamp = timestamp
        self.level = level
        self.message = message
    
    def is_error(self) -> bool:
        """Check if this is an error-level log entry"""
        return self.level.upper() in ["ERROR", "ERR", "FATAL"]
    
    def is_warning(self) -> bool:
        """Check if this is a warning-level log entry"""
        return self.level.upper() in ["WARNING", "WARN"]

class LogAnalyzer:
    """Analyzes log files and extracts insights"""
    
    def __init__(self) -> None:
        self.entries: List[LogEntry] = []
    
    def parse_log_line(self, line: str) -> Optional[LogEntry]:
        """Parse a single log line into a LogEntry"""
        # Simple format: "TIMESTAMP LEVEL MESSAGE"
        # Example: "2023-01-01 10:30:45 ERROR Database connection failed"
        
        parts = line.strip().split()
        if len(parts) < 3:
            return None
        
        # Extract timestamp (assume first two parts: date and time)
        if len(parts) >= 2:
            timestamp = parts[0] + " " + parts[1]
            level = parts[2]
            message = " ".join(parts[3:])
        else:
            return None
        
        return LogEntry(timestamp, level, message)
    
    def load_from_string(self, log_content: str) -> None:
        """Load log entries from string content"""
        lines = log_content.split('\n')
        for line in lines:
            if line.strip():
                entry = self.parse_log_line(line)
                if entry:
                    self.entries.append(entry)
    
    def count_by_level(self) -> Dict[str, int]:
        """Count log entries by level"""
        counts: Dict[str, int] = {}
        
        for entry in self.entries:
            level = entry.level.upper()
            if level in counts:
                counts[level] += 1
            else:
                counts[level] = 1
        
        return counts
    
    def get_error_messages(self) -> List[str]:
        """Get all error messages"""
        errors: List[str] = []
        for entry in self.entries:
            if entry.is_error():
                errors.append(entry.message)
        return errors
    
    def find_patterns(self, pattern: str) -> List[LogEntry]:
        """Find log entries containing specific pattern"""
        matches: List[LogEntry] = []
        pattern_lower = pattern.lower()
        
        for entry in self.entries:
            if pattern_lower in entry.message.lower():
                matches.append(entry)
        
        return matches
    
    def get_hourly_stats(self) -> Dict[str, int]:
        """Get log entry counts by hour"""
        hourly_counts: Dict[str, int] = {}
        
        for entry in self.entries:
            # Extract hour from timestamp (assume format "YYYY-MM-DD HH:MM:SS")
            timestamp_parts = entry.timestamp.split()
            if len(timestamp_parts) >= 2:
                time_part = timestamp_parts[1]
                hour_parts = time_part.split(":")
                if len(hour_parts) >= 1:
                    hour = hour_parts[0]
                    if hour in hourly_counts:
                        hourly_counts[hour] += 1
                    else:
                        hourly_counts[hour] = 1
        
        return hourly_counts
    
    def get_top_error_patterns(self, limit: int = 5) -> List[Tuple[str, int]]:
        """Get most common error patterns"""
        error_messages = self.get_error_messages()
        
        # Simple pattern extraction - look for common words in error messages
        word_counts: Dict[str, int] = {}
        
        for message in error_messages:
            words = message.lower().split()
            for word in words:
                # Skip common words
                if len(word) > 3 and word not in ["the", "and", "for", "with", "from"]:
                    if word in word_counts:
                        word_counts[word] += 1
                    else:
                        word_counts[word] = 1
        
        # Sort by count and return top patterns
        sorted_patterns: List[Tuple[str, int]] = []
        for word, count in word_counts.items():
            sorted_patterns.append((word, count))
        
        # Simple sorting by count (bubble sort)
        n = len(sorted_patterns)
        for i in range(n):
            for j in range(0, n - i - 1):
                if sorted_patterns[j][1] < sorted_patterns[j + 1][1]:
                    sorted_patterns[j], sorted_patterns[j + 1] = sorted_patterns[j + 1], sorted_patterns[j]
        
        return sorted_patterns[:limit]

def analyze_log_health(log_content: str) -> Dict[str, float]:
    """Analyze overall log health metrics"""
    analyzer = LogAnalyzer()
    analyzer.load_from_string(log_content)
    
    if not analyzer.entries:
        return {"health_score": 0.0, "error_rate": 0.0, "warning_rate": 0.0}
    
    total_entries = len(analyzer.entries)
    error_count = len([e for e in analyzer.entries if e.is_error()])
    warning_count = len([e for e in analyzer.entries if e.is_warning()])
    
    error_rate = error_count / total_entries
    warning_rate = warning_count / total_entries
    
    # Simple health score: 1.0 is perfect (no errors/warnings)
    health_score = 1.0 - (error_rate * 0.8 + warning_rate * 0.3)
    if health_score < 0.0:
        health_score = 0.0
    
    return {
        "health_score": health_score,
        "error_rate": error_rate,
        "warning_rate": warning_rate
    }