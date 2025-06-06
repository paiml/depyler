/// Utilities for WASM-specific functionality
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Log to browser console
#[allow(dead_code)]
pub fn console_log(msg: &str) {
    log(msg);
}

/// Set up panic hook for better error messages in browser console
#[allow(dead_code)]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Measure memory usage in the browser
#[allow(dead_code)]
pub fn get_memory_info() -> Option<MemoryInfo> {
    // The memory API is not standard and not available in web-sys
    // Return None for now - in a real implementation, you could:
    // 1. Use JavaScript interop to access navigator.memory if available
    // 2. Track WASM memory growth manually
    // 3. Use performance.measureUserAgentSpecificMemory() when available
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MemoryInfo {
    pub used_js_heap_size: u32,
    pub total_js_heap_size: u32,
    pub js_heap_size_limit: u32,
}

#[allow(dead_code)]
impl MemoryInfo {
    pub fn used_mb(&self) -> f64 {
        self.used_js_heap_size as f64 / 1_048_576.0
    }

    pub fn total_mb(&self) -> f64 {
        self.total_js_heap_size as f64 / 1_048_576.0
    }
}

/// Performance timing utilities
#[allow(dead_code)]
pub struct Timer {
    start_time: f64,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        let start_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        Self { start_time }
    }

    pub fn elapsed_ms(&self) -> f64 {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now() - self.start_time)
            .unwrap_or(0.0)
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
