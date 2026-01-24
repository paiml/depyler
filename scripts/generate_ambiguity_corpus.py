#!/usr/bin/env python3
"""
OPERATION AMBIGUITY - Civil War Corpus Generator
=================================================
Generates 2,000 Python files specifically designed to break depyler's
dictionary type inference, targeting the Type System Schism:
  - type_tokens.rs expects HashMap<String, V>
  - type_mapper.rs infers HashMap<DepylerValue, V>

Target: E0308 (Type Mismatch) and E0599 (Method Not Found) errors.

Patterns:
  1. Literal Trap - {"a": 1} vs {1: "a"} vs {} (empty dicts)
  2. Flow Gap - Dict passing through functions, returns, typed variables
  3. Method Clash - Classes with to_dict()/from_dict() returning mixed types
  4. Module Boundary - Cross-module dict definitions and access

Usage:
    python scripts/generate_ambiguity_corpus.py [--output DIR] [--count N]
"""

import argparse
import os
import random
import string
import hashlib
from pathlib import Path
from typing import List, Tuple, Callable
from dataclasses import dataclass
from itertools import product


# ==============================================================================
# CONFIGURATION
# ==============================================================================

DEFAULT_OUTPUT_DIR = "training_corpus/ambiguity_v1"
DEFAULT_FILE_COUNT = 2000

# Seed for reproducibility
random.seed(0xDE9713A)  # "DEPYLER" in spirit

# Pattern distribution (must sum to 1.0)
PATTERN_WEIGHTS = {
    "literal_trap": 0.30,      # 600 files
    "flow_gap": 0.30,          # 600 files
    "method_clash": 0.25,      # 500 files
    "module_boundary": 0.15,   # 300 files
}


# ==============================================================================
# UTILITY GENERATORS
# ==============================================================================

def random_identifier(prefix: str = "var", length: int = 4) -> str:
    """Generate a random valid Python identifier."""
    suffix = ''.join(random.choices(string.ascii_lowercase, k=length))
    return f"{prefix}_{suffix}"


def random_string_literal() -> str:
    """Generate a random string literal."""
    words = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta"]
    return f'"{random.choice(words)}"'


def random_int_literal() -> str:
    """Generate a random integer literal."""
    return str(random.randint(-1000, 1000))


def random_float_literal() -> str:
    """Generate a random float literal."""
    return f"{random.uniform(-100.0, 100.0):.4f}"


def random_bool_literal() -> str:
    """Generate a random boolean literal."""
    return random.choice(["True", "False"])


def random_none_literal() -> str:
    """Generate None literal."""
    return "None"


def random_primitive_literal() -> Tuple[str, str]:
    """Return (literal, type_name) for a random primitive."""
    choices = [
        (random_string_literal, "str"),
        (random_int_literal, "int"),
        (random_float_literal, "float"),
        (random_bool_literal, "bool"),
    ]
    gen, type_name = random.choice(choices)
    return gen(), type_name


# ==============================================================================
# PATTERN 1: THE LITERAL TRAP
# ==============================================================================

