# [doc = " Depyler: verified panic-free"] # [doc = " Depyler: proven to terminate"] pub fn classify_number (n : i32) -> String {
    "Classify a number as zero, positive, or negative." . to_string ();
    if (n == 0) {
    return "zero" . to_string ();
   
}
else {
    if (n > 0) {
    return "positive" . to_string ();
   
}
else {
    return "negative" . to_string ();
   
}
} }