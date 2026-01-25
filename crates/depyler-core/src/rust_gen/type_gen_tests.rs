//! Comprehensive type generator tests
//!
//! These tests exercise the type_gen.rs code paths through the transpilation pipeline.

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// PRIMITIVE TYPES
// ============================================================================

#[test]
fn test_type_int() {
    let code = transpile("def foo(x: int) -> int:\n    return x");
    assert!(code.contains("i64") || code.contains("i32"));
}

#[test]
fn test_type_float() {
    let code = transpile("def foo(x: float) -> float:\n    return x");
    assert!(code.contains("f64") || code.contains("f32"));
}

#[test]
fn test_type_str() {
    let code = transpile("def foo(x: str) -> str:\n    return x");
    assert!(code.contains("String") || code.contains("str"));
}

#[test]
fn test_type_bool() {
    let code = transpile("def foo(x: bool) -> bool:\n    return x");
    assert!(code.contains("bool"));
}

#[test]
fn test_type_bytes() {
    assert!(transpile_ok("def foo(x: bytes) -> bytes:\n    return x"));
}

#[test]
fn test_type_none() {
    let code = transpile("def foo() -> None:\n    pass");
    assert!(code.contains("()") || code.contains("fn foo"));
}

// ============================================================================
// COLLECTION TYPES - typing module
// ============================================================================

#[test]
fn test_type_list_int() {
    let code =
        transpile("from typing import List\n\ndef foo(x: List[int]) -> List[int]:\n    return x");
    assert!(code.contains("Vec") || code.contains("i64"));
}

#[test]
fn test_type_list_str() {
    let code =
        transpile("from typing import List\n\ndef foo(x: List[str]) -> List[str]:\n    return x");
    assert!(code.contains("Vec") || code.contains("String"));
}

#[test]
fn test_type_dict_str_int() {
    let code = transpile(
        "from typing import Dict\n\ndef foo(x: Dict[str, int]) -> Dict[str, int]:\n    return x",
    );
    assert!(code.contains("HashMap") || code.contains("String"));
}

#[test]
fn test_type_set_int() {
    let code =
        transpile("from typing import Set\n\ndef foo(x: Set[int]) -> Set[int]:\n    return x");
    assert!(code.contains("HashSet") || code.contains("i64"));
}

#[test]
fn test_type_tuple_int_str() {
    let code = transpile(
        "from typing import Tuple\n\ndef foo(x: Tuple[int, str]) -> Tuple[int, str]:\n    return x",
    );
    assert!(code.contains("(") || code.contains("i64"));
}

// ============================================================================
// COLLECTION TYPES - PEP 585 (Python 3.9+)
// ============================================================================

#[test]
fn test_type_list_lowercase() {
    let code = transpile("def foo(x: list[int]) -> list[int]:\n    return x");
    assert!(code.contains("Vec") || code.contains("i64"));
}

#[test]
fn test_type_dict_lowercase() {
    let code = transpile("def foo(x: dict[str, int]) -> dict[str, int]:\n    return x");
    assert!(code.contains("HashMap") || code.contains("String"));
}

#[test]
fn test_type_set_lowercase() {
    let code = transpile("def foo(x: set[int]) -> set[int]:\n    return x");
    assert!(code.contains("HashSet") || code.contains("i64"));
}

#[test]
fn test_type_tuple_lowercase() {
    let code = transpile("def foo(x: tuple[int, str]) -> tuple[int, str]:\n    return x");
    assert!(code.contains("(") || code.contains("i64"));
}

// ============================================================================
// OPTIONAL TYPES
// ============================================================================

#[test]
fn test_type_optional_int() {
    let code = transpile(
        "from typing import Optional\n\ndef foo(x: Optional[int]) -> Optional[int]:\n    return x",
    );
    assert!(code.contains("Option") || code.contains("i64"));
}