class LiteralTrapGenerator:
    """
    Generates Python code with ambiguous dict literal types.

    Attacks:
    - String keys vs int keys vs mixed keys
    - Empty dicts with no type context
    - Reassignment with different key types
    - Dict comprehensions with varying key types
    """

    @staticmethod
    def empty_dict_no_context(idx: int) -> str:
        """Empty dict assigned without any type annotation."""
        var = random_identifier("empty_dict")
        return f'''# LITERAL_TRAP_001: Empty dict without type context
# Expected failure: Cannot infer key/value types from empty literal

def process_{idx}() -> dict:
    {var} = {{}}
    return {var}

def use_empty_{idx}():
    d = process_{idx}()
    d["key"] = 42  # Assumes string key
    d[1] = "value"  # Assumes int key - CONFLICT
    return d
'''

    @staticmethod
    def empty_dict_delayed_use(idx: int) -> str:
        """Empty dict with delayed heterogeneous insertions."""
        var = random_identifier("delayed")
        return f'''# LITERAL_TRAP_002: Empty dict with delayed heterogeneous use

def build_delayed_{idx}():
    {var} = {{}}

    # Phase 1: String keys
    for i in range(3):
        {var}[f"key_{{i}}"] = i

    # Phase 2: Int keys - type conflict
    for i in range(3):
        {var}[i] = f"value_{{i}}"

    return {var}

result_{idx} = build_delayed_{idx}()
'''

    @staticmethod
    def string_vs_int_key(idx: int) -> str:
        """Direct conflict: string key dict vs int key dict."""
        str_var = random_identifier("str_dict")
        int_var = random_identifier("int_dict")
        return f'''# LITERAL_TRAP_003: String keys vs Int keys

{str_var}_{idx} = {{"a": 1, "b": 2, "c": 3}}
{int_var}_{idx} = {{1: "a", 2: "b", 3: "c"}}

def get_by_key_{idx}(use_string: bool):
    if use_string:
        return {str_var}_{idx}["a"]
    else:
        return {int_var}_{idx}[1]

# Merge operation - key type conflict
def merge_{idx}():
    merged = {{**{str_var}_{idx}, **{int_var}_{idx}}}  # Mixed keys
    return merged
'''

    @staticmethod
    def tuple_key_dict(idx: int) -> str:
        """Dict with tuple keys (non-hashable in some contexts)."""
        var = random_identifier("tuple_key")
        return f'''# LITERAL_TRAP_004: Tuple keys (complex hashable type)

{var}_{idx} = {{
    (0, 0): "origin",
    (1, 0): "right",
    (0, 1): "up",
    (-1, -1): "diagonal",
}}

def get_coord_{idx}(x: int, y: int) -> str:
    key = (x, y)
    return {var}_{idx}.get(key, "unknown")

def all_keys_{idx}():
    return list({var}_{idx}.keys())  # Returns List[Tuple[int, int]]
'''

    @staticmethod
    def bool_key_dict(idx: int) -> str:
        """Dict with boolean keys (rare but valid)."""
        var = random_identifier("bool_key")
        return f'''# LITERAL_TRAP_005: Boolean keys

{var}_{idx} = {{True: "yes", False: "no"}}

def describe_{idx}(flag: bool) -> str:
    return {var}_{idx}[flag]

def invert_{idx}():
    return {{not k: v for k, v in {var}_{idx}.items()}}
'''

    @staticmethod
    def none_key_dict(idx: int) -> str:
        """Dict with None as a key (edge case)."""
        var = random_identifier("none_key")
        return f'''# LITERAL_TRAP_006: None as key

{var}_{idx} = {{None: "nothing", "something": 42}}

def lookup_{idx}(key):
    return {var}_{idx}.get(key, "default")

def check_none_{idx}():
    return None in {var}_{idx}  # Key membership check
'''

    @staticmethod
    def float_key_dict(idx: int) -> str:
        """Dict with float keys (hash/eq complexity)."""
        var = random_identifier("float_key")
        return f'''# LITERAL_TRAP_007: Float keys (NaN handling issues)

import math

{var}_{idx} = {{
    0.0: "zero",
    1.5: "one_half",
    -3.14159: "neg_pi",
    float("inf"): "infinity",
}}

def get_float_{idx}(f: float) -> str:
    return {var}_{idx}.get(f, "unknown")

def handle_nan_{idx}():
    nan_dict = {{float("nan"): "not_a_number"}}
    # NaN != NaN, so lookup will fail
    return float("nan") in nan_dict
'''

    @staticmethod
    def mixed_primitive_keys(idx: int) -> str:
        """Dict with multiple primitive key types in same dict."""
        var = random_identifier("mixed_key")
        return f'''# LITERAL_TRAP_008: Mixed primitive key types

{var}_{idx} = {{
    "str_key": 1,
    42: "int_val",
    3.14: "float_val",
    True: "bool_val",
    None: "none_val",
}}

def access_all_{idx}():
    results = []
    results.append({var}_{idx}["str_key"])
    results.append({var}_{idx}[42])
    results.append({var}_{idx}[3.14])
    results.append({var}_{idx}[True])
    results.append({var}_{idx}[None])
    return results
'''

    @staticmethod
    def dict_comprehension_key_expr(idx: int) -> str:
        """Dict comprehension with computed keys."""
        var = random_identifier("comp_dict")
        return f'''# LITERAL_TRAP_009: Dict comprehension with expression keys

words_{idx} = ["apple", "banana", "cherry"]

# String keys from expression
{var}_str_{idx} = {{word.upper(): len(word) for word in words_{idx}}}

# Int keys from expression
{var}_int_{idx} = {{i * 2: chr(65 + i) for i in range(5)}}

# Mixed: key type depends on condition
{var}_mixed_{idx} = {{
    (i if i % 2 == 0 else str(i)): i ** 2
    for i in range(10)
}}
'''

    @staticmethod
    def conditional_key_type(idx: int) -> str:
        """Dict where key type depends on runtime condition."""
        var = random_identifier("cond_key")
        return f'''# LITERAL_TRAP_010: Conditional key type

def make_dict_{idx}(use_strings: bool) -> dict:
    {var} = {{}}
    for i in range(5):
        if use_strings:
            key = f"item_{{i}}"
        else:
            key = i
        {var}[key] = i * 10
    return {var}

str_dict_{idx} = make_dict_{idx}(True)
int_dict_{idx} = make_dict_{idx}(False)

# Union would be Dict[Union[str, int], int]
'''

    @staticmethod
    def nested_empty_dicts(idx: int) -> str:
        """Nested structure with empty dicts."""
        return f'''# LITERAL_TRAP_011: Nested empty dicts

def create_nested_{idx}():
    outer = {{}}
    outer["level1"] = {{}}
    outer["level1"]["level2"] = {{}}
    outer["level1"]["level2"]["level3"] = {{}}

    # Now populate deepest level
    outer["level1"]["level2"]["level3"]["value"] = 42
    outer["level1"]["level2"]["level3"][0] = "zero"  # Int key!

    return outer

nested_{idx} = create_nested_{idx}()
'''

    @staticmethod
    def reassignment_type_change(idx: int) -> str:
        """Variable reassigned to dict with different key type."""
        var = random_identifier("reassign")
        return f'''# LITERAL_TRAP_012: Reassignment changes key type

{var}_{idx} = {{"a": 1}}  # Dict[str, int]
print({var}_{idx}["a"])

{var}_{idx} = {{1: "a"}}  # Now Dict[int, str] - CONFLICT
print({var}_{idx}[1])

{var}_{idx} = {{}}  # Now Dict[?, ?]
{var}_{idx}[(1, 2)] = "tuple_key"  # Dict[Tuple[int, int], str]
'''

    @classmethod
    def all_generators(cls) -> List[Callable[[int], str]]:
        return [
            cls.empty_dict_no_context,
            cls.empty_dict_delayed_use,
            cls.string_vs_int_key,
            cls.tuple_key_dict,
            cls.bool_key_dict,
            cls.none_key_dict,
            cls.float_key_dict,
            cls.mixed_primitive_keys,
            cls.dict_comprehension_key_expr,
            cls.conditional_key_type,
            cls.nested_empty_dicts,
            cls.reassignment_type_change,
        ]


# ==============================================================================
# PATTERN 2: THE FLOW GAP
# ==============================================================================

