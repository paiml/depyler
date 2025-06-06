# AWS Lambda function for demonstration
import json
from typing import Dict, Any, List
import base64

def lambda_handler(event: Dict[str, Any], context: Any) -> Dict[str, Any]:
    """
    Process S3 events and return processed results.
    
    This function demonstrates:
    - S3 event processing
    - Error handling
    - JSON response formatting
    """
    
    # Check if this is an S3 event
    if 'Records' not in event:
        return {
            'statusCode': 400,
            'body': json.dumps({'error': 'Invalid event format'})
        }
    
    processed_files = []
    total_size = 0
    
    for record in event['Records']:
        # Extract S3 information
        if 's3' in record:
            bucket = record['s3']['bucket']['name']
            key = record['s3']['object']['key']
            size = record['s3']['object'].get('size', 0)
            
            # Process based on file type
            file_type = 'unknown'
            if key.endswith('.jpg') or key.endswith('.jpeg'):
                file_type = 'image/jpeg'
            elif key.endswith('.png'):
                file_type = 'image/png'
            elif key.endswith('.pdf'):
                file_type = 'document/pdf'
            elif key.endswith('.json'):
                file_type = 'application/json'
            
            processed_files.append({
                'bucket': bucket,
                'key': key,
                'size': size,
                'type': file_type,
                'processed': True
            })
            
            total_size += size
    
    # Return summary
    result = {
        'files_processed': len(processed_files),
        'total_size_bytes': total_size,
        'total_size_mb': round(total_size / (1024 * 1024), 2),
        'files': processed_files
    }
    
    return {
        'statusCode': 200,
        'headers': {
            'Content-Type': 'application/json'
        },
        'body': json.dumps(result)
    }