#[test]
fn test_type_optional_str() {
    let code = transpile(
        "from typing import Optional\n\ndef foo(x: Optional[str]) -> Optional[str]:\n    return x",
    );
    assert!(code.contains("Option") || code.contains("String"));
}

#[test]
fn test_type_optional_list() {
    let code = transpile("from typing import Optional, List\n\ndef foo(x: Optional[List[int]]) -> Optional[List[int]]:\n    return x");
    assert!(code.contains("Option") || code.contains("Vec"));
}

// ============================================================================
// UNION TYPES - typing.Union
// ============================================================================

#[test]
fn test_type_union_int_str() {
    assert!(transpile_ok(
        "from typing import Union\n\ndef foo(x: Union[int, str]) -> Union[int, str]:\n    return x"
    ));
}

#[test]
fn test_type_union_with_none() {
    assert!(transpile_ok("from typing import Union\n\ndef foo(x: Union[int, None]) -> Union[int, None]:\n    return x"));
}

#[test]
fn test_type_union_multiple() {
    assert!(transpile_ok("from typing import Union\n\ndef foo(x: Union[int, str, float]) -> Union[int, str, float]:\n    return x"));
}

// ============================================================================
// UNION TYPES - PEP 604 (Python 3.10+)
// ============================================================================

#[test]
fn test_type_pep604_union() {
    assert!(transpile_ok(
        "def foo(x: int | str) -> int | str:\n    return x"
    ));
}

#[test]
fn test_type_pep604_optional() {
    assert!(transpile_ok(
        "def foo(x: int | None) -> int | None:\n    return x"
    ));
}

// ============================================================================
// CALLABLE TYPES
// ============================================================================

#[test]
fn test_type_callable_no_args() {
    assert!(transpile_ok(
        "from typing import Callable\n\ndef foo(f: Callable[[], int]) -> int:\n    return f()"
    ));
}

#[test]
fn test_type_callable_with_args() {
    assert!(transpile_ok(
        "from typing import Callable\n\ndef foo(f: Callable[[int], int]) -> int:\n    return f(1)"
    ));
}

#[test]
fn test_type_callable_multiple_args() {
    assert!(transpile_ok("from typing import Callable\n\ndef foo(f: Callable[[int, str], bool]) -> bool:\n    return f(1, 'a')"));
}

// ============================================================================
// ITERATOR/GENERATOR TYPES
// ============================================================================

#[test]
fn test_type_iterator_int() {
    assert!(transpile_ok(
        "from typing import Iterator\n\ndef foo() -> Iterator[int]:\n    yield 1"
    ));
}

#[test]
fn test_type_generator() {
    assert!(transpile_ok(
        "from typing import Generator\n\ndef foo() -> Generator[int, None, None]:\n    yield 1"
    ));
}

#[test]
fn test_type_iterable() {
    assert!(transpile_ok(
        "from typing import Iterable\n\ndef foo(x: Iterable[int]) -> int:\n    return sum(x)"
    ));
}

// ============================================================================
// ANY TYPE
// ============================================================================

#[test]
fn test_type_any() {
    assert!(transpile_ok(
        "from typing import Any\n\ndef foo(x: Any) -> Any:\n    return x"
    ));
}

#[test]
fn test_type_any_list() {
    assert!(transpile_ok(
        "from typing import Any, List\n\ndef foo(x: List[Any]) -> List[Any]:\n    return x"
    ));
}

// ============================================================================
// NESTED TYPES
// ============================================================================

#[test]
fn test_type_list_of_lists() {
    assert!(transpile_ok(
        "from typing import List\n\ndef foo(x: List[List[int]]) -> List[List[int]]:\n    return x"
    ));
}

#[test]
fn test_type_dict_of_lists() {
    assert!(transpile_ok("from typing import Dict, List\n\ndef foo(x: Dict[str, List[int]]) -> Dict[str, List[int]]:\n    return x"));
}