class FlowGapGenerator:
    """
    Generates Python code testing dict flow through functions.

    Attacks:
    - Passing untyped dicts to typed parameters
    - Returning dicts from functions with generic return types
    - Dict aliasing and mutation
    - Closure capture of dicts
    """

    @staticmethod
    def untyped_to_typed_param(idx: int) -> str:
        """Pass untyped dict to function expecting typed dict."""
        return f'''# FLOW_GAP_001: Untyped dict to typed parameter
from typing import Dict

def process_typed_{idx}(data: Dict[str, int]) -> int:
    return sum(data.values())

# Caller creates untyped dict
raw_{idx} = {{"a": 1, "b": 2}}  # Inferred as dict
result_{idx} = process_typed_{idx}(raw_{idx})

# Even worse: heterogeneous dict
bad_{idx} = {{"a": 1, "b": "two"}}  # Dict[str, Union[int, str]]
# result2_{idx} = process_typed_{idx}(bad_{idx})  # Should fail
'''

    @staticmethod
    def generic_return_type(idx: int) -> str:
        """Function returns dict but has generic return annotation."""
        return f'''# FLOW_GAP_002: Generic return type

def get_data_{idx}() -> dict:
    return {{"name": "test", "count": 42, "active": True}}

def use_data_{idx}():
    d = get_data_{idx}()  # d: dict (not Dict[str, ???])

    # All these should work but type is unknown
    name: str = d["name"]
    count: int = d["count"]
    active: bool = d["active"]

    return name, count, active
'''

    @staticmethod
    def dict_mutation_through_ref(idx: int) -> str:
        """Dict mutated through reference/alias."""
        return f'''# FLOW_GAP_003: Dict mutation through reference

def mutate_dict_{idx}(d: dict) -> None:
    d["new_str_key"] = 100
    d[999] = "int_key_added"  # Key type changes!

original_{idx} = {{"start": 0}}
mutate_dict_{idx}(original_{idx})
# original is now Dict[Union[str, int], Union[int, str]]
'''

    @staticmethod
    def closure_capture_dict(idx: int) -> str:
        """Dict captured by closure."""
        return f'''# FLOW_GAP_004: Dict captured by closure

def make_counter_{idx}():
    counts = {{}}  # Captured by inner function

    def increment(key: str) -> int:
        if key not in counts:
            counts[key] = 0
        counts[key] += 1
        return counts[key]

    def get_all() -> dict:
        return counts.copy()

    return increment, get_all

inc_{idx}, get_{idx} = make_counter_{idx}()
inc_{idx}("a")
inc_{idx}("b")
inc_{idx}("a")
all_counts_{idx} = get_{idx}()
'''

    @staticmethod
    def dict_as_default_arg(idx: int) -> str:
        """Dict as mutable default argument (Python gotcha)."""
        return f'''# FLOW_GAP_005: Dict as default argument

def add_item_{idx}(key: str, value: int, container: dict = {{}}) -> dict:
    container[key] = value
    return container

# Each call should get fresh dict but doesn't
r1_{idx} = add_item_{idx}("a", 1)
r2_{idx} = add_item_{idx}("b", 2)
# r1 and r2 might be same object!

# With None default pattern
def add_item_safe_{idx}(key: str, value: int, container: dict = None) -> dict:
    if container is None:
        container = {{}}
    container[key] = value
    return container
'''

    @staticmethod
    def dict_in_list(idx: int) -> str:
        """List of dicts with varying types."""
        return f'''# FLOW_GAP_006: List of dicts with varying types
from typing import List

records_{idx}: List[dict] = [
    {{"name": "Alice", "age": 30}},
    {{"id": 1, "active": True}},
    {{"x": 1.5, "y": 2.5, "z": 3.5}},
]

def process_records_{idx}(records: List[dict]) -> List[str]:
    results = []
    for r in records:
        # Each r has different structure
        results.append(str(r))
    return results
'''

    @staticmethod
    def dict_union_return(idx: int) -> str:
        """Function that returns different dict types based on condition."""
        return f'''# FLOW_GAP_007: Union return type for dicts
from typing import Dict, Union

def fetch_{idx}(as_strings: bool) -> Union[Dict[str, int], Dict[int, str]]:
    if as_strings:
        return {{"one": 1, "two": 2}}
    else:
        return {{1: "one", 2: "two"}}

result_str_{idx} = fetch_{idx}(True)
result_int_{idx} = fetch_{idx}(False)
'''

    @staticmethod
    def dict_kwarg_splat(idx: int) -> str:
        """Dict used with **kwargs splatting."""
        return f'''# FLOW_GAP_008: Dict with **kwargs

def target_func_{idx}(a: int, b: str, c: bool = False) -> str:
    return f"{{a}}-{{b}}-{{c}}"

kwargs_{idx} = {{"a": 42, "b": "hello", "c": True}}
result_{idx} = target_func_{idx}(**kwargs_{idx})

# With extra keys (should fail at runtime)
extra_kwargs_{idx} = {{"a": 1, "b": "x", "c": False, "d": "extra"}}
# target_func_{idx}(**extra_kwargs_{idx})  # TypeError at runtime
'''

    @staticmethod
    def nested_function_dict_flow(idx: int) -> str:
        """Dict flows through multiple function calls."""
        return f'''# FLOW_GAP_009: Dict through call chain

def step1_{idx}() -> dict:
    return {{"stage": 1}}

def step2_{idx}(d: dict) -> dict:
    d["stage"] = 2
    d["extra"] = "added"
    return d

def step3_{idx}(d: dict) -> dict:
    d["stage"] = 3
    d[42] = "int_key"  # Changes key type!
    return d

def pipeline_{idx}():
    data = step1_{idx}()
    data = step2_{idx}(data)
    data = step3_{idx}(data)
    return data

final_{idx} = pipeline_{idx}()
'''

    @staticmethod
    def generator_yields_dict(idx: int) -> str:
        """Generator that yields different dict types."""
        return f'''# FLOW_GAP_010: Generator yielding dicts
from typing import Iterator

def dict_generator_{idx}() -> Iterator[dict]:
    yield {{"type": "str_key", "value": 1}}
    yield {{0: "int_key", 1: "more"}}
    yield {{(0, 0): "tuple_key"}}
    yield {{True: "bool_key", False: "other"}}

def consume_generator_{idx}():
    all_dicts = list(dict_generator_{idx}())
    return all_dicts
'''

    @staticmethod
    def recursive_dict_build(idx: int) -> str:
        """Recursively build dict structure."""
        return f'''# FLOW_GAP_011: Recursive dict construction

def build_tree_{idx}(depth: int) -> dict:
    if depth == 0:
        return {{"leaf": True, "value": depth}}
    else:
        return {{
            "leaf": False,
            "left": build_tree_{idx}(depth - 1),
            "right": build_tree_{idx}(depth - 1),
            depth: f"level_{{depth}}",  # Int key mixed with str keys!
        }}

tree_{idx} = build_tree_{idx}(3)
'''

    @staticmethod
    def lambda_dict_transform(idx: int) -> str:
        """Lambda functions transforming dicts."""
        return f'''# FLOW_GAP_012: Lambda dict transforms
from typing import Callable, Dict

data_{idx} = {{"a": 1, "b": 2, "c": 3}}

# Transform with lambda
double_{idx}: Callable[[dict], dict] = lambda d: {{k: v * 2 for k, v in d.items()}}
inverted_{idx}: Callable[[dict], dict] = lambda d: {{v: k for k, v in d.items()}}

doubled_{idx} = double_{idx}(data_{idx})  # Dict[str, int]
inverted_result_{idx} = inverted_{idx}(data_{idx})  # Dict[int, str] - key type changed!
'''

    @classmethod
    def all_generators(cls) -> List[Callable[[int], str]]:
        return [
            cls.untyped_to_typed_param,
            cls.generic_return_type,
            cls.dict_mutation_through_ref,
            cls.closure_capture_dict,
            cls.dict_as_default_arg,
            cls.dict_in_list,
            cls.dict_union_return,
            cls.dict_kwarg_splat,
            cls.nested_function_dict_flow,
            cls.generator_yields_dict,
            cls.recursive_dict_build,
            cls.lambda_dict_transform,
        ]


