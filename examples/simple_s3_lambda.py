def parse_s3_record(bucket_name: str, object_path: str) -> str:
    """Parse an S3 record and return a processing message."""
    result: str = "Processing " + object_path + " from " + bucket_name
    return result


def process_s3_event(bucket_name: str, object_path: str) -> int:
    """Process an S3 event and return status code."""
    msg: str = parse_s3_record(bucket_name, object_path)
    if len(msg) > 0:
        return 200
    return 400


def test_s3_handler() -> int:
    """Test the S3 handler logic."""
    status: int = process_s3_event("my-bucket", "data/file.txt")
    if status != 200:
        return 0
    msg: str = parse_s3_record("test-bucket", "uploads/photo.jpg")
    if len(msg) == 0:
        return 0
    return 1


if __name__ == "__main__":
    result: int = test_s3_handler()
    if result != 1:
        raise ValueError("test failed")
