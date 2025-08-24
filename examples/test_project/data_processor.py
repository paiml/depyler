"""
Data processing utilities - Example for Depyler transpilation
Demonstrates list comprehensions, dict operations, and functional patterns
"""
from typing import Dict, List, Tuple, Any, Callable, Optional
from functools import reduce
import operator


class DataProcessor:
    """Process and analyze data collections."""
    
    def __init__(self, data: List[Dict[str, Any]]) -> None:
        """Initialize with data collection."""
        self.data = data
    
    def filter_by_field(self, field: str, value: Any) -> List[Dict[str, Any]]:
        """Filter records by field value."""
        return [record for record in self.data if record.get(field) == value]
    
    def map_field(self, field: str, transform: Callable[[Any], Any]) -> List[Any]:
        """Apply transformation to a specific field."""
        return [transform(record.get(field)) for record in self.data if field in record]
    
    def group_by(self, field: str) -> Dict[Any, List[Dict[str, Any]]]:
        """Group records by field value."""
        groups: Dict[Any, List[Dict[str, Any]]] = {}
        
        for record in self.data:
            key = record.get(field)
            if key not in groups:
                groups[key] = []
            groups[key].append(record)
        
        return groups
    
    def aggregate(self, field: str, operation: str = "sum") -> Optional[float]:
        """Aggregate numeric field values."""
        values = [record.get(field, 0) for record in self.data 
                 if isinstance(record.get(field), (int, float))]
        
        if not values:
            return None
        
        if operation == "sum":
            return sum(values)
        elif operation == "avg":
            return sum(values) / len(values)
        elif operation == "min":
            return min(values)
        elif operation == "max":
            return max(values)
        else:
            return None
    
    def sort_by(self, field: str, reverse: bool = False) -> List[Dict[str, Any]]:
        """Sort records by field value."""
        return sorted(self.data, key=lambda x: x.get(field, ""), reverse=reverse)
    
    def project(self, fields: List[str]) -> List[Dict[str, Any]]:
        """Project specific fields from records."""
        return [
            {field: record.get(field) for field in fields if field in record}
            for record in self.data
        ]
    
    def distinct(self, field: str) -> List[Any]:
        """Get distinct values for a field."""
        seen = set()
        result = []
        
        for record in self.data:
            value = record.get(field)
            if value not in seen:
                seen.add(value)
                result.append(value)
        
        return result


def process_numbers(numbers: List[int]) -> Dict[str, Any]:
    """Process a list of numbers with various operations."""
    if not numbers:
        return {"error": "Empty list"}
    
    # Using list comprehensions
    evens = [n for n in numbers if n % 2 == 0]
    odds = [n for n in numbers if n % 2 != 0]
    squares = [n ** 2 for n in numbers]
    
    # Using map and filter
    doubled = list(map(lambda x: x * 2, numbers))
    positive = list(filter(lambda x: x > 0, numbers))
    
    # Using reduce for aggregation
    product = reduce(operator.mul, numbers, 1)
    
    return {
        "original": numbers,
        "count": len(numbers),
        "sum": sum(numbers),
        "average": sum(numbers) / len(numbers),
        "min": min(numbers),
        "max": max(numbers),
        "evens": evens,
        "odds": odds,
        "squares": squares,
        "doubled": doubled,
        "positive": positive,
        "product": product
    }


def transform_text(text: str) -> Dict[str, Any]:
    """Transform and analyze text."""
    words = text.split()
    
    # Word statistics
    word_count = len(words)
    char_count = len(text)
    
    # Word frequency using dict comprehension
    word_freq = {}
    for word in words:
        word_lower = word.lower()
        word_freq[word_lower] = word_freq.get(word_lower, 0) + 1
    
    # Find longest and shortest words
    longest = max(words, key=len) if words else ""
    shortest = min(words, key=len) if words else ""
    
    # Character frequency
    char_freq = {char: text.count(char) for char in set(text)}
    
    return {
        "original": text,
        "word_count": word_count,
        "char_count": char_count,
        "words": words,
        "unique_words": list(set(words)),
        "word_frequency": word_freq,
        "longest_word": longest,
        "shortest_word": shortest,
        "char_frequency": char_freq,
        "reversed": text[::-1],
        "uppercase": text.upper(),
        "lowercase": text.lower()
    }


def matrix_operations(matrix: List[List[int]]) -> Dict[str, Any]:
    """Perform operations on 2D matrix."""
    if not matrix or not matrix[0]:
        return {"error": "Invalid matrix"}
    
    rows = len(matrix)
    cols = len(matrix[0])
    
    # Transpose using list comprehension
    transposed = [[matrix[i][j] for i in range(rows)] for j in range(cols)]
    
    # Flatten matrix
    flattened = [elem for row in matrix for elem in row]
    
    # Row and column sums
    row_sums = [sum(row) for row in matrix]
    col_sums = [sum(matrix[i][j] for i in range(rows)) for j in range(cols)]
    
    # Find diagonal elements (if square matrix)
    diagonal = [matrix[i][i] for i in range(min(rows, cols))]
    
    return {
        "original": matrix,
        "rows": rows,
        "cols": cols,
        "transposed": transposed,
        "flattened": flattened,
        "row_sums": row_sums,
        "col_sums": col_sums,
        "diagonal": diagonal,
        "total_sum": sum(flattened),
        "max_element": max(flattened),
        "min_element": min(flattened)
    }


def pipeline_example(data: List[int]) -> int:
    """Example of functional pipeline pattern."""
    # Simulate pipeline: filter -> map -> reduce
    result = reduce(
        operator.add,
        map(lambda x: x ** 2,
            filter(lambda x: x % 2 == 0, data)),
        0
    )
    return result


def main() -> None:
    """Test data processing functions."""
    # Test DataProcessor
    sample_data = [
        {"id": 1, "name": "Alice", "age": 30, "city": "NYC"},
        {"id": 2, "name": "Bob", "age": 25, "city": "LA"},
        {"id": 3, "name": "Charlie", "age": 35, "city": "NYC"},
        {"id": 4, "name": "Diana", "age": 28, "city": "Chicago"}
    ]
    
    processor = DataProcessor(sample_data)
    print("NYC residents:", processor.filter_by_field("city", "NYC"))
    print("Grouped by city:", processor.group_by("city"))
    print("Average age:", processor.aggregate("age", "avg"))
    print("Distinct cities:", processor.distinct("city"))
    
    # Test number processing
    numbers = [1, 2, 3, 4, 5, -1, -2, 0]
    print("\nNumber processing:", process_numbers(numbers))
    
    # Test text processing
    text = "Hello World from Depyler"
    print("\nText analysis:", transform_text(text))
    
    # Test matrix operations
    matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]
    print("\nMatrix operations:", matrix_operations(matrix))
    
    # Test pipeline
    print("\nPipeline result:", pipeline_example([1, 2, 3, 4, 5, 6]))


if __name__ == "__main__":
    main()