# ==============================================================================
# PATTERN 3: THE METHOD CLASH
# ==============================================================================

class MethodClashGenerator:
    """
    Generates classes with to_dict/from_dict that return ambiguous types.

    Attacks:
    - to_dict() returning heterogeneous dicts
    - from_dict() consuming untyped dicts
    - Inheritance hierarchies with conflicting dict methods
    - Protocol/ABC implementations
    """

    @staticmethod
    def basic_to_dict_heterogeneous(idx: int) -> str:
        """Basic class with heterogeneous to_dict return."""
        return f'''# METHOD_CLASH_001: Heterogeneous to_dict return

class Entity_{idx}:
    def __init__(self, id: int, name: str, active: bool):
        self.id = id
        self.name = name
        self.active = active
        self.metadata = {{}}

    def to_dict(self) -> dict:
        return {{
            "id": self.id,          # int value
            "name": self.name,      # str value
            "active": self.active,  # bool value
            "metadata": self.metadata,  # dict value
            42: "secret",           # int KEY - breaks str key assumption
        }}

e_{idx} = Entity_{idx}(1, "test", True)
d_{idx} = e_{idx}.to_dict()
'''

    @staticmethod
    def from_dict_untyped(idx: int) -> str:
        """from_dict consuming completely untyped dict."""
        return f'''# METHOD_CLASH_002: from_dict with untyped input

class Config_{idx}:
    def __init__(self):
        self.settings = {{}}

    @classmethod
    def from_dict(cls, data: dict) -> "Config_{idx}":
        instance = cls()
        # No type info about data's structure
        for key, value in data.items():
            instance.settings[key] = value
        return instance

    def get(self, key):
        return self.settings.get(key)

raw_data_{idx} = {{"host": "localhost", "port": 8080, "debug": True, 0: "zero"}}
config_{idx} = Config_{idx}.from_dict(raw_data_{idx})
'''

    @staticmethod
    def inheritance_dict_conflict(idx: int) -> str:
        """Inheritance with conflicting dict return types."""
        return f'''# METHOD_CLASH_003: Inheritance dict conflict
from typing import Dict

class BaseModel_{idx}:
    def to_dict(self) -> Dict[str, str]:
        return {{"type": "base"}}

class DerivedModel_{idx}(BaseModel_{idx}):
    def __init__(self, count: int):
        self.count = count

    def to_dict(self) -> dict:  # Wider return type
        base = super().to_dict()
        base["count"] = self.count  # Now has int value!
        base[self.count] = "indexed"  # Int key!
        return base

derived_{idx} = DerivedModel_{idx}(42)
d_{idx} = derived_{idx}.to_dict()
'''

    @staticmethod
    def protocol_dict_method(idx: int) -> str:
        """Protocol/ABC with dict method."""
        return f'''# METHOD_CLASH_004: Protocol with dict method
from typing import Protocol, Dict, Any

class Serializable_{idx}(Protocol):
    def to_dict(self) -> Dict[str, Any]: ...

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Serializable_{idx}": ...

class User_{idx}:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    def to_dict(self) -> Dict[str, Any]:
        return {{"name": self.name, "age": self.age}}

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "User_{idx}":
        return cls(data["name"], data["age"])

# But actual usage has int keys
class BadUser_{idx}:
    def to_dict(self) -> dict:
        return {{0: "zero", 1: "one"}}  # Violates protocol intent
'''

    @staticmethod
    def dataclass_asdict(idx: int) -> str:
        """Dataclass with asdict producing complex types."""
        return f'''# METHOD_CLASH_005: Dataclass asdict
from dataclasses import dataclass, asdict
from typing import List, Optional

@dataclass
class NestedData_{idx}:
    items: List[int]
    mapping: dict  # Untyped dict field
    parent: Optional["NestedData_{idx}"] = None

@dataclass
class Container_{idx}:
    nested: NestedData_{idx}
    extras: dict

def serialize_{idx}(c: Container_{idx}) -> dict:
    # asdict recursively converts, but dict fields are opaque
    return asdict(c)

n_{idx} = NestedData_{idx}([1, 2, 3], {{"a": 1, 0: "zero"}})
c_{idx} = Container_{idx}(n_{idx}, {{"key": "value"}})
result_{idx} = serialize_{idx}(c_{idx})
'''

    @staticmethod
    def mixin_dict_methods(idx: int) -> str:
        """Mixin classes providing dict methods."""
        return f'''# METHOD_CLASH_006: Mixin dict methods

class DictMixin_{idx}:
    def to_dict(self) -> dict:
        return {{k: v for k, v in self.__dict__.items() if not k.startswith("_")}}

class JsonMixin_{idx}:
    def to_json_dict(self) -> dict:
        d = {{}}
        for k, v in self.__dict__.items():
            if isinstance(v, (int, str, float, bool)):
                d[k] = v
            else:
                d[k] = str(v)
        return d

class MyClass_{idx}(DictMixin_{idx}, JsonMixin_{idx}):
    def __init__(self):
        self.name = "test"
        self.value = 42
        self.data = {{1: "one", 2: "two"}}  # Dict with int keys

obj_{idx} = MyClass_{idx}()
d1_{idx} = obj_{idx}.to_dict()
d2_{idx} = obj_{idx}.to_json_dict()
'''

    @staticmethod
    def dict_subclass(idx: int) -> str:
        """Custom dict subclass with weird behavior."""
        return f'''# METHOD_CLASH_007: Dict subclass

class SmartDict_{idx}(dict):
    def __setitem__(self, key, value):
        # Coerce keys to different types
        if isinstance(key, int):
            key = str(key)  # Convert int keys to str
        elif isinstance(key, str) and key.isdigit():
            key = int(key)  # Convert numeric strings to int
        super().__setitem__(key, value)

    def to_standard_dict(self) -> dict:
        return dict(self)

smart_{idx} = SmartDict_{idx}()
smart_{idx}["a"] = 1
smart_{idx}[42] = "was_int"
smart_{idx}["123"] = "was_str_digit"
standard_{idx} = smart_{idx}.to_standard_dict()
'''

    @staticmethod
    def property_returns_dict(idx: int) -> str:
        """Property that returns dict with complex type."""
        return f'''# METHOD_CLASH_008: Property returning dict

class DataHolder_{idx}:
    def __init__(self):
        self._cache = None
        self._raw = [("a", 1), ("b", 2), (0, "zero")]

    @property
    def as_dict(self) -> dict:
        if self._cache is None:
            self._cache = dict(self._raw)  # Mixed key types
        return self._cache

    @property
    def keys_as_strings(self) -> dict:
        return {{str(k): v for k, v in self._raw}}

holder_{idx} = DataHolder_{idx}()
d_{idx} = holder_{idx}.as_dict
s_{idx} = holder_{idx}.keys_as_strings
'''

    @staticmethod
    def class_with_dict_operators(idx: int) -> str:
        """Class implementing dict-like operators."""
        return f'''# METHOD_CLASH_009: Dict-like operators

class DictLike_{idx}:
    def __init__(self):
        self._data = {{}}

    def __getitem__(self, key):
        return self._data[key]

    def __setitem__(self, key, value):
        self._data[key] = value

    def __contains__(self, key):
        return key in self._data

    def __or__(self, other: dict) -> dict:
        # Merge operator
        return {{**self._data, **other}}

    def items(self):
        return self._data.items()

dl_{idx} = DictLike_{idx}()
dl_{idx}["str_key"] = 1
dl_{idx}[42] = "int_key"
merged_{idx} = dl_{idx} | {{"extra": "value"}}
'''

    @staticmethod
    def static_method_dict_factory(idx: int) -> str:
        """Static methods as dict factories."""
        return f'''# METHOD_CLASH_010: Static method dict factories

class Factories_{idx}:
    @staticmethod
    def empty() -> dict:
        return {{}}

    @staticmethod
    def with_strings() -> dict:
        return {{"a": 1, "b": 2}}

    @staticmethod
    def with_ints() -> dict:
        return {{1: "a", 2: "b"}}

    @staticmethod
    def mixed() -> dict:
        return {{"str": 1, 42: "int", (1, 2): "tuple"}}

    @staticmethod
    def from_pairs(pairs) -> dict:
        return dict(pairs)

e_{idx} = Factories_{idx}.empty()
s_{idx} = Factories_{idx}.with_strings()
i_{idx} = Factories_{idx}.with_ints()
m_{idx} = Factories_{idx}.mixed()
p_{idx} = Factories_{idx}.from_pairs([(0, "a"), (1, "b")])
'''

    @staticmethod
    def nested_class_dict(idx: int) -> str:
        """Nested classes with dict methods."""
        return f'''# METHOD_CLASH_011: Nested classes with dict methods

class Outer_{idx}:
    class Inner:
        def __init__(self, val: int):
            self.val = val

        def to_dict(self) -> dict:
            return {{"inner_val": self.val, self.val: "indexed"}}

    def __init__(self):
        self.inners = [self.Inner(i) for i in range(3)]

    def to_dict(self) -> dict:
        return {{
            "count": len(self.inners),
            "items": [i.to_dict() for i in self.inners],
            0: self.inners[0].to_dict(),  # Int key
        }}

outer_{idx} = Outer_{idx}()
d_{idx} = outer_{idx}.to_dict()
'''

    @staticmethod
    def enum_to_dict(idx: int) -> str:
        """Enum with dict conversion."""
        return f'''# METHOD_CLASH_012: Enum to dict
from enum import Enum, auto

class Status_{idx}(Enum):
    PENDING = auto()
    ACTIVE = auto()
    DONE = auto()

    def to_dict(self) -> dict:
        return {{
            "name": self.name,
            "value": self.value,
            self.value: self.name,  # Int key
        }}

    @classmethod
    def all_to_dict(cls) -> dict:
        return {{member.value: member.name for member in cls}}

s_{idx} = Status_{idx}.ACTIVE
d_{idx} = s_{idx}.to_dict()
all_{idx} = Status_{idx}.all_to_dict()
'''

    @classmethod
    def all_generators(cls) -> List[Callable[[int], str]]:
        return [
            cls.basic_to_dict_heterogeneous,
            cls.from_dict_untyped,
            cls.inheritance_dict_conflict,
            cls.protocol_dict_method,
            cls.dataclass_asdict,
            cls.mixin_dict_methods,
            cls.dict_subclass,
            cls.property_returns_dict,
            cls.class_with_dict_operators,
            cls.static_method_dict_factory,
            cls.nested_class_dict,
            cls.enum_to_dict,
        ]


