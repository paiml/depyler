def lambda_handler(event, context):
    """Simple S3 Lambda handler for testing."""
    for record in event['Records']:
        bucket = record['s3']['bucket']['name']
        key = record['s3']['object']['key']
        print(f"Processing {key} from {bucket}")
    
    return {'statusCode': 200}