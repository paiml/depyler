# @depyler: string_strategy = "zero_copy"
# @depyler: optimization_level = "size"
from typing import List, Dict, Optional

class CSVParser:
    """Simple CSV parser without external dependencies"""
    
    def __init__(self, delimiter: str = ",", quote_char: str = '"') -> None:
        self.delimiter = delimiter
        self.quote_char = quote_char
    
    def parse_line(self, line: str) -> List[str]:
        """Parse a single CSV line into fields"""
        fields: List[str] = []
        current_field = ""
        in_quotes = False
        i = 0
        
        while i < len(line):
            char = line[i]
            
            if char == self.quote_char:
                if in_quotes and i + 1 < len(line) and line[i + 1] == self.quote_char:
                    # Escaped quote
                    current_field += self.quote_char
                    i += 2
                else:
                    # Toggle quote state
                    in_quotes = not in_quotes
                    i += 1
            elif char == self.delimiter and not in_quotes:
                # Field separator
                fields.append(current_field)
                current_field = ""
                i += 1
            else:
                # Regular character
                current_field += char
                i += 1
        
        # Add the last field
        fields.append(current_field)
        return fields
    
    def parse_string(self, csv_content: str) -> List[List[str]]:
        """Parse entire CSV string into rows and fields"""
        lines = csv_content.split('\n')
        rows: List[List[str]] = []
        
        for line in lines:
            stripped = line.strip()
            if stripped:  # Skip empty lines
                fields = self.parse_line(stripped)
                rows.append(fields)
        
        return rows
    
    def to_dict_list(self, csv_content: str) -> List[Dict[str, str]]:
        """Parse CSV and return list of dictionaries using first row as headers"""
        rows = self.parse_string(csv_content)
        if not rows:
            return []
        
        headers = rows[0]
        result: List[Dict[str, str]] = []
        
        for row in rows[1:]:
            row_dict: Dict[str, str] = {}
            for i, value in enumerate(row):
                if i < len(headers):
                    row_dict[headers[i]] = value
                else:
                    # Handle extra columns
                    row_dict[f"column_{i}"] = value
            result.append(row_dict)
        
        return result

def calculate_column_stats(csv_content: str, column_name: str) -> Dict[str, float]:
    """Calculate basic statistics for a numeric column in CSV"""
    parser = CSVParser()
    dict_rows = parser.to_dict_list(csv_content)
    
    if not dict_rows or column_name not in dict_rows[0]:
        return {"count": 0.0, "sum": 0.0, "mean": 0.0, "min": 0.0, "max": 0.0}
    
    values: List[float] = []
    for row in dict_rows:
        try:
            value = float(row[column_name])
            values.append(value)
        except:
            continue  # Skip non-numeric values
    
    if not values:
        return {"count": 0.0, "sum": 0.0, "mean": 0.0, "min": 0.0, "max": 0.0}
    
    total = sum(values)
    count = len(values)
    mean_val = total / count
    min_val = min(values)
    max_val = max(values)
    
    return {
        "count": float(count),
        "sum": total,
        "mean": mean_val,
        "min": min_val,
        "max": max_val
    }

def filter_csv_rows(csv_content: str, column_name: str, condition_value: str) -> str:
    """Filter CSV rows where column equals condition_value"""
    parser = CSVParser()
    rows = parser.parse_string(csv_content)
    
    if not rows:
        return ""
    
    headers = rows[0]
    if column_name not in headers:
        return csv_content  # Return original if column not found
    
    column_index = headers.index(column_name)
    filtered_rows = [headers]  # Keep headers
    
    for row in rows[1:]:
        if column_index < len(row) and row[column_index] == condition_value:
            filtered_rows.append(row)
    
    # Convert back to CSV string
    result_lines: List[str] = []
    for row in filtered_rows:
        # Simple CSV formatting (doesn't handle quotes properly)
        line = ",".join(row)
        result_lines.append(line)
    
    return "\n".join(result_lines)

def group_by_column(csv_content: str, group_column: str) -> Dict[str, List[Dict[str, str]]]:
    """Group CSV rows by values in specified column"""
    parser = CSVParser()
    dict_rows = parser.to_dict_list(csv_content)
    
    groups: Dict[str, List[Dict[str, str]]] = {}
    
    for row in dict_rows:
        if group_column in row:
            group_key = row[group_column]
            if group_key not in groups:
                groups[group_key] = []
            groups[group_key].append(row)
    
    return groups