# ==============================================================================
# PATTERN 4: THE MODULE BOUNDARY
# ==============================================================================

class ModuleBoundaryGenerator:
    """
    Generates multi-file scenarios for cross-module dict type inference.

    Attacks:
    - Dict defined in one file, used in another
    - Re-exports of dict values
    - Star imports with dicts
    - Circular imports with dicts
    """

    @staticmethod
    def shared_dict_definition(idx: int) -> Tuple[str, str]:
        """Dict defined in one module, imported in another."""
        module_a = f'''# MODULE_BOUNDARY_001_A: Dict definitions

GLOBAL_STR_DICT_{idx} = {{"a": 1, "b": 2}}
GLOBAL_INT_DICT_{idx} = {{1: "a", 2: "b"}}
GLOBAL_MIXED_DICT_{idx} = {{"str": 1, 42: "int"}}
GLOBAL_EMPTY_DICT_{idx} = {{}}

def get_dict_{idx}() -> dict:
    return GLOBAL_STR_DICT_{idx}

def get_int_dict_{idx}() -> dict:
    return GLOBAL_INT_DICT_{idx}
'''
        module_b = f'''# MODULE_BOUNDARY_001_B: Dict usage
from module_{idx:04d}_a import (
    GLOBAL_STR_DICT_{idx},
    GLOBAL_INT_DICT_{idx},
    GLOBAL_MIXED_DICT_{idx},
    GLOBAL_EMPTY_DICT_{idx},
    get_dict_{idx},
)

def use_imported_{idx}():
    # Using imported dicts
    GLOBAL_STR_DICT_{idx}["c"] = 3
    GLOBAL_INT_DICT_{idx}[3] = "c"
    GLOBAL_MIXED_DICT_{idx}["new"] = 99
    GLOBAL_MIXED_DICT_{idx}[99] = "new"
    GLOBAL_EMPTY_DICT_{idx}["first"] = 1
    GLOBAL_EMPTY_DICT_{idx}[1] = "first"  # Mixed keys!

    d = get_dict_{idx}()
    d["extra"] = 42
    return d
'''
        return module_a, module_b

    @staticmethod
    def type_alias_export(idx: int) -> Tuple[str, str]:
        """Type alias for dict exported and used."""
        module_a = f'''# MODULE_BOUNDARY_002_A: Type alias export
from typing import Dict, Any, TypeAlias

# Type aliases
StringDict_{idx}: TypeAlias = Dict[str, int]
IntKeyDict_{idx}: TypeAlias = Dict[int, str]
MixedDict_{idx}: TypeAlias = Dict[Any, Any]

def create_string_dict_{idx}() -> StringDict_{idx}:
    return {{"x": 1}}

def create_int_dict_{idx}() -> IntKeyDict_{idx}:
    return {{1: "x"}}
'''
        module_b = f'''# MODULE_BOUNDARY_002_B: Type alias usage
from module_{idx:04d}_a import (
    StringDict_{idx},
    IntKeyDict_{idx},
    MixedDict_{idx},
    create_string_dict_{idx},
    create_int_dict_{idx},
)

def process_{idx}():
    sd: StringDict_{idx} = create_string_dict_{idx}()
    id: IntKeyDict_{idx} = create_int_dict_{idx}()

    # Cross-assign (type violation)
    # sd[1] = 2  # Should fail: int key in StringDict
    # id["a"] = "b"  # Should fail: str key in IntKeyDict

    # MixedDict allows anything
    md: MixedDict_{idx} = {{**sd, **id}}  # Merged
    return md
'''
        return module_a, module_b

    @staticmethod
    def dict_in_class_export(idx: int) -> Tuple[str, str]:
        """Class with dict attribute exported."""
        module_a = f'''# MODULE_BOUNDARY_003_A: Class with dict export

class Container_{idx}:
    data: dict = {{}}  # Class-level dict

    def __init__(self, initial: dict = None):
        self.instance_data = initial or {{}}

    @classmethod
    def add_class_data(cls, key, value):
        cls.data[key] = value

    def add_instance_data(self, key, value):
        self.instance_data[key] = value

def get_container_{idx}() -> Container_{idx}:
    return Container_{idx}({{"init": 1}})
'''
        module_b = f'''# MODULE_BOUNDARY_003_B: Class with dict usage
from module_{idx:04d}_a import Container_{idx}, get_container_{idx}

def manipulate_{idx}():
    c = get_container_{idx}()

    # Modify class-level dict
    Container_{idx}.add_class_data("str_key", 1)
    Container_{idx}.add_class_data(42, "int_key")  # Int key

    # Modify instance dict
    c.add_instance_data("a", 1)
    c.add_instance_data(1, "a")  # Int key

    return c.instance_data, Container_{idx}.data
'''
        return module_a, module_b

    @staticmethod
    def star_import_dicts(idx: int) -> Tuple[str, str]:
        """Star import of dict values."""
        module_a = f'''# MODULE_BOUNDARY_004_A: Dicts for star import

__all__ = [
    "CONFIG_{idx}",
    "SETTINGS_{idx}",
    "LOOKUP_{idx}",
    "make_config_{idx}",
]

CONFIG_{idx} = {{"debug": True, "level": 5}}
SETTINGS_{idx} = {{0: "off", 1: "on", 2: "auto"}}
LOOKUP_{idx} = {{"a": 1, 1: "a"}}  # Mixed keys

def make_config_{idx}(extra: dict = None) -> dict:
    base = CONFIG_{idx}.copy()
    if extra:
        base.update(extra)
    return base
'''
        module_b = f'''# MODULE_BOUNDARY_004_B: Star import usage
from module_{idx:04d}_a import *

def use_imported_{idx}():
    # Using star-imported values
    CONFIG_{idx}["new"] = "value"
    SETTINGS_{idx}[3] = "custom"

    # Mixed key usage
    LOOKUP_{idx}["b"] = 2
    LOOKUP_{idx}[2] = "b"

    cfg = make_config_{idx}({{"extra": "data"}})
    return cfg
'''
        return module_a, module_b

    @staticmethod
    def reexport_dicts(idx: int) -> Tuple[str, str, str]:
        """Chain of re-exports."""
        module_a = f'''# MODULE_BOUNDARY_005_A: Original dict definitions

ORIGINAL_DICT_{idx} = {{"source": "A", "value": 1}}
ORIGINAL_INT_DICT_{idx} = {{1: "one", 2: "two"}}
'''
        module_b = f'''# MODULE_BOUNDARY_005_B: Re-export with modification
from module_{idx:04d}_a import ORIGINAL_DICT_{idx}, ORIGINAL_INT_DICT_{idx}

# Re-export with additions
EXTENDED_DICT_{idx} = {{**ORIGINAL_DICT_{idx}, "extended": True}}
EXTENDED_INT_DICT_{idx} = {{**ORIGINAL_INT_DICT_{idx}, 3: "three"}}

# Mixed re-export
MIXED_{idx} = {{**ORIGINAL_DICT_{idx}, **ORIGINAL_INT_DICT_{idx}}}
'''
        module_c = f'''# MODULE_BOUNDARY_005_C: Final consumer
from module_{idx:04d}_b import EXTENDED_DICT_{idx}, EXTENDED_INT_DICT_{idx}, MIXED_{idx}

def final_use_{idx}():
    # Type should propagate through chain
    EXTENDED_DICT_{idx}["final"] = "touch"
    EXTENDED_INT_DICT_{idx}[4] = "four"

    # Mixed has both key types
    print(MIXED_{idx}["source"])  # str key
    print(MIXED_{idx}[1])  # int key

    return MIXED_{idx}
'''
        return module_a, module_b, module_c

    @staticmethod
    def function_returns_imported_dict_type(idx: int) -> Tuple[str, str]:
        """Function's return dict type comes from another module."""
        module_a = f'''# MODULE_BOUNDARY_006_A: Dict factory

def create_str_dict_{idx}() -> dict:
    return {{"key": "value"}}

def create_int_dict_{idx}() -> dict:
    return {{1: 100, 2: 200}}

def create_mixed_{idx}() -> dict:
    return {{"str": 1, 42: "int"}}
'''
        module_b = f'''# MODULE_BOUNDARY_006_B: Uses factory, modifies result
from module_{idx:04d}_a import create_str_dict_{idx}, create_int_dict_{idx}, create_mixed_{idx}

def extend_str_{idx}() -> dict:
    d = create_str_dict_{idx}()
    d["extra"] = "more"
    d[0] = "now_int_key"  # Type changes!
    return d

def combine_{idx}() -> dict:
    s = create_str_dict_{idx}()
    i = create_int_dict_{idx}()
    return {{**s, **i}}  # str and int keys mixed
'''
        return module_a, module_b

    @staticmethod
    def generate_pair(idx: int) -> List[Tuple[str, str]]:
        """Generate a pair of related modules."""
        generators = [
            ModuleBoundaryGenerator.shared_dict_definition,
            ModuleBoundaryGenerator.type_alias_export,
            ModuleBoundaryGenerator.dict_in_class_export,
            ModuleBoundaryGenerator.star_import_dicts,
            ModuleBoundaryGenerator.function_returns_imported_dict_type,
        ]
        gen = random.choice(generators)
        result = gen(idx)

        if len(result) == 2:
            return [(f"module_{idx:04d}_a.py", result[0]),
                    (f"module_{idx:04d}_b.py", result[1])]
        else:
            return [(f"module_{idx:04d}_a.py", result[0]),
                    (f"module_{idx:04d}_b.py", result[1]),
                    (f"module_{idx:04d}_c.py", result[2])]

    @classmethod
    def all_generators(cls) -> List[Callable[[int], Tuple[str, ...]]]:
        return [
            cls.shared_dict_definition,
            cls.type_alias_export,
            cls.dict_in_class_export,
            cls.star_import_dicts,
            cls.reexport_dicts,
            cls.function_returns_imported_dict_type,
        ]


