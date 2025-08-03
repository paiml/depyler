# Test CSV module mapping
import csv
from typing import List, Dict

def read_csv_rows(filename: str) -> List[List[str]]:
    """Read CSV file as list of rows"""
    rows = []
    with open(filename, 'r') as f:
        reader = csv.reader(f)
        for row in reader:
            rows.append(row)
    return rows

def write_csv_data(filename: str, data: List[List[str]]) -> None:
    """Write data to CSV file"""
    with open(filename, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerows(data)

def read_csv_dict(filename: str) -> List[Dict[str, str]]:
    """Read CSV file as list of dictionaries"""
    records = []
    with open(filename, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            records.append(dict(row))
    return records