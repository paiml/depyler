📄 Source: examples/test_async_methods.py (647 bytes)
📝 Output: examples/test_async_methods.rs (703 bytes)
⏱️  Parse time: 7ms
📊 Throughput: 88.8 KB/s
⏱️  Total time: 7ms
-> i32 {
    self._simulate_delay().await;
    self.value = self.value + 1;
    return self.value;
   
}
pub fn get_value(& mut self)  -> i32 {
    return self.value;
   
}
pub fn _simulate_delay(& mut self) {
    {
}
}
}
#[derive(Debug, Clone)] pub struct AsyncDataProcessor {
   
}
impl AsyncDataProcessor {
    pub fn new()  -> Self {
    Self {
   
}
} pub fn process(& mut self, data: String)  -> String {
    self._async_work().await;
    return data.upper();
   
}
pub fn _async_work(& mut self) {
    {
}
} }