# ==============================================================================
# COMPOSITE GENERATORS (Extra complexity)
# ==============================================================================

class CompositeGenerator:
    """Generate files that combine multiple patterns for maximum chaos."""

    @staticmethod
    def all_patterns_combined(idx: int) -> str:
        """File that hits all four patterns."""
        return f'''# COMPOSITE_001: All patterns combined
from typing import Dict, List, Optional, Any, Protocol
from dataclasses import dataclass

# === LITERAL TRAP ===
empty_dict_{idx} = {{}}
str_dict_{idx} = {{"a": 1}}
int_dict_{idx} = {{1: "a"}}
mixed_dict_{idx} = {{"s": 1, 2: "i", (3, 4): "t", None: "n"}}

# === FLOW GAP ===
def process_{idx}(d: dict) -> dict:
    d["processed"] = True
    d[999] = "int_key_added"
    return d

def chain_{idx}():
    d = {{}}
    d = process_{idx}(d)
    return d

# === METHOD CLASH ===
class Serializable_{idx}(Protocol):
    def to_dict(self) -> dict: ...

@dataclass
class Entity_{idx}:
    name: str
    value: int
    tags: List[str]
    meta: dict

    def to_dict(self) -> dict:
        return {{
            "name": self.name,
            "value": self.value,
            "tags": self.tags,
            "meta": self.meta,
            self.value: "value_as_key",  # Int key!
        }}

    @classmethod
    def from_dict(cls, data: dict) -> "Entity_{idx}":
        return cls(
            name=data.get("name", ""),
            value=data.get("value", 0),
            tags=data.get("tags", []),
            meta=data.get("meta", {{}})
        )

# === Usage ===
def main_{idx}():
    # Create from literal
    e = Entity_{idx}("test", 42, ["a", "b"], {{"x": 1, 1: "x"}})

    # Convert to dict
    d = e.to_dict()

    # Flow through function
    d = process_{idx}(d)

    # Merge with different key types
    result = {{**str_dict_{idx}, **int_dict_{idx}, **d}}

    return result

result_{idx} = main_{idx}()
'''

    @staticmethod
    def deeply_nested_dicts(idx: int) -> str:
        """Deeply nested dict structures."""
        return f'''# COMPOSITE_002: Deep nesting

def create_deep_{idx}(depth: int = 5) -> dict:
    if depth == 0:
        return {{"leaf": True, 0: "zero"}}

    return {{
        "level": depth,
        depth: f"depth_{{depth}}",  # Int key
        "child": create_deep_{idx}(depth - 1),
        "siblings": [
            create_deep_{idx}(max(0, depth - 2)),
            {{"inline": depth, depth: "inline_int_key"}},
        ],
    }}

def access_deep_{idx}(d: dict, path: List) -> Any:
    current = d
    for key in path:
        if isinstance(current, dict):
            current = current.get(key)
        elif isinstance(current, list) and isinstance(key, int):
            current = current[key]
        else:
            return None
    return current

deep_{idx} = create_deep_{idx}()
value_{idx} = access_deep_{idx}(deep_{idx}, ["child", "child", "level"])
'''

    @staticmethod
    def dict_with_all_value_types(idx: int) -> str:
        """Dict containing every possible value type."""
        return f'''# COMPOSITE_003: All value types
from typing import Dict, List, Set, Tuple, Optional, Callable
from dataclasses import dataclass
from enum import Enum

class Status_{idx}(Enum):
    ACTIVE = 1
    INACTIVE = 2

@dataclass
class Inner_{idx}:
    x: int

def func_{idx}(x: int) -> int:
    return x * 2

# Dict with every value type
mega_dict_{idx}: dict = {{
    # Primitives
    "int_val": 42,
    "float_val": 3.14,
    "str_val": "hello",
    "bool_val": True,
    "none_val": None,

    # Collections
    "list_val": [1, 2, 3],
    "tuple_val": (1, "a", True),
    "set_val": {{1, 2, 3}},
    "dict_val": {{"nested": "dict"}},

    # Complex types
    "class_val": Inner_{idx}(42),
    "enum_val": Status_{idx}.ACTIVE,
    "func_val": func_{idx},

    # Non-string keys mixed in
    0: "int_key",
    1.5: "float_key",
    True: "bool_key",
    (1, 2): "tuple_key",
    None: "none_key",
}}

def access_all_{idx}():
    results = []
    for key in mega_dict_{idx}:
        results.append((key, type(key).__name__, mega_dict_{idx}[key]))
    return results
'''

    @classmethod
    def all_generators(cls) -> List[Callable[[int], str]]:
        return [
            cls.all_patterns_combined,
            cls.deeply_nested_dicts,
            cls.dict_with_all_value_types,
        ]


