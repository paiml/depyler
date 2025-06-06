# Example showcasing Depyler v0.2 annotation system

# @depyler: optimization_level = "aggressive"
# @depyler: thread_safety = "required"
# @depyler: bounds_checking = "explicit"
def parallel_sum(numbers: List[int]) -> int:
    """Compute sum with parallel processing hints."""
    total = 0
    for num in numbers:
        total += num
    return total

# @depyler: string_strategy = "zero_copy"
# @depyler: ownership = "borrowed"
def process_text(text: str) -> str:
    """Process text with zero-copy string strategy."""
    return text.upper()

# @depyler: hash_strategy = "fnv"
# @depyler: ownership = "owned"
def count_words(text: str) -> Dict[str, int]:
    """Count word frequencies with FNV hash strategy."""
    word_count = {}
    words = text.split()
    for word in words:
        if word in word_count:
            word_count[word] += 1
        else:
            word_count[word] = 1
    return word_count

# @depyler: panic_behavior = "convert_to_result"
# @depyler: error_strategy = "result_type"
def safe_divide(a: int, b: int) -> Optional[float]:
    """Safe division with Result type."""
    if b == 0:
        return None
    return a / b

# @depyler: optimization_hint = "vectorize"
# @depyler: performance_critical = "true"
def dot_product(v1: List[float], v2: List[float]) -> float:
    """Compute dot product with SIMD hints."""
    result = 0.0
    for i in range(len(v1)):
        result += v1[i] * v2[i]
    return result