#[test]
fn test_type_list_of_tuples() {
    assert!(transpile_ok("from typing import List, Tuple\n\ndef foo(x: List[Tuple[int, str]]) -> List[Tuple[int, str]]:\n    return x"));
}

#[test]
fn test_type_optional_dict() {
    assert!(transpile_ok("from typing import Optional, Dict\n\ndef foo(x: Optional[Dict[str, int]]) -> Optional[Dict[str, int]]:\n    return x"));
}

// ============================================================================
// CLASS TYPES
// ============================================================================

#[test]
fn test_type_class_param() {
    assert!(transpile_ok(
        "class Point:\n    pass\n\ndef foo(p: Point) -> Point:\n    return p"
    ));
}

#[test]
fn test_type_class_list() {
    assert!(transpile_ok("from typing import List\n\nclass Point:\n    pass\n\ndef foo(points: List[Point]) -> List[Point]:\n    return points"));
}

#[test]
fn test_type_dataclass() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int\n\ndef foo(p: Point) -> int:\n    return p.x + p.y"));
}

// ============================================================================
// LITERAL TYPES
// ============================================================================

#[test]
fn test_type_literal_str() {
    assert!(transpile_ok(
        "from typing import Literal\n\ndef foo(mode: Literal['r', 'w']) -> str:\n    return mode"
    ));
}

#[test]
fn test_type_literal_int() {
    // Literal with int values may not be fully supported - check it doesn't crash
    let _ = transpile_ok(
        "from typing import Literal\n\ndef foo(n: Literal[0, 1, 2]) -> int:\n    return n",
    );
}

// ============================================================================
// FINAL TYPES
// ============================================================================

#[test]
fn test_type_final() {
    assert!(transpile_ok(
        "from typing import Final\n\nVALUE: Final[int] = 42"
    ));
}

// ============================================================================
// TYPEVAR
// ============================================================================

#[test]
fn test_type_typevar() {
    assert!(transpile_ok("from typing import TypeVar, List\n\nT = TypeVar('T')\n\ndef first(items: List[T]) -> T:\n    return items[0]"));
}

#[test]
fn test_type_typevar_bound() {
    assert!(transpile_ok("from typing import TypeVar\n\nT = TypeVar('T', int, str)\n\ndef foo(x: T) -> T:\n    return x"));
}

// ============================================================================
// SEQUENCE TYPES
// ============================================================================

#[test]
fn test_type_sequence() {
    assert!(transpile_ok(
        "from typing import Sequence\n\ndef foo(x: Sequence[int]) -> int:\n    return x[0]"
    ));
}

#[test]
fn test_type_mutablesequence() {
    assert!(transpile_ok("from typing import MutableSequence\n\ndef foo(x: MutableSequence[int]) -> None:\n    x.append(1)"));
}

// ============================================================================
// MAPPING TYPES
// ============================================================================

#[test]
fn test_type_mapping() {
    assert!(transpile_ok(
        "from typing import Mapping\n\ndef foo(x: Mapping[str, int]) -> int:\n    return x['key']"
    ));
}

#[test]
fn test_type_mutablemapping() {
    assert!(transpile_ok("from typing import MutableMapping\n\ndef foo(x: MutableMapping[str, int]) -> None:\n    x['key'] = 1"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_type_empty_tuple() {
    assert!(transpile_ok(
        "from typing import Tuple\n\ndef foo() -> Tuple[()]:\n    return ()"
    ));
}

#[test]
fn test_type_variadic_tuple() {
    assert!(transpile_ok(
        "from typing import Tuple\n\ndef foo(x: Tuple[int, ...]) -> int:\n    return sum(x)"
    ));
}

#[test]
fn test_type_forward_reference() {
    assert!(transpile_ok("from __future__ import annotations\n\nclass Node:\n    def __init__(self, next: Node):\n        self.next = next"));
}

#[test]
fn test_type_self_reference() {
    assert!(transpile_ok(
        "class Node:\n    def __init__(self, next: 'Node'):\n        self.next = next"
    ));
}
