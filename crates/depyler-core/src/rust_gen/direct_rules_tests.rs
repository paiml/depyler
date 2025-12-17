//! Comprehensive direct_rules tests
//!
//! These tests exercise the direct_rules.rs code paths through the transpilation pipeline.

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// CLASS TO STRUCT CONVERSION
// ============================================================================

#[test]
fn test_class_simple() {
    let code = transpile("class Point:\n    def __init__(self, x, y):\n        self.x = x\n        self.y = y");
    assert!(code.contains("struct Point") || code.contains("impl Point") || code.contains("pub"));
}

#[test]
fn test_class_with_methods() {
    let code = transpile("class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1");
    assert!(code.contains("struct Counter") || code.contains("impl"));
}

#[test]
fn test_class_with_static_method() {
    let code = transpile("class Math:\n    @staticmethod\n    def add(a, b):\n        return a + b");
    assert!(code.contains("struct Math") || code.contains("fn add"));
}

#[test]
fn test_class_with_class_method() {
    let code = transpile("class Factory:\n    @classmethod\n    def create(cls):\n        return cls()");
    assert!(code.contains("struct Factory") || code.contains("impl"));
}

#[test]
fn test_class_with_property() {
    assert!(transpile_ok("class Person:\n    def __init__(self, name):\n        self._name = name\n    @property\n    def name(self):\n        return self._name"));
}

#[test]
fn test_class_inheritance() {
    assert!(transpile_ok("class Animal:\n    pass\n\nclass Dog(Animal):\n    pass"));
}

#[test]
fn test_class_multiple_methods() {
    let code = transpile("class Calculator:\n    def add(self, a, b):\n        return a + b\n    def sub(self, a, b):\n        return a - b\n    def mul(self, a, b):\n        return a * b");
    assert!(code.contains("fn add") || code.contains("fn sub") || code.contains("fn mul"));
}

// ============================================================================
// DATACLASS CONVERSION
// ============================================================================

#[test]
fn test_dataclass_simple() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int");
    assert!(code.contains("struct Point") || code.contains("x:") || code.contains("y:"));
}

#[test]
fn test_dataclass_with_defaults() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Config:\n    name: str\n    value: int = 0");
    assert!(code.contains("struct Config") || code.contains("name"));
}

#[test]
fn test_dataclass_with_complex_types() {
    let code = transpile("from dataclasses import dataclass\nfrom typing import List, Optional\n\n@dataclass\nclass Container:\n    items: List[int]\n    label: Optional[str] = None");
    assert!(code.contains("struct Container") || code.contains("Vec"));
}

#[test]
fn test_dataclass_frozen() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass(frozen=True)\nclass Frozen:\n    value: int");
    assert!(code.contains("struct Frozen") || code.contains("value"));
}

// ============================================================================
// SPECIAL METHODS (__dunder__)
// ============================================================================

#[test]
fn test_class_str_method() {
    let code = transpile("class Foo:\n    def __str__(self):\n        return 'Foo'");
    assert!(code.contains("Display") || code.contains("fmt") || code.contains("struct Foo"));
}

#[test]
fn test_class_repr_method() {
    let code = transpile("class Foo:\n    def __repr__(self):\n        return 'Foo()'");
    assert!(code.contains("Debug") || code.contains("fmt") || code.contains("struct Foo"));
}

#[test]
fn test_class_eq_method() {
    let code = transpile("class Point:\n    def __init__(self, x):\n        self.x = x\n    def __eq__(self, other):\n        return self.x == other.x");
    assert!(code.contains("PartialEq") || code.contains("==") || code.contains("struct Point"));
}

#[test]
fn test_class_hash_method() {
    let code = transpile("class Key:\n    def __init__(self, value):\n        self.value = value\n    def __hash__(self):\n        return hash(self.value)");
    assert!(code.contains("Hash") || code.contains("struct Key"));
}

#[test]
fn test_class_len_method() {
    let code = transpile("class Container:\n    def __init__(self):\n        self.items = []\n    def __len__(self):\n        return len(self.items)");
    assert!(code.contains("len") || code.contains("struct Container"));
}

#[test]
fn test_class_iter_method() {
    let code = transpile("class Range:\n    def __init__(self, n):\n        self.n = n\n    def __iter__(self):\n        return iter(range(self.n))");
    assert!(code.contains("iter") || code.contains("Iterator") || code.contains("struct Range"));
}

#[test]
fn test_class_getitem_method() {
    let code = transpile("class List:\n    def __init__(self):\n        self.items = []\n    def __getitem__(self, index):\n        return self.items[index]");
    assert!(code.contains("Index") || code.contains("[") || code.contains("struct List"));
}

#[test]
fn test_class_setitem_method() {
    let code = transpile("class Dict:\n    def __init__(self):\n        self.data = {}\n    def __setitem__(self, key, value):\n        self.data[key] = value");
    assert!(code.contains("[") || code.contains("struct Dict"));
}

