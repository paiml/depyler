def classify_number(n: int) -> str:
    """Classify a number as zero, positive, or negative."""
    if n == 0:
        return "zero"
    elif n > 0:
        return "positive"
    else:
        return "negative"