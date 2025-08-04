#[doc = "// Python import: os"] #[doc = "// Python import: sys"] #[doc = "// Python import: re"] use serde_json::from_str;
    use serde_json::to_string;
    use std::path::Path::join as path_join;
    use std::collections::HashMap;
    #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_current_dir()  -> String {
    return std::env::current_dir().unwrap().to_string_lossy().to_string();
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn parse_json(data: String)  -> HashMap<String, serde_json::Value>{
    return serde_json::from_str(data);
   
}
#[doc = " Depyler: verified panic-free"] pub fn join_paths<'a>(base: & 'a str)  -> String {
    let mut result = base;
    for p in paths.iter() {
    result = std::path::Path::join (result, p);
   
}
return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn find_pattern<'a, 'b>(text: & 'a str, pattern: & 'b str)  -> Vec<String>{
    let mut regex = regex::Regex::new(pattern).unwrap();
    return regex.find_iter(text).map(| m | m.as_str().to_string()).collect::<Vec<String>>()
}