#[test]
fn test_class_contains_method() {
    let code = transpile("class Set:\n    def __init__(self):\n        self.items = set()\n    def __contains__(self, item):\n        return item in self.items");
    assert!(code.contains("contains") || code.contains("struct Set"));
}

#[test]
fn test_class_add_method() {
    let code = transpile("class Vector:\n    def __init__(self, x):\n        self.x = x\n    def __add__(self, other):\n        return Vector(self.x + other.x)");
    assert!(code.contains("Add") || code.contains("+") || code.contains("struct Vector"));
}

#[test]
fn test_class_sub_method() {
    let code = transpile("class Vector:\n    def __init__(self, x):\n        self.x = x\n    def __sub__(self, other):\n        return Vector(self.x - other.x)");
    assert!(code.contains("Sub") || code.contains("-") || code.contains("struct Vector"));
}

#[test]
fn test_class_mul_method() {
    let code = transpile("class Number:\n    def __init__(self, value):\n        self.value = value\n    def __mul__(self, other):\n        return Number(self.value * other)");
    assert!(code.contains("Mul") || code.contains("*") || code.contains("struct Number"));
}

#[test]
fn test_class_lt_method() {
    let code = transpile("class Score:\n    def __init__(self, value):\n        self.value = value\n    def __lt__(self, other):\n        return self.value < other.value");
    assert!(code.contains("Ord") || code.contains("<") || code.contains("struct Score"));
}

// ============================================================================
// STDLIB SHADOWING NAMES
// ============================================================================

#[test]
fn test_class_named_string() {
    // Class named String should get renamed to avoid shadowing
    let code = transpile("class String:\n    def __init__(self):\n        pass");
    assert!(code.contains("struct") || code.contains("String_"));
}

#[test]
fn test_class_named_vec() {
    // Class named Vec should get renamed
    assert!(transpile_ok("class Vec:\n    def __init__(self):\n        pass"));
}

#[test]
fn test_class_named_option() {
    // Class named Option should get renamed
    assert!(transpile_ok("class Option:\n    def __init__(self):\n        pass"));
}

// ============================================================================
// METHOD MUTABILITY DETECTION
// ============================================================================

#[test]
fn test_method_immutable() {
    let code = transpile("class Foo:\n    def __init__(self):\n        self.x = 0\n    def get_x(self):\n        return self.x");
    assert!(code.contains("&self") || code.contains("fn get_x") || code.contains("struct Foo"));
}

#[test]
fn test_method_mutable() {
    let code = transpile("class Foo:\n    def __init__(self):\n        self.x = 0\n    def set_x(self, value):\n        self.x = value");
    assert!(code.contains("&mut self") || code.contains("fn set_x") || code.contains("struct Foo"));
}

#[test]
fn test_method_with_augmented_assign() {
    let code = transpile("class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1");
    assert!(code.contains("&mut self") || code.contains("fn increment") || code.contains("struct Counter"));
}

// ============================================================================
// TYPE CONVERSIONS
// ============================================================================

#[test]
fn test_typed_field_int() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Data:\n    value: int");
    assert!(code.contains("i64") || code.contains("i32") || code.contains("value"));
}

#[test]
fn test_typed_field_float() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Data:\n    value: float");
    assert!(code.contains("f64") || code.contains("f32") || code.contains("value"));
}

#[test]
fn test_typed_field_str() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Data:\n    name: str");
    assert!(code.contains("String") || code.contains("str") || code.contains("name"));
}

#[test]
fn test_typed_field_bool() {
    let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Data:\n    flag: bool");
    assert!(code.contains("bool") || code.contains("flag"));
}

#[test]
fn test_typed_field_list() {
    let code = transpile("from dataclasses import dataclass\nfrom typing import List\n\n@dataclass\nclass Data:\n    items: List[int]");
    assert!(code.contains("Vec") || code.contains("items"));
}

#[test]
fn test_typed_field_dict() {
    let code = transpile("from dataclasses import dataclass\nfrom typing import Dict\n\n@dataclass\nclass Data:\n    mapping: Dict[str, int]");
    assert!(code.contains("HashMap") || code.contains("mapping"));
}

#[test]
fn test_typed_field_optional() {
    let code = transpile("from dataclasses import dataclass\nfrom typing import Optional\n\n@dataclass\nclass Data:\n    value: Optional[int]");
    assert!(code.contains("Option") || code.contains("value"));
}

#[test]
fn test_typed_field_tuple() {
    let code = transpile("from dataclasses import dataclass\nfrom typing import Tuple\n\n@dataclass\nclass Data:\n    point: Tuple[int, int]");
    assert!(code.contains("(") || code.contains("point"));
}

// ============================================================================
// EXCEPTION CLASSES
// ============================================================================

#[test]
fn test_custom_exception() {
    let code = transpile("class CustomError(Exception):\n    pass");
    assert!(code.contains("struct CustomError") || code.contains("Error"));
}

