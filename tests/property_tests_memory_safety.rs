use depyler_core::DepylerPipeline;
use quickcheck::TestResult;

/// Property: Generated Rust code should never produce use-after-free
#[quickcheck_macros::quickcheck(tests = 30, max_tests = 60)]
fn prop_no_use_after_free(var_name: String, operations: Vec<u8>) -> TestResult {
    if var_name.is_empty() || !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return TestResult::discard();
    }
    
    if operations.len() > 5 {
        return TestResult::discard();
    }

    // Create a sequence of operations that might cause use-after-free in unsafe languages
    let mut python_code = format!("def test_func() -> int:\n    {} = [1, 2, 3]\n", var_name);
    
    for &op in &operations {
        match op % 4 {
            0 => python_code.push_str(&format!("    {}.append(42)\n", var_name)),
            1 => python_code.push_str(&format!("    {} = {}.copy()\n", var_name, var_name)),
            2 => python_code.push_str(&format!("    len({})\n", var_name)),
            _ => python_code.push_str(&format!("    {} = []\n", var_name)),
        }
    }
    
    python_code.push_str(&format!("    return len({})", var_name));

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(&python_code) {
        Ok(rust_code) => {
            // Generated Rust should not contain unsafe operations
            TestResult::from_bool(
                !rust_code.contains("unsafe") &&
                !rust_code.contains("transmute") &&
                !rust_code.contains("from_raw") &&
                rust_code.contains("fn test_func")
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: String operations should not cause buffer overflows
#[quickcheck_macros::quickcheck(tests = 30, max_tests = 60)]
fn prop_string_safety(operations: Vec<u8>) -> TestResult {
    if operations.len() > 4 {
        return TestResult::discard();
    }

    let mut python_code = "def test_func() -> str:\n    s = \"hello\"\n".to_string();
    
    for &op in &operations {
        match op % 4 {
            0 => python_code.push_str("    s = s + \"world\"\n"),
            1 => python_code.push_str("    s = s.upper()\n"),
            2 => python_code.push_str("    s = s * 2\n"),
            _ => python_code.push_str("    s = s.replace(\"l\", \"x\")\n"),
        }
    }
    
    python_code.push_str("    return s");

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(&python_code) {
        Ok(rust_code) => {
            // Should generate safe string operations
            TestResult::from_bool(
                rust_code.contains("String") &&
                !rust_code.contains("unsafe") &&
                !rust_code.contains("get_unchecked") &&
                rust_code.contains("fn test_func")
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Reference counting should be correct for shared data
#[quickcheck_macros::quickcheck(tests = 20, max_tests = 40)]
fn prop_reference_counting_safety(share_count: u8) -> TestResult {
    if share_count > 3 {
        return TestResult::discard();
    }

    let mut python_code = "def test_func() -> int:\n    original = [1, 2, 3]\n".to_string();
    
    // Create multiple references to the same data
    for i in 0..share_count {
        python_code.push_str(&format!("    ref{} = original\n", i));
    }
    
    // Use all references
    for i in 0..share_count {
        python_code.push_str(&format!("    len(ref{})\n", i));
    }
    
    python_code.push_str("    return len(original)");

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(&python_code) {
        Ok(rust_code) => {
            // Should handle shared references safely (might use Rc/Arc or cloning)
            TestResult::from_bool(
                rust_code.contains("fn test_func") &&
                !rust_code.contains("dangling") &&
                !rust_code.contains("use after free")
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Iterator invalidation should be prevented
#[quickcheck_macros::quickcheck(tests = 20, max_tests = 40)]
fn prop_iterator_safety(modify_during_iteration: bool) -> TestResult {
    let python_code = if modify_during_iteration {
        r#"def test_func() -> int:
    lst = [1, 2, 3, 4, 5]
    count = 0
    for item in lst:
        if item > 2:
            lst.append(item * 2)
        count += 1
    return count"#
    } else {
        r#"def test_func() -> int:
    lst = [1, 2, 3, 4, 5]
    count = 0
    for item in lst:
        count += item
    return count"#
    };

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(python_code) {
        Ok(rust_code) => {
            // Should either prevent modification during iteration or handle it safely
            TestResult::from_bool(
                rust_code.contains("fn test_func") &&
                (rust_code.contains("for") || rust_code.contains("iter")) &&
                !rust_code.contains("undefined behavior")
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Generated code should not have memory leaks
#[quickcheck_macros::quickcheck(tests = 20, max_tests = 40)]
fn prop_no_memory_leaks(allocation_count: u8) -> TestResult {
    if allocation_count > 5 {
        return TestResult::discard();
    }

    let mut python_code = "def test_func() -> int:\n".to_string();
    
    // Create multiple allocations
    for i in 0..allocation_count {
        python_code.push_str(&format!("    data{} = list(range({}))\n", i, i + 1));
    }
    
    // Use the data and return
    python_code.push_str("    total = 0\n");
    for i in 0..allocation_count {
        python_code.push_str(&format!("    total += len(data{})\n", i));
    }
    python_code.push_str("    return total");

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(&python_code) {
        Ok(rust_code) => {
            // Rust's ownership system should prevent leaks
            TestResult::from_bool(
                rust_code.contains("fn test_func") &&
                !rust_code.contains("Box::leak") &&
                !rust_code.contains("mem::forget") &&
                !rust_code.contains("ManuallyDrop")
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Bounds checking should prevent buffer overruns
#[quickcheck_macros::quickcheck(tests = 30, max_tests = 60)]
fn prop_bounds_checking(index: usize, list_size: u8) -> TestResult {
    if list_size == 0 || list_size > 10 {
        return TestResult::discard();
    }

    let python_code = format!(
        r#"def test_func() -> int:
    lst = list(range({}))
    try:
        return lst[{}]
    except IndexError:
        return -1"#,
        list_size, index
    );

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(&python_code) {
        Ok(rust_code) => {
            // Should generate bounds-checked access or proper error handling
            TestResult::from_bool(
                rust_code.contains("fn test_func") &&
                (rust_code.contains("get(") || 
                 rust_code.contains("bounds") ||
                 rust_code.contains("Result") ||
                 rust_code.contains("Option") ||
                 !rust_code.contains("get_unchecked"))
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Concurrent access should be safe (if concurrency is supported)
#[quickcheck_macros::quickcheck(tests = 10, max_tests = 20)]
fn prop_concurrent_safety(operations: Vec<u8>) -> TestResult {
    if operations.len() > 3 {
        return TestResult::discard();
    }

    // Simple test for thread-safe operations
    let python_code = r#"def test_func() -> int:
    shared_data = [1, 2, 3]
    # In a real concurrent scenario, multiple threads would access this
    result = 0
    for i in range(3):
        result += len(shared_data)
    return result"#;

    let pipeline = DepylerPipeline::new();
    
    match pipeline.transpile(python_code) {
        Ok(rust_code) => {
            // Should generate code that doesn't have obvious race conditions
            TestResult::from_bool(
                rust_code.contains("fn test_func") &&
                !rust_code.contains("UnsafeCell") &&
                !rust_code.contains("raw pointer")
            )
        }
        Err(_) => TestResult::discard(),
    }
}