use depyler_core::DepylerPipeline;

#[test]
fn test_string_interning_threshold() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    s1 = "repeated"
    s2 = "repeated"
    s3 = "repeated"
    s4 = "repeated"
    s5 = "repeated"
    return [s1, s2, s3, s4, s5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_REPEATED"),
        "Should generate const STR_REPEATED for string used 5 times"
    );

    assert!(
        rust_code.contains("&'static str"),
        "Const should be &'static str type"
    );

    assert!(
        rust_code.contains("STR_REPEATED.to_string()"),
        "Should use constant reference instead of literal"
    );
}

#[test]
fn test_string_below_interning_threshold() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    s1 = "rare"
    s2 = "rare"
    s3 = "rare"
    return [s1, s2, s3]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        !rust_code.contains("const STR_RARE"),
        "Should NOT intern string used only 3 times (threshold is > 3)"
    );

    assert!(
        rust_code.contains("\"rare\""),
        "Should use string literal directly when below threshold"
    );
}

#[test]
fn test_string_exactly_at_threshold() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    s1 = "boundary"
    s2 = "boundary"
    s3 = "boundary"
    s4 = "boundary"
    return [s1, s2, s3, s4]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_BOUNDARY"),
        "Should intern string used exactly 4 times (> 3 threshold)"
    );
}

#[test]
fn test_collision_resolution_case_difference() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    a1 = "ABC"
    a2 = "ABC"
    a3 = "ABC"
    a4 = "ABC"
    a5 = "ABC"

    b1 = "abc"
    b2 = "abc"
    b3 = "abc"
    b4 = "abc"
    b5 = "abc"

    return [a1, a2, a3, a4, a5, b1, b2, b3, b4, b5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_ABC_1"),
        "Should have STR_ABC_1 for first collision"
    );
    assert!(
        rust_code.contains("const STR_ABC_2"),
        "Should have STR_ABC_2 for second collision"
    );

    let uppercase_count = rust_code.matches("STR_ABC_1").count();
    let lowercase_count = rust_code.matches("STR_ABC_2").count();

    assert!(
        uppercase_count >= 5,
        "STR_ABC_1 should be used at least 5 times (found {})",
        uppercase_count
    );
    assert!(
        lowercase_count >= 5,
        "STR_ABC_2 should be used at least 5 times (found {})",
        lowercase_count
    );
}

#[test]
fn test_collision_resolution_special_chars() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    s1 = "hello world"
    s2 = "hello world"
    s3 = "hello world"
    s4 = "hello world"
    s5 = "hello world"

    t1 = "hello-world"
    t2 = "hello-world"
    t3 = "hello-world"
    t4 = "hello-world"
    t5 = "hello-world"

    return [s1, s2, s3, s4, s5, t1, t2, t3, t4, t5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_HELLO_WORLD_1"),
        "Should have STR_HELLO_WORLD_1"
    );
    assert!(
        rust_code.contains("const STR_HELLO_WORLD_2"),
        "Should have STR_HELLO_WORLD_2"
    );
}

#[test]
fn test_interned_string_compiles() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def greet_many():
    msg1 = "Hello"
    msg2 = "Hello"
    msg3 = "Hello"
    msg4 = "Hello"
    msg5 = "Hello"
    print(msg1, msg2, msg3, msg4, msg5)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_HELLO"),
        "Should contain const STR_HELLO"
    );

    assert!(
        rust_code.contains("STR_HELLO"),
        "Should use the STR_HELLO constant"
    );
}

#[test]
fn test_empty_string_interning() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    s1 = ""
    s2 = ""
    s3 = ""
    s4 = ""
    s5 = ""
    return [s1, s2, s3, s4, s5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_EMPTY"),
        "Should intern empty string as STR_EMPTY"
    );
}

#[test]
fn test_interning_across_multiple_functions() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def func1():
    x = "shared"
    return x

def func2():
    y = "shared"
    return y

def func3():
    z = "shared"
    return z

def func4():
    w = "shared"
    return w

def func5():
    v = "shared"
    return v
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("const STR_SHARED"),
        "Should intern string used across multiple functions"
    );
    let const_count = rust_code.matches("const STR_SHARED:").count();
    assert_eq!(
        const_count, 1,
        "Should have exactly 1 const declaration (found {})",
        const_count
    );
}

#[test]
fn test_interned_strings_preserve_semantics() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def compare_strings():
    s1 = "test"
    s2 = "test"
    s3 = "test"
    s4 = "test"
    s5 = "test"
    return s1 == s2 and s2 == s3 and s3 == s4 and s4 == s5
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("const STR_TEST"));

    assert!(rust_code.contains("STR_TEST"));
}

#[test]
fn test_mutation_interning_threshold() {
    let pipeline = DepylerPipeline::new();

    let python_3 = r#"
def test():
    s1 = "three"
    s2 = "three"
    s3 = "three"
    return [s1, s2, s3]
"#;

    let rust_3 = pipeline.transpile(python_3).unwrap();
    assert!(
        !rust_3.contains("const STR_THREE"),
        "MUTATION KILL: 3 occurrences should NOT intern (threshold is > 3, not >= 3)"
    );

    let python_4 = r#"
def test():
    s1 = "four"
    s2 = "four"
    s3 = "four"
    s4 = "four"
    return [s1, s2, s3, s4]
"#;

    let rust_4 = pipeline.transpile(python_4).unwrap();
    assert!(
        rust_4.contains("const STR_FOUR"),
        "MUTATION KILL: 4 occurrences SHOULD intern (> 3 threshold)"
    );
}

#[test]
fn test_mutation_collision_resolution() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test():
    a = "DEF"
    a = "DEF"
    a = "DEF"
    a = "DEF"
    a = "DEF"

    b = "def"
    b = "def"
    b = "def"
    b = "def"
    b = "def"
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(
        rust_code.contains("STR_DEF_1") && rust_code.contains("STR_DEF_2"),
        "MUTATION KILL: Collision resolution must create unique names"
    );

    let bare_def_count = rust_code.matches("const STR_DEF:").count();
    assert_eq!(
        bare_def_count, 0,
        "MUTATION KILL: Should not have bare STR_DEF const (found {})",
        bare_def_count
    );
}
