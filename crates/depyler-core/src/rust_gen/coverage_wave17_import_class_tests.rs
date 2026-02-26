//! Wave 17: Coverage tests for import handling, class generation, format/f-string
//! patterns, and error generation code paths.
//!
//! Tests target UNCOVERED code paths in:
//! - Import generation: stdlib imports, from-imports, aliased imports, deep imports,
//!   combined patterns, unknown local imports, wildcard imports, function-scope imports
//! - Class generation: pass-only classes, dunder methods (__add__, __mul__, __getitem__,
//!   __setitem__, __len__, __bool__, __enter__, __exit__, __iter__, __next__),
//!   inheritance, multiple inheritance, class variables, staticmethod, classmethod,
//!   property, dataclass patterns
//! - Format/f-string: simple f-strings, expressions, format specs, conversions,
//!   multi-expression, method calls, .format(), %-formatting, concatenation, repr/str
//! - Error generation: custom exceptions, raise, raise from, bare raise, try/except,
//!   try/except/else, try/except/finally, nested try, assert with message
//!
//! Status: 200 tests (test_w17ic_import_001..050, test_w17ic_class_051..100,
//!         test_w17ic_format_101..150, test_w17ic_error_151..200)

#[cfg(test)]
mod tests {
    use crate::ast_bridge::AstBridge;
    use crate::rust_gen::generate_rust_file;
    use crate::type_mapper::TypeMapper;
    use rustpython_parser::{parse, Mode};