# ==============================================================================
# MAIN GENERATOR
# ==============================================================================

@dataclass
class GeneratedFile:
    """Represents a generated Python file."""
    filename: str
    content: str
    pattern: str
    variant: str


class AmbiguityCorpusGenerator:
    """Main generator for the ambiguity corpus."""

    def __init__(self, output_dir: Path, file_count: int):
        self.output_dir = output_dir
        self.file_count = file_count
        self.files: List[GeneratedFile] = []

        # Pattern generators
        self.patterns = {
            "literal_trap": LiteralTrapGenerator.all_generators(),
            "flow_gap": FlowGapGenerator.all_generators(),
            "method_clash": MethodClashGenerator.all_generators(),
            "composite": CompositeGenerator.all_generators(),
        }

    def generate(self) -> List[GeneratedFile]:
        """Generate all files according to pattern weights."""
        self.files = []
        idx = 0

        # Calculate file counts per pattern
        literal_count = int(self.file_count * PATTERN_WEIGHTS["literal_trap"])
        flow_count = int(self.file_count * PATTERN_WEIGHTS["flow_gap"])
        method_count = int(self.file_count * PATTERN_WEIGHTS["method_clash"])
        module_count = int(self.file_count * PATTERN_WEIGHTS["module_boundary"])

        # Generate LITERAL TRAP files
        print(f"Generating {literal_count} LITERAL_TRAP files...")
        for i in range(literal_count):
            gen = random.choice(self.patterns["literal_trap"])
            content = gen(idx)
            filename = f"literal_trap_{idx:04d}.py"
            self.files.append(GeneratedFile(filename, content, "literal_trap", gen.__name__))
            idx += 1

        # Generate FLOW GAP files
        print(f"Generating {flow_count} FLOW_GAP files...")
        for i in range(flow_count):
            gen = random.choice(self.patterns["flow_gap"])
            content = gen(idx)
            filename = f"flow_gap_{idx:04d}.py"
            self.files.append(GeneratedFile(filename, content, "flow_gap", gen.__name__))
            idx += 1

        # Generate METHOD CLASH files
        print(f"Generating {method_count} METHOD_CLASH files...")
        for i in range(method_count):
            gen = random.choice(self.patterns["method_clash"])
            content = gen(idx)
            filename = f"method_clash_{idx:04d}.py"
            self.files.append(GeneratedFile(filename, content, "method_clash", gen.__name__))
            idx += 1

        # Generate MODULE BOUNDARY files (pairs/triples)
        print(f"Generating {module_count} MODULE_BOUNDARY file sets...")
        boundary_idx = 0
        while len([f for f in self.files if f.pattern == "module_boundary"]) < module_count:
            file_pairs = ModuleBoundaryGenerator.generate_pair(boundary_idx)
            for filename, content in file_pairs:
                full_filename = f"module_boundary_{filename}"
                self.files.append(GeneratedFile(
                    full_filename, content, "module_boundary", "multi_file"
                ))
            boundary_idx += 1

        # Add composite files to fill remaining quota
        remaining = self.file_count - len(self.files)
        if remaining > 0:
            print(f"Generating {remaining} COMPOSITE files...")
            for i in range(remaining):
                gen = random.choice(self.patterns["composite"])
                content = gen(idx)
                filename = f"composite_{idx:04d}.py"
                self.files.append(GeneratedFile(filename, content, "composite", gen.__name__))
                idx += 1

        return self.files

    def write_files(self) -> int:
        """Write all generated files to disk."""
        self.output_dir.mkdir(parents=True, exist_ok=True)

        written = 0
        for gf in self.files:
            filepath = self.output_dir / gf.filename
            filepath.write_text(gf.content)
            written += 1

        return written

    def write_manifest(self) -> Path:
        """Write manifest file with metadata."""
        manifest_path = self.output_dir / "MANIFEST.txt"

        pattern_counts = {}
        variant_counts = {}

        for gf in self.files:
            pattern_counts[gf.pattern] = pattern_counts.get(gf.pattern, 0) + 1
            key = f"{gf.pattern}:{gf.variant}"
            variant_counts[key] = variant_counts.get(key, 0) + 1

        content = f"""OPERATION AMBIGUITY - Corpus Manifest
=====================================
Generated: {self.file_count} target files
Seed: 0xDEPYLER (deterministic)

Pattern Distribution:
"""
        for pattern, count in sorted(pattern_counts.items()):
            content += f"  {pattern}: {count} files ({100*count/len(self.files):.1f}%)\n"

        content += "\nVariant Breakdown:\n"
        for variant, count in sorted(variant_counts.items()):
            content += f"  {variant}: {count}\n"

        content += f"""
Purpose:
  These files are designed to trigger E0308 (Type Mismatch) and E0599
  (Method Not Found) errors in the depyler transpiler by exploiting
  the Type System Schism between:
    - type_tokens.rs (expects HashMap<String, V>)
    - type_mapper.rs (infers HashMap<DepylerValue, V>)

Next Steps:
  1. Vectorize failures:
     cargo run --bin depyler -- graph vectorize \\
       --corpus training_corpus/ambiguity_v1 \\
       --output training_corpus/ambiguity_vectors.ndjson

  2. Retrain Oracle:
     cargo run --bin depyler-oracle -- train \\
       --input training_corpus/ambiguity_vectors.ndjson \\
       --output ~/.depyler/depyler_oracle_v3.23.apr
"""
        manifest_path.write_text(content)
        return manifest_path


