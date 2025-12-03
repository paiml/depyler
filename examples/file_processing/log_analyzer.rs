use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}
impl LogEntry {
    pub fn new(timestamp: String, level: String, message: String) -> Self {
        Self {
            timestamp,
            level,
            message,
        }
    }
    pub fn is_error(&self) -> bool {
        return ["ERROR".to_string(), "ERR".to_string(), "FATAL".to_string()]
            .contains_key(&self.level.to_uppercase());
    }
    pub fn is_warning(&self) -> bool {
        return ["WARNING".to_string(), "WARN".to_string()]
            .contains_key(&self.level.to_uppercase());
    }
}
#[derive(Debug, Clone)]
pub struct LogAnalyzer {
    pub entries: Vec<LogEntry>,
}
impl LogAnalyzer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn parse_log_line(&self, line: String) -> Option<LogEntry> {
        let mut parts = line
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if parts.len() < 3 {
            return ();
        };
        if parts.len() >= 2 {
            let mut timestamp = parts[0 as usize] + " ".to_string() + parts[1 as usize];
            let mut level = parts[2 as usize];
            let mut message = {
                let s = &parts;
                let len = s.chars().count() as isize;
                let start_idx = (3) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            }
            .join(" ".to_string());
        } else {
            return ();
        };
        return LogEntry::new(timestamp, level, message);
    }
    pub fn load_from_string(&self, log_content: String) {
        let mut lines = log_content
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for line in lines {
            if line.trim().to_string() {
                let mut entry = self.parse_log_line(line);
                if entry {
                    self.entries.push(entry);
                };
            };
        }
    }
    pub fn count_by_level(&self) -> HashMap<String, i32> {
        let mut counts = {
            let mut map = std::collections::HashMap::new();
            map
        };
        for entry in self.entries {
            let mut level = entry.level.to_uppercase();
            if counts.contains_key(&level) {
                counts.insert(level, counts[level as usize] + 1);
            } else {
                counts.insert(level, 1);
            };
        }
        return counts;
    }
    pub fn get_error_messages(&self) -> Vec<String> {
        let mut errors = vec![];
        for entry in self.entries {
            if entry.is_error() {
                errors.push(entry.message);
            };
        }
        return errors;
    }
    pub fn find_patterns(&self, pattern: String) -> Vec<LogEntry> {
        let mut matches = vec![];
        let mut pattern_lower = pattern.to_lowercase();
        for entry in self.entries {
            if entry.message.to_lowercase().contains(&pattern_lower) {
                matches.push(entry);
            };
        }
        return matches;
    }
    pub fn get_hourly_stats(&self) -> HashMap<String, i32> {
        let mut hourly_counts = {
            let mut map = std::collections::HashMap::new();
            map
        };
        for entry in self.entries {
            let mut timestamp_parts = entry
                .timestamp
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            if timestamp_parts.len() >= 2 {
                let mut time_part = timestamp_parts[1 as usize];
                let mut hour_parts = time_part
                    .split(":")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                if hour_parts.len() >= 1 {
                    let mut hour = hour_parts[0 as usize];
                    if hourly_counts.contains_key(&hour) {
                        hourly_counts.insert(hour, hourly_counts[hour as usize] + 1);
                    } else {
                        hourly_counts.insert(hour, 1);
                    };
                };
            };
        }
        return hourly_counts;
    }
    pub fn get_top_error_patterns(&self, limit: i32) -> Vec<(String, i32)> {
        let mut error_messages = self.get_error_messages();
        let mut word_counts = {
            let mut map = std::collections::HashMap::new();
            map
        };
        for message in error_messages {
            let mut words = message
                .to_lowercase()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for word in words {
                if word.len() > 3
                    && ![
                        "the".to_string(),
                        "and".to_string(),
                        "for".to_string(),
                        "with".to_string(),
                        "from".to_string(),
                    ]
                    .contains_key(&word)
                {
                    if word_counts.contains_key(&word) {
                        word_counts.insert(word, word_counts[word as usize] + 1);
                    } else {
                        word_counts.insert(word, 1);
                    };
                };
            }
        }
        let mut sorted_patterns = vec![];
        for (word, count) in word_counts.items() {
            sorted_patterns.push((word, count));
        }
        let mut n = sorted_patterns.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if sorted_patterns[j as usize][1 as usize]
                    < sorted_patterns[j + 1 as usize][1 as usize]
                {
                    {
                        let _swap_temp =
                            (sorted_patterns[j + 1 as usize], sorted_patterns[j as usize]);
                        sorted_patterns[(j) as usize] = _swap_temp.0;
                        sorted_patterns[(j + 1) as usize] = _swap_temp.1;
                    }
                };
            }
        }
        return {
            let s = &sorted_patterns;
            let len = s.chars().count() as isize;
            let stop_idx = (limit) as isize;
            let stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            s.chars().take(stop).collect::<String>()
        };
    }
}
#[doc = "Analyze overall log health metrics"]
#[doc = " Depyler: proven to terminate"]
pub fn analyze_log_health(
    log_content: &str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let analyzer = LogAnalyzer::new();
    analyzer.load_from_string(log_content);
    if !analyzer.entries {
        return Ok({
            let mut map = HashMap::new();
            map.insert("health_score".to_string(), 0.0);
            map.insert("error_rate".to_string(), 0.0);
            map.insert("warning_rate".to_string(), 0.0);
            map
        });
    }
    let _cse_temp_0 = analyzer.entries.len() as i32;
    let total_entries = _cse_temp_0;
    let _cse_temp_1 = analyzer
        .entries
        .into_iter()
        .filter(|e| e.is_error())
        .map(|e| e)
        .collect::<Vec<_>>()
        .len() as i32;
    let error_count = _cse_temp_1;
    let warning_count = _cse_temp_1;
    let _cse_temp_2 = error_count / total_entries;
    let error_rate = _cse_temp_2;
    let _cse_temp_3 = warning_count / total_entries;
    let warning_rate = _cse_temp_3;
    let _cse_temp_4 = error_rate * 0.8;
    let _cse_temp_5 = warning_rate * 0.3;
    let _cse_temp_6 = 1.0 - _cse_temp_4 + _cse_temp_5;
    let mut health_score = _cse_temp_6.clone();
    let _cse_temp_7 = health_score < 0.0;
    if _cse_temp_7 {
        health_score = 0.0;
    }
    Ok({
        let mut map = HashMap::new();
        map.insert("health_score".to_string(), health_score);
        map.insert("error_rate".to_string(), error_rate);
        map.insert("warning_rate".to_string(), warning_rate);
        map
    })
}
