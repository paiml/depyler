# [doc = " Depyler: proven to terminate"] pub fn process_config (config : & HashMap < String , String >) -> Option < String > {
    "Process configuration dictionary and return debug value if present." . to_string ();
    if config . contains_key (& "debug" . to_string ()) {
    return config . get ("debug" . to_string () as usize) . copied () . unwrap_or_default ();
   
}
return ();
    }