use base64;
use serde_json as json;
use serde_json;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "\n    Process S3 events and return processed results.\n    \n    This function demonstrates:\n    - S3 event processing\n    - Error handling\n    - JSON response formatting\n    "]
pub fn lambda_handler(
    event: &HashMap<String, Any>,
    context: Any,
) -> Result<HashMap<String, Any>, IndexError> {
    let _cse_temp_0 = !event.contains_key(&"Records");
    if _cse_temp_0 {
        return Ok({
            let mut map = HashMap::new();
            map.insert("statusCode".to_string(), 400);
            map.insert(
                "body".to_string(),
                serde_json::to_string(&{
                    let mut map = HashMap::new();
                    map.insert(
                        "error".to_string().to_string(),
                        "Invalid event format".to_string(),
                    );
                    map
                })
                .unwrap(),
            );
            map
        });
    }
    let mut processed_files = vec![];
    let mut total_size = 0;
    for record in event.get("Records").cloned().unwrap_or_default() {
        if record.contains_key(&"s3") {
            let bucket = record
                .get("s3")
                .cloned()
                .unwrap_or_default()
                .get("bucket")
                .cloned()
                .unwrap_or_default()
                .get("name")
                .cloned()
                .unwrap_or_default();
            let key = record
                .get("s3")
                .cloned()
                .unwrap_or_default()
                .get("object")
                .cloned()
                .unwrap_or_default()
                .get("key")
                .cloned()
                .unwrap_or_default();
            let size = record
                .get("s3")
                .cloned()
                .unwrap_or_default()
                .get("object")
                .cloned()
                .unwrap_or_default()
                .get(&"size".to_string())
                .cloned()
                .unwrap_or(0);
            let mut file_type = "unknown";
            let mut file_type;
            if key.ends_with(".jpg") || key.ends_with(".jpeg") {
                file_type = "image/jpeg";
            } else {
                let mut file_type;
                if key.ends_with(".png") {
                    file_type = "image/png";
                } else {
                    let mut file_type;
                    if key.ends_with(".pdf") {
                        file_type = "document/pdf";
                    } else {
                        if key.ends_with(".json") {
                            file_type = "application/json".to_string();
                        }
                    }
                }
            }
            processed_files.push({
                let mut map = HashMap::new();
                map.insert("bucket".to_string().to_string(), bucket);
                map.insert("key".to_string().to_string(), key);
                map.insert("size".to_string().to_string(), size);
                map.insert("type".to_string().to_string(), file_type);
                map.insert("processed".to_string().to_string(), true);
                map
            });
            total_size = total_size + size;
        }
    }
    let result = {
        let mut map = HashMap::new();
        map.insert("files_processed".to_string(), processed_files.len() as i32);
        map.insert("total_size_bytes".to_string(), total_size);
        map.insert(
            "total_size_mb".to_string(),
            (total_size / 1048576 as f64).round() as i32,
        );
        map.insert("files".to_string(), processed_files);
        map
    };
    Ok({
        let mut map = HashMap::new();
        map.insert("statusCode".to_string(), 200);
        map.insert("headers".to_string(), {
            let mut map = HashMap::new();
            map.insert("Content-Type".to_string(), "application/json".to_string());
            map
        });
        map.insert("body".to_string(), serde_json::to_string(&result).unwrap());
        map
    })
}
