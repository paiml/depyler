# AWS Lambda function for demonstration (simplified for transpiler)


def classify_file_type(filename: str) -> str:
    """Classify file type based on extension."""
    if filename.endswith(".jpg"):
        return "image/jpeg"
    if filename.endswith(".jpeg"):
        return "image/jpeg"
    if filename.endswith(".png"):
        return "image/png"
    if filename.endswith(".pdf"):
        return "document/pdf"
    if filename.endswith(".json"):
        return "application/json"
    return "unknown"


def compute_size_mb(size_bytes: int) -> int:
    """Compute size in megabytes (integer, truncated)."""
    if size_bytes <= 0:
        return 0
    mb: int = size_bytes // (1024 * 1024)
    return mb


def process_file_record(bucket_name: str, file_path: str, file_size: int) -> int:
    """Process a single file record and return 1 if successful."""
    file_type: str = classify_file_type(file_path)
    if len(file_type) == 0:
        return 0
    if len(bucket_name) == 0:
        return 0
    return 1


def process_batch(
    buckets: list[str], paths: list[str], sizes: list[int]
) -> int:
    """Process a batch of file records. Returns count of processed files."""
    n: int = len(buckets)
    processed: int = 0
    total_size: int = 0
    i: int = 0
    while i < n:
        ok: int = process_file_record(buckets[i], paths[i], sizes[i])
        if ok == 1:
            processed = processed + 1
            total_size = total_size + sizes[i]
        i = i + 1
    return processed


def build_response(status_code: int, file_count: int, total_bytes: int) -> int:
    """Build a response status. Returns status code if valid."""
    if file_count < 0:
        return 400
    if status_code != 200 and status_code != 400:
        return 500
    return status_code


def test_lambda_handler() -> int:
    """Test the lambda handler logic."""
    ft: str = classify_file_type("photo.jpg")
    if ft != "image/jpeg":
        return 0

    ft2: str = classify_file_type("data.json")
    if ft2 != "application/json":
        return 0

    ft3: str = classify_file_type("readme.txt")
    if ft3 != "unknown":
        return 0

    mb: int = compute_size_mb(2097152)
    if mb != 2:
        return 0

    mb2: int = compute_size_mb(500)
    if mb2 != 0:
        return 0

    ok: int = process_file_record("my-bucket", "data/file.json", 1024)
    if ok != 1:
        return 0

    buckets: list[str] = ["b1", "b2", "b3"]
    paths: list[str] = ["f1.jpg", "f2.png", "f3.pdf"]
    sizes: list[int] = [100, 200, 300]
    cnt: int = process_batch(buckets, paths, sizes)
    if cnt != 3:
        return 0

    resp: int = build_response(200, 3, 600)
    if resp != 200:
        return 0

    return 1


if __name__ == "__main__":
    result: int = test_lambda_handler()
    if result != 1:
        raise ValueError("lambda_demo test failed")