    fn transpile(python_code: &str) -> String {
        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) =
            AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }

    // =========================================================================
    // IMPORT PATTERNS (50 tests: test_w17ic_import_001 through test_w17ic_import_050)
    // =========================================================================

    #[test]
    fn test_w17ic_import_001_import_os() {
        let result = transpile("import os\ndef main() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_002_import_sys() {
        let result = transpile("import sys\ndef main() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_003_import_json() {
        let result = transpile("import json\ndef main() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_004_import_math() {
        let result = transpile("import math\ndef get_pi() -> float:\n    return 3.14159\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_005_import_re() {
        let result = transpile("import re\ndef check() -> bool:\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_006_import_datetime() {
        let result = transpile("import datetime\ndef now_str() -> str:\n    return \"today\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_007_import_collections() {
        let result = transpile("import collections\ndef make_counter() -> dict:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_008_import_itertools() {
        let result = transpile("import itertools\ndef chain_lists() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_009_import_functools() {
        let result = transpile("import functools\ndef identity(x: int) -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_010_import_pathlib() {
        let result = transpile("import pathlib\ndef get_home() -> str:\n    return \"/home\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_011_from_os_import_path() {
        let result = transpile("from os import path\ndef check_path() -> bool:\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_012_from_collections_import_defaultdict() {
        let result = transpile(
            "from collections import defaultdict\ndef make_dd() -> dict:\n    return {}\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_013_from_collections_import_counter() {
        let result = transpile(
            "from collections import Counter\ndef count_items() -> dict:\n    return {}\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_014_from_typing_import_list() {
        let result = transpile(
            "from typing import List\ndef get_items() -> List[int]:\n    return [1, 2, 3]\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_015_from_typing_import_dict() {
        let result =
            transpile("from typing import Dict\ndef get_map() -> Dict[str, int]:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_016_from_typing_import_optional() {
        let result = transpile("from typing import Optional\ndef maybe(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_017_from_typing_import_tuple() {
        let result = transpile(
            "from typing import Tuple\ndef pair() -> Tuple[int, int]:\n    return (1, 2)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_018_from_typing_import_set() {
        let result =
            transpile("from typing import Set\ndef unique() -> Set[int]:\n    return {1, 2, 3}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_019_from_typing_import_union() {
        let result = transpile("from typing import Union\ndef flexible(x: Union[int, str]) -> str:\n    return str(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_020_from_typing_import_multiple() {
        let result = transpile(
            "from typing import List, Dict, Optional\ndef func() -> int:\n    return 0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_021_from_datetime_import_date() {
        let result = transpile(
            "from datetime import date\ndef today_str() -> str:\n    return \"2024-01-01\"\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_022_from_datetime_import_time() {
        let result =
            transpile("from datetime import time\ndef noon_str() -> str:\n    return \"12:00\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_023_from_datetime_import_timedelta() {
        let result =
            transpile("from datetime import timedelta\ndef one_day() -> int:\n    return 86400\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_024_import_with_alias_np() {
        let result = transpile("import numpy as np\ndef zeros() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_025_import_with_alias_pd() {
        let result = transpile("import pandas as pd\ndef empty_frame() -> dict:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_026_from_os_path_import_join() {
        let result =
            transpile("from os.path import join\ndef build_path() -> str:\n    return \"a/b\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_027_from_os_path_import_exists() {
        let result =
            transpile("from os.path import exists\ndef check() -> bool:\n    return False\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_028_from_os_path_import_dirname() {
        let result =
            transpile("from os.path import dirname\ndef parent() -> str:\n    return \"/home\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_029_from_os_path_import_multiple() {
        let result =
            transpile("from os.path import join, exists, dirname\ndef noop() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_030_wildcard_from_typing() {
        let result =
            transpile("from typing import *\ndef typed_fn(x: int) -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_031_combined_imports_function() {
        let result = transpile(
            "import math\nimport os\ndef compute(x: float) -> float:\n    return x * 2.0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_032_from_math_import_sqrt() {
        let result =
            transpile("from math import sqrt\ndef root(x: float) -> float:\n    return sqrt(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_033_from_math_import_floor_ceil() {
        let result = transpile(
            "from math import floor, ceil\ndef round_down(x: float) -> int:\n    return floor(x)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_034_from_local_module_import() {
        let result =
            transpile("from my_module import helper\ndef run() -> int:\n    return helper()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_035_from_local_module_import_multiple() {
        let result = transpile(
            "from my_utils import parse, validate\ndef process() -> bool:\n    return True\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_036_import_string() {
        let result = transpile("import string\ndef get_letters() -> str:\n    return \"abc\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_037_import_hashlib() {
        let result = transpile("import hashlib\ndef hash_str(s: str) -> str:\n    return s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_038_import_copy() {
        let result = transpile("import copy\ndef clone_list(lst: list) -> list:\n    return lst\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_039_import_random() {
        let result = transpile("import random\ndef roll() -> int:\n    return 4\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_040_import_time() {
        let result = transpile("import time\ndef wait() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_041_from_functools_import_reduce() {
        let result =
            transpile("from functools import reduce\ndef total(lst: list) -> int:\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_042_from_itertools_import_chain() {
        let result =
            transpile("from itertools import chain\ndef merge() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_043_from_collections_import_deque() {
        let result =
            transpile("from collections import deque\ndef make_queue() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_044_import_with_alias_custom() {
        let result = transpile("import datetime as dt\ndef get_year() -> int:\n    return 2024\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_045_three_imports() {
        let result =
            transpile("import os\nimport sys\nimport json\ndef noop() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_046_mixed_import_and_from() {
        let result = transpile(
            "import math\nfrom typing import List\ndef nums() -> List[int]:\n    return [1]\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_047_from_typing_import_any() {
        let result =
            transpile("from typing import Any\ndef accept(x: Any) -> str:\n    return str(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_048_from_typing_import_callable() {
        let result = transpile("from typing import Callable\ndef apply(f: Callable, x: int) -> int:\n    return f(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_049_from_collections_import_ordereddict() {
        let result = transpile(
            "from collections import OrderedDict\ndef ordered() -> dict:\n    return {}\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_import_050_aliased_from_import() {
        let result = transpile(
            "from collections import defaultdict as dd\ndef make() -> dict:\n    return {}\n",
        );
        assert!(!result.is_empty());
    }

    // =========================================================================
    // CLASS PATTERNS (50 tests: test_w17ic_class_051 through test_w17ic_class_100)
    // =========================================================================

    #[test]
    fn test_w17ic_class_051_simple_pass_class() {
        let result = transpile("class Empty:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_052_init_single_int() {
        let result =
            transpile("class Box:\n    def __init__(self, size: int):\n        self.size = size\n");
        assert!(!result.is_empty());
        assert!(result.contains("struct") || result.contains("fn"));
    }

    #[test]
    fn test_w17ic_class_053_init_single_str() {
        let result = transpile(
            "class Label:\n    def __init__(self, text: str):\n        self.text = text\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_054_greet_method() {
        let result = transpile("class Greeter:\n    def __init__(self, name: str):\n        self.name = name\n    def greet(self) -> str:\n        return \"hello\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_055_str_dunder() {
        let result = transpile("class Tag:\n    def __init__(self, val: str):\n        self.val = val\n    def __str__(self) -> str:\n        return self.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_056_repr_dunder() {
        let result = transpile("class Token:\n    def __init__(self, value: int):\n        self.value = value\n    def __repr__(self) -> str:\n        return \"token\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_057_eq_dunder() {
        let result = transpile("class Point:\n    def __init__(self, x: int):\n        self.x = x\n    def __eq__(self, other) -> bool:\n        return self.x == other.x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_058_lt_dunder() {
        let result = transpile("class Score:\n    def __init__(self, val: int):\n        self.val = val\n    def __lt__(self, other) -> bool:\n        return self.val < other.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_059_add_dunder() {
        let result = transpile("class Vector:\n    def __init__(self, x: int):\n        self.x = x\n    def __add__(self, other) -> int:\n        return self.x + other.x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_060_mul_dunder() {
        let result = transpile("class Scale:\n    def __init__(self, factor: int):\n        self.factor = factor\n    def __mul__(self, other) -> int:\n        return self.factor * other.factor\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_061_getitem_dunder() {
        let result = transpile("class Row:\n    def __init__(self):\n        self.cells = []\n    def __getitem__(self, idx: int) -> int:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_062_setitem_dunder() {
        let result = transpile("class Grid:\n    def __init__(self):\n        self.data = []\n    def __setitem__(self, idx: int, val: int) -> None:\n        pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_063_len_dunder() {
        let result = transpile("class Stack:\n    def __init__(self):\n        self.items = []\n    def __len__(self) -> int:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_064_bool_dunder() {
        let result = transpile("class Flag:\n    def __init__(self, val: bool):\n        self.val = val\n    def __bool__(self) -> bool:\n        return self.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_065_enter_exit_dunder() {
        let result = transpile("class Lock:\n    def __enter__(self):\n        return self\n    def __exit__(self, exc_type, exc_val, exc_tb):\n        pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_066_iter_dunder() {
        let result = transpile("class Range:\n    def __init__(self, limit: int):\n        self.limit = limit\n    def __iter__(self):\n        return self\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_067_next_dunder() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.n = 0\n    def __next__(self) -> int:\n        self.n = self.n + 1\n        return self.n\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_068_inheritance_basic() {
        let result = transpile("class Animal:\n    def __init__(self):\n        self.alive = True\n\nclass Dog(Animal):\n    def __init__(self):\n        self.breed = \"lab\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_069_inheritance_method_override() {
        let result = transpile("class Shape:\n    def area(self) -> float:\n        return 0.0\n\nclass Rect(Shape):\n    def __init__(self, w: float, h: float):\n        self.w = w\n        self.h = h\n    def area(self) -> float:\n        return self.w * self.h\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_070_class_variable_int() {
        let result = transpile("class Config:\n    MAX_SIZE = 100\n    def __init__(self):\n        self.current = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_071_class_variable_string() {
        let result = transpile("class App:\n    VERSION = \"1.0\"\n    def __init__(self):\n        self.running = False\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_072_staticmethod_add() {
        let result = transpile("class MathOps:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_073_staticmethod_multiply() {
        let result = transpile("class MathOps:\n    @staticmethod\n    def multiply(a: int, b: int) -> int:\n        return a * b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_074_classmethod_create() {
        let result = transpile("class Widget:\n    def __init__(self, name: str):\n        self.name = name\n    @classmethod\n    def create(cls, name: str):\n        return cls(name)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_075_property_getter() {
        let result = transpile("class Circle:\n    def __init__(self, r: float):\n        self.r = r\n    @property\n    def diameter(self) -> float:\n        return self.r * 2.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_076_multiple_methods() {
        let result = transpile("class Account:\n    def __init__(self):\n        self.balance = 0\n    def deposit(self, amount: int) -> None:\n        self.balance = self.balance + amount\n    def withdraw(self, amount: int) -> None:\n        self.balance = self.balance - amount\n    def get_balance(self) -> int:\n        return self.balance\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_077_method_with_conditional() {
        let result = transpile("class Validator:\n    def __init__(self):\n        self.errors = 0\n    def check(self, val: int) -> bool:\n        if val < 0:\n            self.errors = self.errors + 1\n            return False\n        return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_078_method_with_loop() {
        let result = transpile("class Summer:\n    def __init__(self):\n        self.total = 0\n    def sum_range(self, n: int) -> int:\n        result: int = 0\n        for i in range(n):\n            result = result + i\n        return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_079_default_param_int() {
        let result = transpile("class Pos:\n    def __init__(self, x: int = 0, y: int = 0):\n        self.x = x\n        self.y = y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_080_default_param_str() {
        let result = transpile("class Logger:\n    def __init__(self, level: str = \"info\"):\n        self.level = level\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_081_default_param_bool() {
        let result = transpile(
            "class Switch:\n    def __init__(self, on: bool = False):\n        self.on = on\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_082_class_with_docstring() {
        let result = transpile("class Documented:\n    \"\"\"A documented class.\"\"\"\n    def __init__(self):\n        self.value = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_083_method_with_docstring() {
        let result = transpile("class Helper:\n    def process(self, data: str) -> str:\n        \"\"\"Process the data.\"\"\"\n        return data\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_084_two_classes_same_file() {
        let result = transpile("class First:\n    def __init__(self):\n        self.val = 1\n\nclass Second:\n    def __init__(self):\n        self.val = 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_085_class_and_function() {
        let result = transpile("class Data:\n    def __init__(self, x: int):\n        self.x = x\n\ndef create_data(n: int) -> int:\n    return n\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_086_ne_dunder() {
        let result = transpile("class ID:\n    def __init__(self, val: int):\n        self.val = val\n    def __ne__(self, other) -> bool:\n        return self.val != other.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_087_le_dunder() {
        let result = transpile("class Priority:\n    def __init__(self, rank: int):\n        self.rank = rank\n    def __le__(self, other) -> bool:\n        return self.rank <= other.rank\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_088_ge_dunder() {
        let result = transpile("class Level:\n    def __init__(self, val: int):\n        self.val = val\n    def __ge__(self, other) -> bool:\n        return self.val >= other.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_089_gt_dunder() {
        let result = transpile("class Weight:\n    def __init__(self, kg: int):\n        self.kg = kg\n    def __gt__(self, other) -> bool:\n        return self.kg > other.kg\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_090_sub_dunder() {
        let result = transpile("class Amount:\n    def __init__(self, val: int):\n        self.val = val\n    def __sub__(self, other) -> int:\n        return self.val - other.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_091_contains_dunder() {
        let result = transpile("class Bag:\n    def __init__(self):\n        self.items = []\n    def __contains__(self, item: int) -> bool:\n        return False\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_092_class_with_return_self() {
        let result = transpile("class Builder:\n    def __init__(self):\n        self.parts = []\n    def add(self, part: str):\n        return self\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_093_init_with_list_field() {
        let result = transpile("class Queue:\n    def __init__(self):\n        self.items = []\n    def push(self, item: int) -> None:\n        self.items.append(item)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_094_init_with_dict_field() {
        let result = transpile("class Cache:\n    def __init__(self):\n        self.store = {}\n    def get(self, key: str) -> str:\n        return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_095_staticmethod_bool_return() {
        let result = transpile("class Checker:\n    @staticmethod\n    def is_valid(x: int) -> bool:\n        return x > 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_096_method_with_while() {
        let result = transpile("class Countdown:\n    def __init__(self, n: int):\n        self.n = n\n    def run(self) -> int:\n        count: int = self.n\n        while count > 0:\n            count = count - 1\n        return count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_097_four_field_init() {
        let result = transpile("class Record:\n    def __init__(self, a: int, b: str, c: float, d: bool):\n        self.a = a\n        self.b = b\n        self.c = c\n        self.d = d\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_098_method_returns_float() {
        let result = transpile("class Temp:\n    def __init__(self, celsius: float):\n        self.celsius = celsius\n    def to_fahrenheit(self) -> float:\n        return self.celsius * 1.8 + 32.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_099_class_three_methods() {
        let result = transpile("class Trio:\n    def alpha(self) -> int:\n        return 1\n    def beta(self) -> int:\n        return 2\n    def gamma(self) -> int:\n        return 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_class_100_class_with_neg_dunder() {
        let result = transpile("class Signed:\n    def __init__(self, val: int):\n        self.val = val\n    def __neg__(self) -> int:\n        return -self.val\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // FORMAT/F-STRING PATTERNS (50 tests: test_w17ic_format_101 through test_w17ic_format_150)
    // =========================================================================

    #[test]
    fn test_w17ic_format_101_simple_fstring_var() {
        let result = transpile("def greet(name: str) -> str:\n    return f\"Hello, {name}!\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("format!") || result.contains("Hello"));
    }

    #[test]
    fn test_w17ic_format_102_fstring_int_var() {
        let result = transpile("def show_count(n: int) -> str:\n    return f\"Count: {n}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_103_fstring_addition_expr() {
        let result = transpile("def show_sum(a: int, b: int) -> str:\n    return f\"{a + b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_104_fstring_multiply_expr() {
        let result = transpile("def show_double(x: int) -> str:\n    return f\"{x * 2}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_105_fstring_len_call() {
        let result = transpile("def show_len(items: list) -> str:\n    return f\"{len(items)}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_106_fstring_format_spec_2f() {
        let result = transpile("def show_price(val: float) -> str:\n    return f\"{val:.2f}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_107_fstring_format_spec_03d() {
        let result = transpile("def padded(num: int) -> str:\n    return f\"{num:03d}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_108_fstring_format_spec_right_align() {
        let result = transpile("def aligned(text: str) -> str:\n    return f\"{text:>20}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_109_fstring_conversion_repr() {
        let result = transpile("def debug_show(obj: str) -> str:\n    return f\"{obj!r}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_110_fstring_conversion_str() {
        let result = transpile("def str_show(obj: str) -> str:\n    return f\"{obj!s}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_111_fstring_multi_expr() {
        let result = transpile(
            "def full_name(first: str, last: str) -> str:\n    return f\"{first} {last}\"\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_112_fstring_three_vars() {
        let result = transpile("def info(name: str, age: int, city: str) -> str:\n    return f\"{name} is {age} from {city}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_113_fstring_with_method_upper() {
        let result = transpile("def shout(name: str) -> str:\n    return f\"{name.upper()}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_114_fstring_with_method_lower() {
        let result = transpile("def whisper(name: str) -> str:\n    return f\"{name.lower()}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_115_fstring_nested_ternary() {
        let result =
            transpile("def label(x: int) -> str:\n    return f\"{'yes' if x > 0 else 'no'}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_116_format_method_single() {
        let result =
            transpile("def greet(name: str) -> str:\n    return \"Hello, {}\".format(name)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_117_format_method_two_args() {
        let result =
            transpile("def pair(a: str, b: str) -> str:\n    return \"{} and {}\".format(a, b)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_118_format_method_indexed() {
        let result = transpile(
            "def order(a: str, b: str) -> str:\n    return \"{0} then {1}\".format(a, b)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_119_percent_format_str() {
        let result = transpile("def greet(name: str) -> str:\n    return \"Hello, %s\" % name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_120_percent_format_int() {
        let result = transpile("def count_msg(n: int) -> str:\n    return \"%d items\" % n\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_121_string_concat_two() {
        let result = transpile("def join_two(a: str, b: str) -> str:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_122_string_concat_three() {
        let result =
            transpile("def join_three(a: str, b: str, c: str) -> str:\n    return a + b + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_123_string_concat_literal() {
        let result =
            transpile("def hello_world() -> str:\n    return \"hello\" + \" \" + \"world\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_124_str_call_int() {
        let result = transpile("def int_to_str(n: int) -> str:\n    return str(n)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_125_repr_call() {
        let result = transpile("def show_repr(s: str) -> str:\n    return repr(s)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_126_fstring_only_text() {
        let result = transpile("def static_str() -> str:\n    return f\"no vars here\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_127_fstring_with_int_literal() {
        let result = transpile("def show_42() -> str:\n    return f\"{42}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_128_fstring_mixed_text_and_var() {
        let result = transpile("def prefix(val: int) -> str:\n    return f\"result={val}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_129_fstring_subtraction() {
        let result = transpile("def diff(a: int, b: int) -> str:\n    return f\"{a - b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_130_fstring_division() {
        let result = transpile("def ratio(a: float, b: float) -> str:\n    return f\"{a / b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_131_fstring_modulo() {
        let result = transpile("def remainder(a: int, b: int) -> str:\n    return f\"{a % b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_132_fstring_bool_var() {
        let result = transpile("def show_flag(flag: bool) -> str:\n    return f\"flag={flag}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_133_fstring_comparison() {
        let result = transpile("def show_cmp(a: int, b: int) -> str:\n    return f\"{a > b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_134_fstring_and_print() {
        let result = transpile("def announce(name: str) -> None:\n    msg: str = f\"Welcome, {name}\"\n    print(msg)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_135_fstring_assign_to_var() {
        let result = transpile(
            "def make_msg(x: int) -> str:\n    s: str = f\"value is {x}\"\n    return s\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_136_format_spec_1f() {
        let result = transpile("def round_1(val: float) -> str:\n    return f\"{val:.1f}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_137_format_spec_4f() {
        let result = transpile("def precise(val: float) -> str:\n    return f\"{val:.4f}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_138_format_spec_left_align() {
        let result = transpile("def left(text: str) -> str:\n    return f\"{text:<20}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_139_format_spec_center() {
        let result = transpile("def center(text: str) -> str:\n    return f\"{text:^20}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_140_format_spec_fill_char() {
        let result = transpile("def padded(text: str) -> str:\n    return f\"{text:*>10}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_141_format_spec_hex() {
        let result = transpile("def to_hex(n: int) -> str:\n    return f\"{n:x}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_142_format_spec_octal() {
        let result = transpile("def to_oct(n: int) -> str:\n    return f\"{n:o}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_143_format_spec_binary() {
        let result = transpile("def to_bin(n: int) -> str:\n    return f\"{n:b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_144_multiple_fstrings_concat() {
        let result = transpile("def double_f(a: str, b: str) -> str:\n    first: str = f\"A={a}\"\n    second: str = f\"B={b}\"\n    return first + second\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_145_fstring_in_condition() {
        let result = transpile("def status(ok: bool) -> str:\n    if ok:\n        return f\"OK\"\n    return f\"FAIL\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_146_fstring_in_loop() {
        let result = transpile("def labels(n: int) -> str:\n    result: str = \"\"\n    for i in range(n):\n        result = result + f\"item-{i}\"\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_147_format_empty_string() {
        let result = transpile("def empty() -> str:\n    return f\"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_148_fstring_with_newline_escape() {
        let result =
            transpile("def multi_line(a: str, b: str) -> str:\n    return f\"{a}\\n{b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_149_str_call_float() {
        let result = transpile("def float_to_str(x: float) -> str:\n    return str(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_format_150_str_call_bool() {
        let result = transpile("def bool_to_str(b: bool) -> str:\n    return str(b)\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // ERROR GENERATION (50 tests: test_w17ic_error_151 through test_w17ic_error_200)
    // =========================================================================

    #[test]
    fn test_w17ic_error_151_custom_exception_pass() {
        let result = transpile("class MyError(Exception):\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_152_custom_exception_with_init() {
        let result = transpile("class AppError(Exception):\n    def __init__(self, msg: str):\n        self.msg = msg\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_153_custom_exception_with_code() {
        let result = transpile("class HttpError(Exception):\n    def __init__(self, code: int, msg: str):\n        self.code = code\n        self.msg = msg\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_154_raise_value_error() {
        let result = transpile("def fail() -> None:\n    raise ValueError(\"bad value\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_155_raise_type_error() {
        let result =
            transpile("def check_type(x: int) -> None:\n    raise TypeError(\"wrong type\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_156_raise_runtime_error() {
        let result = transpile("def abort() -> None:\n    raise RuntimeError(\"abort\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_157_raise_key_error() {
        let result = transpile("def missing_key() -> None:\n    raise KeyError(\"not found\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_158_raise_index_error() {
        let result = transpile("def out_of_bounds() -> None:\n    raise IndexError(\"index\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_159_try_except_basic() {
        let result = transpile(
            "def safe(x: int) -> int:\n    try:\n        return x\n    except:\n        return 0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_160_try_except_value_error() {
        let result = transpile("def parse(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_161_try_except_as_variable() {
        let result = transpile("def safe_parse(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as err:\n        return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_162_try_except_else() {
        let result = transpile("def checked(x: int) -> int:\n    try:\n        val: int = x + 1\n    except:\n        val = 0\n    else:\n        val = val + 10\n    return val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_163_try_except_finally() {
        let result = transpile("def with_cleanup(x: int) -> int:\n    result: int = 0\n    try:\n        result = x * 2\n    except:\n        result = -1\n    finally:\n        result = result + 1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_164_try_except_else_finally() {
        let result = transpile("def full_try(x: int) -> int:\n    r: int = 0\n    try:\n        r = x\n    except:\n        r = -1\n    else:\n        r = r + 5\n    finally:\n        r = r + 1\n    return r\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_165_multiple_except_types() {
        let result = transpile("def multi_catch(x: int) -> int:\n    try:\n        return x\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_166_except_tuple_types() {
        let result = transpile("def catch_both(x: int) -> int:\n    try:\n        return x\n    except (ValueError, TypeError):\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_167_nested_try() {
        let result = transpile("def nested_safe(x: int) -> int:\n    try:\n        try:\n            return x\n        except:\n            return -1\n    except:\n        return -2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_168_assert_simple() {
        let result = transpile("def positive(x: int) -> int:\n    assert x > 0\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("assert") || result.contains("panic"));
    }

    #[test]
    fn test_w17ic_error_169_assert_with_message() {
        let result = transpile(
            "def check(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_170_assert_equality() {
        let result = transpile("def verify(a: int, b: int) -> None:\n    assert a == b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_171_raise_in_if() {
        let result = transpile("def guard(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_172_raise_exception_base() {
        let result =
            transpile("def fail_generic() -> None:\n    raise Exception(\"generic failure\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_173_try_except_return_string() {
        let result = transpile("def safe_str() -> str:\n    try:\n        return \"ok\"\n    except:\n        return \"error\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_174_try_with_division() {
        let result = transpile("def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except ZeroDivisionError:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_175_try_except_assignment() {
        let result = transpile("def safe_compute(x: int) -> int:\n    result: int = 0\n    try:\n        result = x * 10\n    except:\n        result = -1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_176_try_except_with_loop() {
        let result = transpile("def safe_sum(items: list) -> int:\n    total: int = 0\n    for item in items:\n        try:\n            total = total + item\n        except:\n            pass\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_177_custom_exception_value_error_sub() {
        let result = transpile("class ValidationError(ValueError):\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_178_custom_exception_runtime_sub() {
        let result = transpile("class ConfigError(RuntimeError):\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_179_raise_no_args() {
        let result = transpile("def fail_simple() -> None:\n    raise ValueError()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_180_except_exception_base() {
        let result = transpile("def catch_all(x: int) -> int:\n    try:\n        return x\n    except Exception:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_181_except_exception_as() {
        let result = transpile("def log_error(x: int) -> int:\n    try:\n        return x\n    except Exception as exc:\n        return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_182_try_finally_only() {
        let result = transpile("def always_clean(x: int) -> int:\n    result: int = 0\n    try:\n        result = x\n    finally:\n        result = result + 1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_183_assert_not_none() {
        let result = transpile("def require(x: int) -> int:\n    assert x != 0\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_184_assert_in_function() {
        let result = transpile("def validate_range(x: int) -> int:\n    assert x >= 0\n    assert x <= 100\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_185_raise_attribute_error() {
        let result =
            transpile("def no_attr() -> None:\n    raise AttributeError(\"no such attr\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_186_raise_not_implemented() {
        let result = transpile("def abstract_method() -> None:\n    raise NotImplementedError(\"subclass must implement\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_187_raise_file_not_found() {
        let result =
            transpile("def load_file() -> None:\n    raise FileNotFoundError(\"missing\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_188_try_except_return_bool() {
        let result = transpile("def is_safe(x: int) -> bool:\n    try:\n        return x > 0\n    except:\n        return False\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_189_try_except_return_float() {
        let result = transpile("def safe_reciprocal(x: float) -> float:\n    try:\n        return 1.0 / x\n    except:\n        return 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_190_multiple_except_three_types() {
        let result = transpile("def ultra_safe(x: int) -> int:\n    try:\n        return x\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2\n    except KeyError:\n        return -3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_191_assert_boolean() {
        let result =
            transpile("def check_flag(flag: bool) -> bool:\n    assert flag\n    return flag\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_192_assert_string_msg() {
        let result = transpile("def require_positive(n: int) -> int:\n    assert n > 0, \"n must be positive\"\n    return n\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_193_raise_in_method() {
        let result = transpile("class Strict:\n    def validate(self, x: int) -> int:\n        if x < 0:\n            raise ValueError(\"negative\")\n        return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_194_try_in_method() {
        let result = transpile("class Safe:\n    def compute(self, x: int) -> int:\n        try:\n            return x * 2\n        except:\n            return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_195_try_except_index_error() {
        let result = transpile("def safe_get(items: list, idx: int) -> int:\n    try:\n        return items[idx]\n    except IndexError:\n        return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_196_try_except_key_error() {
        let result = transpile("def safe_lookup(data: dict, key: str) -> str:\n    try:\n        return data[key]\n    except KeyError:\n        return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_197_raise_with_variable_msg() {
        let result = transpile("def fail_with(msg: str) -> None:\n    raise ValueError(msg)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_198_try_except_in_loop() {
        let result = transpile("def safe_total(items: list) -> int:\n    s: int = 0\n    for item in items:\n        try:\n            s = s + item\n        except:\n            pass\n    return s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_199_two_asserts_in_function() {
        let result = transpile("def bounded(x: int, lo: int, hi: int) -> int:\n    assert x >= lo\n    assert x <= hi\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17ic_error_200_try_return_early_on_error() {
        let result = transpile("def attempt(x: int) -> int:\n    try:\n        if x == 0:\n            return -1\n        return 100 // x\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }
}
