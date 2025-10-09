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
#[doc = "Shows accumulator pattern."] #[doc = " Depyler: verified panic-free"] pub fn accumulator_example<'a>(items: & 'a DynamicType)  -> DynamicType {
    let result = vec ! [];
    for item in items.iter() {
    result.push(item * 2);
   
}
return result;
   
}
#[doc = "Shows inefficient string building."] #[doc = " Depyler: verified panic-free"] pub fn string_concat_example<'a>(values: & 'a DynamicType)  -> DynamicType {
    let mut output = "";
    for val in values.iter() {
    output = output + str(val);
   
}
return output;
   
}
#[doc = "Shows range(len()) antipattern."] #[doc = " Depyler: proven to terminate"] pub fn enumerate_example<'a>(data: & 'a DynamicType)  -> Result<DynamicType, IndexError>{
    for i in 0..data.len() {
    print(i, data.get(i as usize).copied().unwrap_or_default());
   
}
} #[doc = "Shows while True pattern."] #[doc = " Depyler: verified panic-free"] pub fn while_true_example()  -> DynamicType {
    let mut count = 0;
    while true {
    count = count + 1;
    if count>10 {
    break;
   
}
} return count
}