def main():
    parser = argparse.ArgumentParser(
        description="Generate ambiguity corpus for depyler Oracle training"
    )
    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=Path(DEFAULT_OUTPUT_DIR),
        help=f"Output directory (default: {DEFAULT_OUTPUT_DIR})"
    )
    parser.add_argument(
        "--count", "-n",
        type=int,
        default=DEFAULT_FILE_COUNT,
        help=f"Number of files to generate (default: {DEFAULT_FILE_COUNT})"
    )
    args = parser.parse_args()

    print("=" * 60)
    print("OPERATION AMBIGUITY - Civil War Corpus Generator")
    print("=" * 60)
    print(f"Target: {args.count} files -> {args.output}")
    print()

    generator = AmbiguityCorpusGenerator(args.output, args.count)

    # Generate files
    files = generator.generate()
    print(f"\nGenerated {len(files)} file definitions")

    # Write to disk
    written = generator.write_files()
    print(f"Wrote {written} files to {args.output}")

    # Write manifest
    manifest = generator.write_manifest()
    print(f"Wrote manifest to {manifest}")

    # Summary
    print("\n" + "=" * 60)
    print("CORPUS GENERATION COMPLETE")
    print("=" * 60)
    print(f"Total files: {written}")
    print(f"Location: {args.output.absolute()}")
    print("\nNext: Run vectorization pipeline")
    print(f"  cargo run --bin depyler -- graph vectorize \\")
    print(f"    --corpus {args.output} \\")
    print(f"    --output training_corpus/ambiguity_vectors.ndjson")


if __name__ == "__main__":
    main()
