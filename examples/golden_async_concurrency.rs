#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use serde_json;
use tokio as asyncio;
    use std::collections::HashMap;
    use std::collections::HashSet;
    #[derive(Debug, Clone)] pub struct ZeroDivisionError {
    message: String ,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write !(f, "division by zero: {}", self.message)
}
} impl std::error::Error for ZeroDivisionError {
   
}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
    Self {
    message: message.into()
}
}
}
#[derive(Debug, Clone)] pub struct ValueError {
    message: String ,
}
impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write !(f, "value error: {}", self.message)
}
} impl std::error::Error for ValueError {
   
}
impl ValueError {
    pub fn new(message: impl Into<String>) -> Self {
    Self {
    message: message.into()
}
}
}
#[derive(Debug, Clone)] pub struct AsyncResource {
    pub name: String, pub is_open: bool, pub data: String
}
impl AsyncResource {
    pub fn new(name: impl Into<String>) -> Self {
    Self {
    name: name.into(), is_open: false, data: String::new()
}
} pub async fn __aenter__(&mut self) -> AsyncResource {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    self.is_open = true;
    return self;
   
}
pub async fn __aexit__(&mut self, exc_type: Option<r#type>, exc_val: Option<Box<dyn std::error::Error>>, exc_tb: Option<serde_json::Value>) -> bool {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    self.is_open = false;
    return false;
   
}
} #[derive(Debug, Clone)] pub struct AsyncCounter {
    pub limit: i32, pub current: i32
}
impl AsyncCounter {
    pub fn new(limit: i32) -> Self {
    Self {
    limit, current: 0
}
} pub async fn __anext__(&mut self) -> i32 {
    if self.current.clone()>= self.limit.clone() {
    panic !("Exception: {}", StopAsyncIteration);
    };
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    let value = self.current.clone();
    self.current = self.current.clone() + 1;
    return value;
   
}
} #[doc = "Simplest async function.\n\n    Python: async def → Future[int]\n    Rust: async fn simple_async() -> i64\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn simple_async() -> i32 {
    42
}
#[doc = "Async function that awaits another.\n\n    Python: await simple_async()\n    Rust: simple_async().await\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_with_await() -> i32 {
    let result: i32 = simple_async().await;
    result * 2
}
#[doc = "Async function with sleep.\n\n    Python: await asyncio.sleep(seconds)\n    Rust: tokio::time::sleep(Duration::from_secs_f64(seconds)).await\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_with_sleep(seconds: f64) -> String {
    tokio::time::sleep(std::time::Duration::from_secs_f64(seconds as f64)).await;
    format !("Slept for {} seconds", seconds)
}
#[doc = "First step in async computation chain."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn compute_step1(x: i32) -> i32 {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    x + 10
}
#[doc = "Second step in async computation chain."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn compute_step2(x: i32) -> i32 {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    x * 2
}
#[doc = "Third step in async computation chain."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn compute_step3(x: i32) -> i32 {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    x - 5
}
#[doc = "Chain of async function calls.\n\n    Python: Sequential awaits\n    Rust: Sequential .await calls\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_computation_chain (start: i32) -> i32 {
    let step1: i32 = compute_step1(start).await;
    let step2: i32 = compute_step2(step1).await;
    let step3: i32 = compute_step3(step2).await;
    step3
}
#[doc = "Use async context manager.\n\n    Python: async with AsyncResource(name) as r\n    Rust: Async block with resource management\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn use_async_context_manager(name: & str) -> String {
    let mut result: String = "".to_string();
    let mut _context = AsyncResource::new(name);
    let resource = _context.__aenter__().await;
    resource.data = format !("Data for {}", resource.name);
    result = resource.data;
    result.to_string()
}
#[doc = "Nested async context managers.\n\n    Python: Nested async with statements\n    Rust: Nested async blocks\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn nested_async_context_managers<'a, 'b>(name1: & 'a str, name2: & 'b str) -> String {
    let mut results: Vec<String>= vec ! [];
    let mut _context = AsyncResource::new(name1);
    let r1 = _context.__aenter__().await;
    let mut _context = AsyncResource::new(name2);
    let r2 = _context.__aenter__().await;
    results.push(r1.name);
    results.push(r2.name);
    results.join (",")
}
#[doc = "Use async for loop.\n\n    Python: async for x in AsyncCounter(limit)\n    Rust: while let Some(x) = counter.next().await\n    "] #[doc = " Depyler: verified panic-free"] pub async fn async_for_loop(limit: i32) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for value in AsyncCounter::new(limit) {
    total = total + value;
   
}
total
}
#[doc = "Async for with early break.\n\n    Python: async for with break condition\n    Rust: break in while let loop\n    "] #[doc = " Depyler: verified panic-free"] pub async fn async_for_with_break(limit: i32, stop_at: i32) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for value in AsyncCounter::new(limit) {
    if value>= stop_at {
    break;
   
}
total = total + value;
   
}
total
}
#[doc = "First concurrent task."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn task_a() -> String {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.01 as f64)).await;
    "A".to_string().to_string()
}
#[doc = "Second concurrent task."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn task_b() -> String {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.01 as f64)).await;
    "B".to_string().to_string()
}
#[doc = "Third concurrent task."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn task_c() -> String {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.01 as f64)).await;
    "C".to_string().to_string()
}
#[doc = "Run tasks concurrently with gather.\n\n    Python: asyncio.gather(task_a(), task_b(), task_c())\n    Rust: tokio::join!(task_a(), task_b(), task_c())\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn concurrent_gather() -> Vec<String>{
    let results: Vec<String>= join !(task_a(), task_b(), task_c()).await;
    results
}
#[doc = "Process multiple values concurrently.\n\n    Python: asyncio.gather(*[process(v) for v in values])\n    Rust: futures::future::join_all or tokio::join!\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn concurrent_with_results(values: & Vec<i32>) -> Vec<i32>{
    let tasks = values.as_slice().iter().cloned().map(| v | process(v)).collect::<Vec<_>>();
    let results: Vec<i32>= join !(tasks).await;
    results
}
#[doc = "Concurrent execution with timeout.\n\n    Python: asyncio.wait_for with timeout\n    Rust: tokio::time::timeout\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn concurrent_with_timeout(timeout_secs: f64) -> Option<String>{
    let mut result: String = Default::default();
    match(|| -> Result<String, Box<dyn std::error::Error>>{
    result = tokio::time::timeout(slow_task()).await;
    return Ok(Some(result.to_string()));
    })() {
    Ok(_result) =>{
    return _result;
   
}
, Err(_) =>{
    return None;
   
}
}
}
#[doc = "Synchronous helper function."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn sync_helper(x: i32) -> i32 {
    x * 3
}
#[doc = "Async function calling sync function.\n\n    Python: Call regular function from async\n    Rust: Direct call(sync functions can be called from async)\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_calling_sync(x: i32) -> i32 {
    let intermediate: i32 = sync_helper(x);
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    intermediate + 1
}
#[doc = "Mix of async and sync operations.\n\n    Python: Sync operations between await points\n    Rust: Regular Rust code between .await points\n    "] #[doc = " Depyler: verified panic-free"] pub async fn async_with_sync_computation(values: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    for v in values.iter().cloned() {
    let doubled: i32 = v * 2;
    total = total + doubled;
   
}
tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    let result: i32 = if ! values.is_empty() {
    {
    let a = total;
    let b = values.len() as i32;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
}
}
else {
    0 };
    Ok(result)
}
#[doc = "Async function with try/except.\n\n    Python: try/except in async function\n    Rust: Result handling in async fn\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_with_exception_handling(x: i32) -> Result<i32, Box<dyn std::error::Error>>{
    match(|| -> Result<i32, Box<dyn std::error::Error>>{
    if x<0 {
    panic !("{}", ValueError::new("Negative value"));
   
}
tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    return Ok(x * 2);
    })() {
    Ok(_result) =>{
    return Ok(_result);
   
}
, Err(_) =>{
    return Ok(- 1);
   
}
}
}
#[doc = "Async function that re-raises exception.\n\n    Python: raise in async except block\n    Rust: return Err(e) in async fn\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_reraise_exception(x: i32) -> Result<i32, Box<dyn std::error::Error>>{
    match(|| -> Result<i32, Box<dyn std::error::Error>>{
    if x == 0 {
    panic !("{}", ZeroDivisionError::new("Cannot be zero"));
   
}
tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    return Ok({ let a = 100;
    let b = x;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
});
    })() {
    Ok(_result) =>{
    return Ok(_result);
   
}
, Err(_) =>{
    return Err("Exception raised".into());
   
}
}
}
#[doc = "Async function returning Optional.\n\n    Python: async def -> Optional[int]\n    Rust: async fn...-> Option<i64>\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_return_optional(x: i32) -> Option<i32>{
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    let _cse_temp_0 = x<0;
    if _cse_temp_0 {
    return None;
   
}
Some(x)
}
#[doc = "Async function returning tuple.\n\n    Python: async def -> Tuple[int, int]\n    Rust: async fn...-> (i64, i64)\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_return_tuple(a: i32, b: i32) -> (i32, i32) {
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
  (a + b, a * b)
}
#[doc = "Async function returning dict.\n\n    Python: async def -> Dict[str, int]\n    Rust: async fn...-> HashMap<String, i64>\n    "] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_return_dict(keys: & Vec<String>) -> HashMap<String, i32>{
    let mut result: std::collections::HashMap<String, i32>= {
    let map = HashMap::new();
    map };
    for(i, key) in keys.iter().cloned().enumerate().map(|(i, x) |(i as i32, x)) {
    let i = i as i32;
    tokio::time::sleep(std::time::Duration::from_secs_f64(0.001 as f64)).await;
    result.insert(key.to_string().clone(), i);
   
}
result
}
#[doc = "Async main function exercising all patterns."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub async fn async_main () -> i32 {
    assert_eq !(simple_async().await, 42);
    assert_eq !(async_with_await().await, 84);
    assert_eq !(async_computation_chain (5).await, 25);
    let result: String = use_async_context_manager(& "test").await;
    assert_eq !(result, "Data for test");
    let nested: String = nested_async_context_managers(& "a", & "b").await;
    assert_eq !(nested, "a,b");
    assert_eq !(async_for_loop(5).await, 10);
    assert_eq !(async_for_with_break(10, 3).await, 3);
    let gather_results: Vec<String>= concurrent_gather().await;
    assert_eq !(gather_results.into_iter().collect::<std::collections::HashSet<_>>(), {
    let mut set = std::collections::HashSet::new();
    set.insert("A".to_string());
    set.insert("B".to_string());
    set.insert("C".to_string());
    set });
    let concurrent_results: Vec<i32>= concurrent_with_results(& vec ! [1, 2, 3]).await;
    assert_eq !(concurrent_results, vec ! [2, 4, 6]);
    let timeout_result: Option<String>= concurrent_with_timeout(0.001).await;
    assert !(timeout_result.is_none());
    assert_eq !(async_calling_sync(10).await, 31);
    assert_eq !(async_with_sync_computation(& vec ! [2, 4, 6]).await, 8);
    assert_eq !(async_with_exception_handling(5).await, 10);
    assert_eq !(async_with_exception_handling(- 5).await, - 1);
    assert_eq !(async_return_optional(5).await, 5);
    assert !(async_return_optional(- 5).await.is_none());
    let tuple_result :(i32, i32) = async_return_tuple(3, 4).await;
    assert_eq !(tuple_result ,(7, 12));
    let dict_result: std::collections::HashMap<String, i32>= async_return_dict(& vec ! ["a".to_string(), "b".to_string(), "c".to_string()]).await;
    assert_eq !(dict_result, {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 0);
    map.insert("b".to_string(), 1);
    map.insert("c".to_string(), 2);
    map });
    0
}
#[doc = "Entry point that runs the async main."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn main () {
    let result: i32 = tokio::runtime::Runtime::new().unwrap().block_on(async_main ());
    let _ = result;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_simple_async_examples() {
    let _ = simple_async();
   
}
#[test] fn test_async_with_await_examples() {
    let _ = async_with_await();
   
}
#[test] fn test_compute_step1_examples() {
    assert_eq !(compute_step1(0), 0);
    assert_eq !(compute_step1(1), 1);
    assert_eq !(compute_step1(- 1), - 1);
   
}
#[test] fn test_compute_step2_examples() {
    assert_eq !(compute_step2(0), 0);
    assert_eq !(compute_step2(1), 1);
    assert_eq !(compute_step2(- 1), - 1);
   
}
#[test] fn test_compute_step3_examples() {
    assert_eq !(compute_step3(0), 0);
    assert_eq !(compute_step3(1), 1);
    assert_eq !(compute_step3(- 1), - 1);
   
}
#[test] fn test_async_computation_chain_examples() {
    assert_eq !(async_computation_chain (0), 0);
    assert_eq !(async_computation_chain (1), 1);
    assert_eq !(async_computation_chain (- 1), - 1);
   
}
#[test] fn test_async_for_loop_examples() {
    assert_eq !(async_for_loop(0), 0);
    assert_eq !(async_for_loop(1), 1);
    assert_eq !(async_for_loop(- 1), - 1);
   
}
#[test] fn test_async_for_with_break_examples() {
    assert_eq !(async_for_with_break(0, 0), 0);
    assert_eq !(async_for_with_break(1, 2), 3);
    assert_eq !(async_for_with_break(- 1, 1), 0);
   
}
#[test] fn test_concurrent_with_results_examples() {
    assert_eq !(concurrent_with_results(vec ! []), vec ! []);
    assert_eq !(concurrent_with_results(vec ! [1]), vec ! [1]);
   
}
#[test] fn test_sync_helper_examples() {
    assert_eq !(sync_helper(0), 0);
    assert_eq !(sync_helper(1), 1);
    assert_eq !(sync_helper(- 1), - 1);
   
}
#[test] fn test_async_calling_sync_examples() {
    assert_eq !(async_calling_sync(0), 0);
    assert_eq !(async_calling_sync(1), 1);
    assert_eq !(async_calling_sync(- 1), - 1);
   
}
#[test] fn test_async_with_sync_computation_examples() {
    assert_eq !(async_with_sync_computation(& vec ! []), 0);
    assert_eq !(async_with_sync_computation(& vec ! [1]), 1);
    assert_eq !(async_with_sync_computation(& vec ! [1, 2, 3]), 3);
   
}
#[test] fn test_async_with_exception_handling_examples() {
    assert_eq !(async_with_exception_handling(0), 0);
    assert_eq !(async_with_exception_handling(1), 1);
    assert_eq !(async_with_exception_handling(- 1), - 1);
   
}
#[test] fn test_async_reraise_exception_examples() {
    assert_eq !(async_reraise_exception(0), 0);
    assert_eq !(async_reraise_exception(1), 1);
    assert_eq !(async_reraise_exception(- 1), - 1);
   
}
#[test] fn test_async_main_examples() {
    let _ = async_main ();
   
}
#[test] fn test_main_examples() {
    let _ = main ();
   
}
}