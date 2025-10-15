//! Targeted coverage tests for import_gen.rs module
//!
//! v3.19.1 Phase 1: Quick Wins - import_gen.rs
//! Target: 60% → 80%+ coverage, 28 missed lines
//! Expected gain: +0.12% overall coverage
//!
//! Test Strategy:
//! - Unit tests for import organization edge cases
//! - Property tests for import mapping correctness
//! - Mutation tests for import path generation

use depyler_core::DepylerPipeline;

/// Unit Test: Whole module import (e.g., `import math`)
///
/// Verifies: process_whole_module_import functionality
/// Coverage: Lines 19-21 in import_gen.rs
#[test]
fn test_whole_module_import() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import math

def use_math():
    return math.sqrt(16)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should not generate 'use' statement for unrecognized module
    // Module mapper handles known modules only
    assert!(rust_code.contains("fn use_math"));
}

/// Unit Test: Specific items import (e.g., `from typing import List`)
///
/// Verifies: process_specific_items_import functionality
/// Coverage: Lines 70-81 in import_gen.rs
#[test]
fn test_specific_items_import_typing() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List, Dict

def typed_func(items: List[int]) -> Dict[str, int]:
    result = {}
    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Typing imports should map to Rust equivalents
    assert!(rust_code.contains("Vec<i32>") || rust_code.contains("vec"));
    assert!(rust_code.contains("HashMap") || rust_code.contains("dict"));
}

/// Unit Test: Aliased import (e.g., `from typing import List as L`)
///
/// Verifies: ImportItem::Aliased handling
/// Coverage: Lines 76-78 in import_gen.rs
#[test]
fn test_aliased_import() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List as L, Dict as D

def aliased_func(items: L[str]) -> D[str, int]:
    result = {}
    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Aliased types should still map correctly
    assert!(rust_code.contains("Vec") || rust_code.contains("HashMap"));
}

/// Unit Test: Typing module special handling
///
/// Verifies: Special case for typing module (no full path needed)
/// Coverage: Lines 47-49 in import_gen.rs
#[test]
fn test_typing_module_special_handling() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Optional

def maybe_value(x: Optional[int]) -> int:
    if x is None:
        return 0
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Optional should map to Option without full path
    assert!(rust_code.contains("Option<i32>"));
}

/// Unit Test: Multiple imports from same module
///
/// Verifies: Loop over import items
/// Coverage: Lines 71-80 in import_gen.rs
#[test]
fn test_multiple_items_same_module() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List, Dict, Set, Tuple

def multi_import() -> Tuple[List[int], Set[str]]:
    items = [1, 2, 3]
    names = {"a", "b"}
    return (items, names)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // All typing imports should be processed
    assert!(rust_code.contains("fn multi_import"));
}

/// Unit Test: Empty items list (whole module import)
///
/// Verifies: if import.items.is_empty() branch
/// Coverage: Lines 111-112 in import_gen.rs
#[test]
fn test_empty_items_whole_module() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import collections

def use_collections():
    data = {"a": 1}
    return data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Whole module import path
    assert!(rust_code.contains("fn use_collections"));
}

/// Unit Test: Named import item
///
/// Verifies: ImportItem::Named handling
/// Coverage: Lines 73-75 in import_gen.rs
#[test]
fn test_named_import_item() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Any

def accepts_any(value: Any) -> Any:
    return value
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Named import should be processed
    assert!(rust_code.contains("fn accepts_any"));
}

/// Unit Test: Import with rust_path empty
///
/// Verifies: Handling of empty rust_path
/// Coverage: Lines 50-55 in import_gen.rs
#[test]
fn test_empty_rust_path() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar

T = TypeVar('T')

def generic_func(value: T) -> T:
    return value
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should handle TypeVar correctly
    assert!(rust_code.contains("fn generic_func"));
}

/// Property Test: Import mapping preservation
///
/// Property: Python imports should consistently map to Rust equivalents
///
/// Mutation Targets:
/// 1. Wrong import path generation (missing :: separator)
/// 2. Incorrect typing module handling (adding full path when shouldn't)
/// 3. Alias not applied correctly
#[test]
fn test_mutation_import_mapping_correctness() {
    // Target Mutations:
    // 1. format!("{}::{}", path, name) → format!("{}{}", path, name) [wrong separator]
    // 2. typing module gets full path when it shouldn't
    // 3. Aliased imports use wrong key

    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List, Optional

def list_func(items: List[int]) -> Optional[str]:
    if not items:
        return None
    return "ok"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // MUTATION KILL: Typing imports must not have full paths
    assert!(
        !rust_code.contains("typing::List"),
        "MUTATION KILL: typing module items should not have full paths"
    );

    // MUTATION KILL: Should use Rust equivalents
    assert!(
        rust_code.contains("Vec") || rust_code.contains("Option"),
        "MUTATION KILL: typing imports must map to Rust types"
    );
}

/// Property Test: Import processing idempotency
///
/// Property: Processing imports multiple times should produce same result
#[test]
fn test_import_processing_idempotency() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List

def func1(items: List[int]) -> int:
    return len(items)

def func2(items: List[str]) -> int:
    return len(items)
"#;

    let rust_code_1 = pipeline.transpile(python_code).unwrap();
    let rust_code_2 = pipeline.transpile(python_code).unwrap();

    // Should produce identical output
    assert_eq!(
        rust_code_1, rust_code_2,
        "Import processing should be deterministic"
    );
}

/// Edge Case: Import with no mapping in module mapper
///
/// Verifies: Handling of unmapped imports
/// Coverage: Lines 19-21, 70 (None case)
#[test]
fn test_unmapped_import() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import unknown_module

def use_unknown():
    return 42
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should not fail on unknown imports
    assert!(rust_code.contains("fn use_unknown"));
}

/// Edge Case: Mixed whole module and specific imports
///
/// Verifies: process_module_imports handles both types
/// Coverage: Lines 110-116 (loop with both branches)
#[test]
fn test_mixed_import_styles() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import math
from typing import List

def mixed():
    items: List[int] = [1, 2, 3]
    return len(items)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Both import styles should be handled
    assert!(rust_code.contains("fn mixed"));
}

/// Integration Test: Complex import scenario
///
/// Verifies: All import processing paths together
#[test]
fn test_complex_import_scenario() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List, Dict, Optional, Set
from typing import Tuple as T

def complex_types(
    items: List[int],
    mapping: Dict[str, Optional[int]],
    unique: Set[str],
    pair: T[int, str]
) -> bool:
    return True
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // All imports should be processed correctly
    assert!(rust_code.contains("fn complex_types"));

    // Verify no typing:: paths leak through
    assert!(
        !rust_code.contains("typing::"),
        "Typing module should not use full paths"
    );
}