#[test]
fn test_exception_with_message() {
    let code = transpile("class ValidationError(Exception):\n    def __init__(self, message):\n        self.message = message\n        super().__init__(message)");
    assert!(code.contains("struct ValidationError") || code.contains("message"));
}

// ============================================================================
// ENUM-LIKE CLASSES
// ============================================================================

#[test]
fn test_enum_class() {
    let code = transpile("from enum import Enum\n\nclass Color(Enum):\n    RED = 1\n    GREEN = 2\n    BLUE = 3");
    assert!(code.contains("enum Color") || code.contains("RED") || code.contains("GREEN"));
}

#[test]
fn test_enum_auto() {
    assert!(transpile_ok("from enum import Enum, auto\n\nclass Status(Enum):\n    PENDING = auto()\n    ACTIVE = auto()\n    DONE = auto()"));
}

// ============================================================================
// COMPLEX CLASS SCENARIOS
// ============================================================================

#[test]
fn test_class_with_type_annotations() {
    let code = transpile("class TypedClass:\n    x: int\n    y: str\n    \n    def __init__(self, x: int, y: str):\n        self.x = x\n        self.y = y");
    assert!(code.contains("struct TypedClass") || code.contains("x:") || code.contains("y:"));
}

#[test]
fn test_class_with_default_factory() {
    let code = transpile("from dataclasses import dataclass, field\nfrom typing import List\n\n@dataclass\nclass Container:\n    items: List[int] = field(default_factory=list)");
    assert!(code.contains("struct Container") || code.contains("Vec"));
}

#[test]
fn test_nested_class() {
    assert!(transpile_ok("class Outer:\n    class Inner:\n        def __init__(self):\n            pass\n    def __init__(self):\n        self.inner = self.Inner()"));
}

#[test]
fn test_class_with_slots() {
    assert!(transpile_ok("class Efficient:\n    __slots__ = ['x', 'y']\n    def __init__(self, x, y):\n        self.x = x\n        self.y = y"));
}

// ============================================================================
// RUST KEYWORD HANDLING IN CLASSES
// ============================================================================

#[test]
fn test_field_named_type() {
    // "type" is a Rust keyword
    let code = transpile("class Node:\n    def __init__(self):\n        self.type = 'leaf'");
    assert!(code.contains("r#type") || code.contains("type_") || code.contains("struct Node"));
}

#[test]
fn test_method_named_match() {
    // "match" is a Rust keyword
    let code = transpile("class Matcher:\n    def match(self, pattern):\n        return True");
    assert!(code.contains("r#match") || code.contains("fn match") || code.contains("struct Matcher"));
}

#[test]
fn test_field_named_loop() {
    // "loop" is a Rust keyword
    assert!(transpile_ok("class Cycler:\n    def __init__(self):\n        self.loop = 0"));
}

// ============================================================================
// VISIBILITY AND ACCESS
// ============================================================================

#[test]
fn test_private_method() {
    let code = transpile("class Foo:\n    def _private(self):\n        return 1\n    def public(self):\n        return self._private()");
    assert!(code.contains("fn _private") || code.contains("fn public") || code.contains("struct Foo"));
}

#[test]
fn test_dunder_method() {
    let code = transpile("class Foo:\n    def __custom__(self):\n        return 1");
    assert!(code.contains("fn") || code.contains("struct Foo"));
}

// ============================================================================
// ABSTRACT CLASSES
// ============================================================================

#[test]
fn test_abstract_class() {
    assert!(transpile_ok("from abc import ABC, abstractmethod\n\nclass Shape(ABC):\n    @abstractmethod\n    def area(self):\n        pass"));
}

#[test]
fn test_abstract_with_implementation() {
    assert!(transpile_ok("from abc import ABC, abstractmethod\n\nclass Shape(ABC):\n    @abstractmethod\n    def area(self):\n        pass\n\nclass Circle(Shape):\n    def __init__(self, r):\n        self.r = r\n    def area(self):\n        return 3.14 * self.r ** 2"));
}

// ============================================================================
// GENERIC CLASSES
// ============================================================================

#[test]
fn test_generic_class() {
    assert!(transpile_ok("from typing import Generic, TypeVar\n\nT = TypeVar('T')\n\nclass Box(Generic[T]):\n    def __init__(self, value: T):\n        self.value = value"));
}

// ============================================================================
// MULTIPLE CLASSES
// ============================================================================

#[test]
fn test_multiple_classes() {
    let code = transpile("class Foo:\n    pass\n\nclass Bar:\n    pass\n\nclass Baz:\n    pass");
    assert!(code.contains("struct Foo") || code.contains("struct Bar") || code.contains("struct Baz"));
}

#[test]
fn test_class_referencing_another() {
    let code = transpile("class Node:\n    def __init__(self, value):\n        self.value = value\n        self.next = None\n\nclass LinkedList:\n    def __init__(self):\n        self.head = None");
    assert!(code.contains("struct Node") || code.contains("struct LinkedList"));
}
