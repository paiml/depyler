use std::collections::HashMap;
    #[derive(Debug, Clone)] pub struct IndexError {
    message: String ,
}
impl std::fmt::Display for IndexError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "index out of range: {}", self.message)
}
} impl std::error::Error for IndexError {
   
}
impl IndexError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Simple S3 Lambda handler for testing."] pub fn lambda_handler<'a>(event: & 'a DynamicType, context: DynamicType)  -> Result<DynamicType, IndexError>{
    for record in event.get("Records").cloned().unwrap_or_default() {
    let bucket = record.get("s3").cloned().unwrap_or_default().get("bucket").cloned().unwrap_or_default().get("name").cloned().unwrap_or_default();
    let key = record.get("s3").cloned().unwrap_or_default().get("object").cloned().unwrap_or_default().get("key").cloned().unwrap_or_default();
    print(format !("Processing {} from {}", key, bucket));
   
}
return Ok({ let mut map = HashMap::new();
    map.insert("statusCode", 200);
    map })
}