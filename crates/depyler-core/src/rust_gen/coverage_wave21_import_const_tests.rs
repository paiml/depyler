//! Wave 21 coverage tests: generate_rust_file_internal, generate_import_tokens,
//! generate_constant_tokens, and class handling code paths.
//!
//! Targets uncovered code paths in:
//! - Import generation: needs_chrono, needs_tempfile, needs_itertools, needs_statrs,
//!   needs_url, async detection, collections, json, csv, re, os, sys, pathlib,
//!   typing, dataclasses, functools, enum, math, random, hashlib, time, contextlib,
//!   struct, io, copy, duplicate dedup, os.path, module aliases
//! - Module-level constants: dict, list, set, tuple, int, float, bool, string,
//!   lambda, TypeVar skip, duplicate dedup, call constants, binary expr,
//!   index, list/set/dict comprehensions, multiple constants
//! - Class handling: __init__, multiple methods, @property, @staticmethod,
//!   @classmethod, dataclass fields/defaults, inheritance, __str__, __repr__,
//!   __eq__, __hash__, __len__, __iter__, self mutation, return annotations,
//!   enum, NamedTuple, nested methods, default params, Optional params,
//!   *args, class variables
//!
//! 200 tests total

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

    // ========================================================================
    // SECTION 1: IMPORT HANDLING (tests 001-060)
    // ========================================================================

    #[test]
    fn test_w21ic_001_from_datetime_import_datetime() {
        let result =
            transpile("from datetime import datetime\ndef now() -> str:\n    return \"now\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_002_from_datetime_import_timedelta() {
        let result =
            transpile("from datetime import timedelta\ndef delta() -> int:\n    return 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_003_from_datetime_import_date() {
        let result =
            transpile("from datetime import date\ndef today() -> str:\n    return \"today\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_004_from_datetime_import_multiple() {
        let result = transpile(
            "from datetime import datetime, timedelta, date\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_005_import_tempfile() {
        let result = transpile("import tempfile\ndef make_temp() -> str:\n    return \"/tmp\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_006_from_itertools_import_combinations() {
        let result =
            transpile("from itertools import combinations\ndef combo() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_007_from_itertools_import_permutations() {
        let result =
            transpile("from itertools import permutations\ndef perm() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_008_from_itertools_import_product() {
        let result =
            transpile("from itertools import product\ndef prod() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_009_from_itertools_import_chain() {
        let result =
            transpile("from itertools import chain\ndef chained() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_010_from_itertools_import_multiple() {
        let result = transpile("from itertools import combinations, permutations, product, chain\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_011_from_statistics_import_mean() {
        let result =
            transpile("from statistics import mean\ndef avg() -> float:\n    return 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_012_from_statistics_import_median() {
        let result =
            transpile("from statistics import median\ndef mid() -> float:\n    return 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_013_from_statistics_import_stdev() {
        let result =
            transpile("from statistics import stdev\ndef sd() -> float:\n    return 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_014_from_statistics_import_variance() {
        let result =
            transpile("from statistics import variance\ndef var() -> float:\n    return 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_015_from_statistics_import_multiple() {
        let result = transpile(
            "from statistics import mean, median, stdev, variance\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_016_from_urllib_parse_import_urlparse() {
        let result = transpile(
            "from urllib.parse import urlparse\ndef parse_url() -> str:\n    return \"\"\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_017_from_urllib_parse_import_parse_qs() {
        let result = transpile(
            "from urllib.parse import parse_qs\ndef parse_query() -> dict:\n    return {}\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_018_from_urllib_parse_import_urlencode() {
        let result = transpile(
            "from urllib.parse import urlencode\ndef encode() -> str:\n    return \"\"\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_019_from_urllib_parse_import_multiple() {
        let result = transpile("from urllib.parse import urlparse, parse_qs, urlencode\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_020_import_asyncio() {
        let result = transpile("import asyncio\nasync def run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_021_from_collections_import_defaultdict() {
        let result =
            transpile("from collections import defaultdict\ndef make() -> dict:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_022_from_collections_import_counter() {
        let result =
            transpile("from collections import Counter\ndef count() -> dict:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_023_from_collections_import_ordereddict() {
        let result = transpile(
            "from collections import OrderedDict\ndef ordered() -> dict:\n    return {}\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_024_from_collections_import_deque() {
        let result =
            transpile("from collections import deque\ndef make_deque() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_025_from_collections_import_namedtuple() {
        let result =
            transpile("from collections import namedtuple\ndef make() -> str:\n    return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_026_from_collections_import_multiple() {
        let result = transpile("from collections import defaultdict, Counter, OrderedDict, deque, namedtuple\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_027_import_json() {
        let result = transpile("import json\ndef to_json() -> str:\n    return \"{}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_028_import_csv() {
        let result = transpile("import csv\ndef read_csv() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_029_import_re() {
        let result = transpile("import re\ndef match_pattern() -> bool:\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_030_import_os() {
        let result = transpile("import os\ndef get_cwd() -> str:\n    return \".\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_031_import_sys() {
        let result = transpile("import sys\ndef get_argv() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_032_from_pathlib_import_path() {
        let result =
            transpile("from pathlib import Path\ndef get_path() -> str:\n    return \"/\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_033_from_typing_import_list() {
        let result =
            transpile("from typing import List\ndef items() -> List[int]:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_034_from_typing_import_dict() {
        let result =
            transpile("from typing import Dict\ndef mapping() -> Dict[str, int]:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_035_from_typing_import_optional() {
        let result = transpile(
            "from typing import Optional\ndef maybe() -> Optional[int]:\n    return None\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_036_from_typing_import_tuple() {
        let result = transpile(
            "from typing import Tuple\ndef pair() -> Tuple[int, str]:\n    return (1, \"a\")\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_037_from_typing_import_set() {
        let result =
            transpile("from typing import Set\ndef uniq() -> Set[int]:\n    return set()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_038_from_typing_import_any() {
        let result = transpile("from typing import Any\ndef anything() -> Any:\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_039_from_typing_import_union() {
        let result = transpile(
            "from typing import Union\ndef flexible() -> Union[int, str]:\n    return 1\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_040_from_typing_import_callable() {
        let result = transpile("from typing import Callable\ndef higher() -> int:\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_041_from_typing_import_all() {
        let result = transpile("from typing import List, Dict, Optional, Tuple, Set, Any, Union, Callable\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_042_from_dataclasses_import_dataclass() {
        let result = transpile("from dataclasses import dataclass\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_043_from_dataclasses_import_field() {
        let result =
            transpile("from dataclasses import dataclass, field\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_044_from_functools_import_lru_cache() {
        let result =
            transpile("from functools import lru_cache\ndef cached() -> int:\n    return 42\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_045_from_functools_import_reduce() {
        let result = transpile("from functools import reduce\ndef total() -> int:\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_046_from_enum_import_enum() {
        let result = transpile("from enum import Enum\ndef get_status() -> int:\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_047_from_enum_import_auto() {
        let result =
            transpile("from enum import Enum, auto\ndef get_auto() -> int:\n    return 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_048_import_math_with_usage() {
        let result = transpile("import math\ndef calc() -> float:\n    return 1.41421\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_049_import_random() {
        let result = transpile("import random\ndef roll() -> int:\n    return 4\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_050_import_hashlib() {
        let result = transpile("import hashlib\ndef hash_it() -> str:\n    return \"abc\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_051_import_time() {
        let result = transpile("import time\ndef wait() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_052_from_contextlib_import_contextmanager() {
        let result =
            transpile("from contextlib import contextmanager\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_053_from_contextlib_import_suppress() {
        let result =
            transpile("from contextlib import suppress\ndef safe_op() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_054_import_struct() {
        let result = transpile("import struct\ndef pack_data() -> bytes:\n    return b\"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_055_import_io() {
        let result = transpile("import io\ndef buffer() -> str:\n    return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_056_import_copy() {
        let result = transpile("import copy\ndef clone_it() -> int:\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_057_duplicate_import_dedup() {
        let result = transpile("import os\nimport os\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_058_import_os_path() {
        let result = transpile("import os\ndef exists() -> bool:\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_059_import_alias_numpy() {
        let result = transpile("import numpy as np\ndef zeros() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_060_multiple_stdlib_imports() {
        let result = transpile("import os\nimport sys\nimport json\nimport re\nimport math\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: MODULE-LEVEL CONSTANTS (tests 061-120)
    // ========================================================================

    #[test]
    fn test_w21ic_061_dict_constant() {
        let result = transpile("CONFIG = {\"key\": \"value\"}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_062_dict_constant_int_keys() {
        let result = transpile("LOOKUP = {1: \"one\", 2: \"two\"}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_063_dict_constant_nested() {
        let result =
            transpile("NESTED = {\"a\": \"b\", \"c\": \"d\"}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_064_list_constant_ints() {
        let result = transpile("ITEMS = [1, 2, 3]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_065_list_constant_strings() {
        let result =
            transpile("NAMES = [\"alice\", \"bob\", \"carol\"]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_066_list_constant_floats() {
        let result = transpile("WEIGHTS = [1.5, 2.5, 3.5]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_067_set_constant_ints() {
        let result = transpile("UNIQUE = {1, 2, 3}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_068_set_constant_strings() {
        let result =
            transpile("TAGS = {\"alpha\", \"beta\", \"gamma\"}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_069_tuple_constant() {
        let result = transpile("PAIR = (1, 2)\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_070_tuple_constant_strings() {
        let result = transpile("COORDS = (\"x\", \"y\", \"z\")\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_071_int_constant() {
        let result = transpile("MAX_SIZE = 100\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("MAX_SIZE"));
    }

    #[test]
    fn test_w21ic_072_int_constant_zero() {
        let result = transpile("ZERO = 0\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_073_int_constant_negative() {
        let result = transpile("MIN_VAL = -1\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_074_float_constant() {
        let result = transpile("RATE = 0.05\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_075_float_constant_large() {
        let result = transpile("BIG_NUM = 1000000.5\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_076_bool_constant_true() {
        let result = transpile("DEBUG = True\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("DEBUG"));
    }

    #[test]
    fn test_w21ic_077_bool_constant_false() {
        let result = transpile("RELEASE = False\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_078_string_constant() {
        let result = transpile("NAME = \"depyler\"\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("NAME"));
    }

    #[test]
    fn test_w21ic_079_string_constant_empty() {
        let result = transpile("EMPTY = \"\"\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_080_string_constant_multiword() {
        let result = transpile("GREETING = \"hello world\"\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_081_lambda_constant_simple() {
        let result = transpile("double = lambda x: x * 2\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn double"));
    }

    #[test]
    fn test_w21ic_082_lambda_constant_add() {
        let result = transpile("add_one = lambda x: x + 1\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn add_one"));
    }

    #[test]
    fn test_w21ic_083_lambda_constant_two_params() {
        let result = transpile("add = lambda x, y: x + y\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn add"));
    }

    #[test]
    fn test_w21ic_084_typevar_skip() {
        let result = transpile("from typing import TypeVar\nT = TypeVar(\"T\")\ndef identity(x: int) -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_085_typevar_skip_with_bound() {
        let result = transpile("from typing import TypeVar\nS = TypeVar(\"S\")\ndef wrap(x: int) -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_086_duplicate_constant_dedup() {
        let result = transpile("X = 1\nX = 2\ndef run() -> int:\n    return X\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_087_duplicate_constant_string_dedup() {
        let result = transpile("MSG = \"old\"\nMSG = \"new\"\ndef run() -> str:\n    return MSG\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_088_constant_from_binary_add() {
        let result = transpile("TOTAL = 10 + 20\ndef run() -> int:\n    return TOTAL\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_089_constant_from_binary_mul() {
        let result = transpile("PRODUCT = 5 * 6\ndef run() -> int:\n    return PRODUCT\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_090_constant_from_binary_sub() {
        let result = transpile("DIFF = 100 - 42\ndef run() -> int:\n    return DIFF\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_091_constant_from_binary_mod() {
        let result = transpile("REMAINDER = 17 % 5\ndef run() -> int:\n    return REMAINDER\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_092_list_comprehension_constant() {
        let result =
            transpile("SQUARES = [x * x for x in range(10)]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_093_list_comprehension_constant_with_filter() {
        let result = transpile(
            "EVENS = [x for x in range(20) if x % 2 == 0]\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_094_set_comprehension_constant() {
        let result = transpile(
            "EVEN_SET = {x for x in range(10) if x % 2 == 0}\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_095_dict_comprehension_constant() {
        let result =
            transpile("SQUARED = {x: x * x for x in range(5)}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_096_multiple_int_constants() {
        let result = transpile("A = 1\nB = 2\nC = 3\ndef run() -> int:\n    return A + B + C\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_097_multiple_string_constants() {
        let result = transpile(
            "FIRST = \"hello\"\nSECOND = \"world\"\ndef run() -> str:\n    return FIRST\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_098_mixed_type_constants() {
        let result = transpile("COUNT = 10\nRATE = 0.5\nNAME = \"test\"\nACTIVE = True\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_099_constant_large_list() {
        let result = transpile(
            "VALS = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_100_constant_large_dict() {
        let result = transpile("MAP = {\"a\": 1, \"b\": 2, \"c\": 3, \"d\": 4, \"e\": 5}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_101_constant_bool_list() {
        let result = transpile("FLAGS = [True, False, True]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_102_constant_empty_list() {
        let result = transpile("EMPTY_LIST = []\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_103_constant_empty_dict() {
        let result = transpile("EMPTY_DICT = {}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_104_constant_single_element_tuple() {
        let result = transpile("SINGLE = (42,)\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_105_constant_negative_float() {
        let result = transpile("NEG_RATE = -0.01\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_106_constant_with_import_context() {
        let result = transpile("import math\nTAU = 6.28318\ndef run() -> float:\n    return TAU\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_107_constant_string_with_spaces() {
        let result =
            transpile("LABEL = \"hello world test\"\ndef run() -> str:\n    return LABEL\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_108_constant_int_power_of_two() {
        let result = transpile("BUFFER_SIZE = 4096\ndef run() -> int:\n    return BUFFER_SIZE\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_109_constant_list_of_tuples() {
        let result = transpile("PAIRS = [(1, 2), (3, 4)]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_110_constant_float_zero() {
        let result = transpile("EPSILON = 0.0\ndef run() -> float:\n    return EPSILON\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_111_constant_binary_floor_div() {
        let result = transpile("HALF = 10 // 2\ndef run() -> int:\n    return HALF\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_112_constant_binary_power() {
        let result = transpile("CUBE = 2 ** 3\ndef run() -> int:\n    return CUBE\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_113_constant_list_comp_strings() {
        let result = transpile(
            "UPPER = [x.upper() for x in [\"a\", \"b\", \"c\"]]\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_114_constant_set_ints() {
        let result = transpile("PRIMES = {2, 3, 5, 7, 11}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_115_constant_tuple_three_elements() {
        let result = transpile("RGB = (255, 128, 0)\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_116_constant_with_function_using_it() {
        let result = transpile("LIMIT = 50\ndef check(x: int) -> bool:\n    return x < LIMIT\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_117_constant_list_mixed_ints() {
        let result = transpile("STEPS = [0, 1, 1, 2, 3, 5, 8, 13]\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_118_constant_string_special_chars() {
        let result = transpile("SEPARATOR = \"---\"\ndef run() -> str:\n    return SEPARATOR\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_119_constant_dict_string_to_int() {
        let result =
            transpile("SCORES = {\"math\": 95, \"science\": 88}\ndef run() -> None:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_120_constant_multiple_lambdas() {
        let result = transpile(
            "square = lambda x: x * x\ncube = lambda x: x * x * x\ndef run() -> None:\n    pass\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn square"));
        assert!(result.contains("fn cube"));
    }

    // ========================================================================
    // SECTION 3: CLASS HANDLING (tests 121-200)
    // ========================================================================

    #[test]
    fn test_w21ic_121_simple_class_with_init() {
        let code = "class Point:\n    def __init__(self, x: int, y: int) -> None:\n        self.x = x\n        self.y = y\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("struct Point"));
    }

    #[test]
    fn test_w21ic_122_class_with_init_and_method() {
        let code = "class Box:\n    def __init__(self, w: int) -> None:\n        self.w = w\n    def area(self) -> int:\n        return self.w * self.w\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn area"));
    }

    #[test]
    fn test_w21ic_123_class_with_multiple_methods() {
        let code = "class Calc:\n    def __init__(self) -> None:\n        self.val = 0\n    def add(self, x: int) -> int:\n        return self.val + x\n    def sub(self, x: int) -> int:\n        return self.val - x\n    def mul(self, x: int) -> int:\n        return self.val * x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn add"));
        assert!(result.contains("fn sub"));
        assert!(result.contains("fn mul"));
    }

    #[test]
    fn test_w21ic_124_class_property_decorator() {
        let code = "class Circle:\n    def __init__(self, r: float) -> None:\n        self.r = r\n    @property\n    def diameter(self) -> float:\n        return self.r * 2.0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn diameter"));
    }

    #[test]
    fn test_w21ic_125_class_staticmethod() {
        let code = "class MathHelper:\n    @staticmethod\n    def double(x: int) -> int:\n        return x * 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn double"));
    }

    #[test]
    fn test_w21ic_126_class_classmethod() {
        let code =
            "class Factory:\n    @classmethod\n    def create(cls) -> int:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn create"));
    }

    #[test]
    fn test_w21ic_127_dataclass_basic() {
        let code = "from dataclasses import dataclass\n\n@dataclass\nclass Item:\n    name: str\n    price: float\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Item"));
    }

    #[test]
    fn test_w21ic_128_dataclass_with_defaults() {
        let code = "from dataclasses import dataclass\n\n@dataclass\nclass Config:\n    debug: bool = False\n    level: int = 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Config"));
    }

    #[test]
    fn test_w21ic_129_class_inheritance_simple() {
        let code = "class Animal:\n    def __init__(self, name: str) -> None:\n        self.name = name\n    def speak(self) -> str:\n        return self.name\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("struct Animal"));
    }

    #[test]
    fn test_w21ic_130_class_with_str_method() {
        let code = "class Person:\n    def __init__(self, name: str) -> None:\n        self.name = name\n    def __str__(self) -> str:\n        return self.name\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Display"));
    }

    #[test]
    fn test_w21ic_131_class_with_repr_method() {
        let code = "class Data:\n    def __init__(self, val: int) -> None:\n        self.val = val\n    def __repr__(self) -> str:\n        return \"Data\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Debug") || result.contains("Display"));
    }

    #[test]
    fn test_w21ic_132_class_with_eq_method() {
        let code = "class Token:\n    def __init__(self, kind: str) -> None:\n        self.kind = kind\n    def __eq__(self, other: object) -> bool:\n        return True\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("PartialEq") || result.contains("eq"));
    }

    #[test]
    fn test_w21ic_133_class_with_hash_method() {
        let code = "class Key:\n    def __init__(self, val: int) -> None:\n        self.val = val\n    def __hash__(self) -> int:\n        return self.val\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_134_class_with_len_method() {
        let code = "class Stack:\n    def __init__(self) -> None:\n        self.items: list = []\n    def __len__(self) -> int:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn len"));
    }

    #[test]
    fn test_w21ic_135_class_with_iter_method() {
        let code = "class Range:\n    def __init__(self, n: int) -> None:\n        self.n = n\n    def __iter__(self) -> list:\n        return []\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_136_class_mutating_self() {
        let code = "class Counter:\n    def __init__(self) -> None:\n        self.count = 0\n    def increment(self) -> None:\n        self.count = self.count + 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut"));
    }

    #[test]
    fn test_w21ic_137_class_mutating_self_decrement() {
        let code = "class Timer:\n    def __init__(self, secs: int) -> None:\n        self.secs = secs\n    def tick(self) -> None:\n        self.secs = self.secs - 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut"));
    }

    #[test]
    fn test_w21ic_138_class_with_return_type_annotation() {
        let code = "class Converter:\n    def __init__(self) -> None:\n        pass\n    def to_string(self, x: int) -> str:\n        return \"val\"\n    def to_int(self, s: str) -> int:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("String") || result.contains("str"));
    }

    #[test]
    fn test_w21ic_139_enum_class() {
        let code = "from enum import Enum\n\nclass Color(Enum):\n    RED = 1\n    GREEN = 2\n    BLUE = 3\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Color"));
    }

    #[test]
    fn test_w21ic_140_enum_class_string_values() {
        let code = "from enum import Enum\n\nclass Status(Enum):\n    ACTIVE = \"active\"\n    INACTIVE = \"inactive\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Status"));
    }

    #[test]
    fn test_w21ic_141_class_with_default_param_int() {
        let code = "class Builder:\n    def __init__(self, size: int = 10) -> None:\n        self.size = size\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_142_class_with_default_param_string() {
        let code = "class Logger:\n    def __init__(self, prefix: str = \"LOG\") -> None:\n        self.prefix = prefix\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_143_class_with_default_param_bool() {
        let code = "class Settings:\n    def __init__(self, verbose: bool = False) -> None:\n        self.verbose = verbose\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_144_class_with_optional_param() {
        let code = "from typing import Optional\n\nclass Node:\n    def __init__(self, val: int, label: Optional[str] = None) -> None:\n        self.val = val\n        self.label = label\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Option"));
    }

    #[test]
    fn test_w21ic_145_class_with_optional_return() {
        let code = "from typing import Optional\n\nclass Finder:\n    def __init__(self) -> None:\n        pass\n    def find(self, key: str) -> Optional[int]:\n        return None\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Option"));
    }

    #[test]
    fn test_w21ic_146_class_with_class_variable() {
        let code =
            "class Config:\n    VERSION = 1\n    def __init__(self) -> None:\n        pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_147_class_with_class_variable_string() {
        let code =
            "class App:\n    NAME = \"myapp\"\n    def __init__(self) -> None:\n        pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_148_class_nested_methods() {
        let code = "class Pipeline:\n    def __init__(self) -> None:\n        self.steps: list = []\n    def add(self, step: str) -> None:\n        pass\n    def run(self) -> str:\n        return \"done\"\n    def reset(self) -> None:\n        pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn add"));
        assert!(result.contains("fn run"));
        assert!(result.contains("fn reset"));
    }

    #[test]
    fn test_w21ic_149_class_method_returns_self_field() {
        let code = "class Wrapper:\n    def __init__(self, data: int) -> None:\n        self.data = data\n    def get(self) -> int:\n        return self.data\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn get"));
    }

    #[test]
    fn test_w21ic_150_class_method_with_list_param() {
        let code = "from typing import List\n\nclass Processor:\n    def __init__(self) -> None:\n        pass\n    def process(self, items: List[int]) -> int:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Vec"));
    }

    #[test]
    fn test_w21ic_151_class_method_with_dict_param() {
        let code = "from typing import Dict\n\nclass Store:\n    def __init__(self) -> None:\n        pass\n    def load(self, data: Dict[str, int]) -> None:\n        pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_152_class_with_bool_return() {
        let code = "class Validator:\n    def __init__(self, threshold: int) -> None:\n        self.threshold = threshold\n    def is_valid(self, x: int) -> bool:\n        return x > self.threshold\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("bool"));
    }

    #[test]
    fn test_w21ic_153_class_with_float_return() {
        let code = "class Stats:\n    def __init__(self, total: float) -> None:\n        self.total = total\n    def average(self, count: int) -> float:\n        return self.total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("f64"));
    }

    #[test]
    fn test_w21ic_154_class_str_and_repr() {
        let code = "class Tag:\n    def __init__(self, name: str) -> None:\n        self.name = name\n    def __str__(self) -> str:\n        return self.name\n    def __repr__(self) -> str:\n        return self.name\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_155_class_with_list_field() {
        let code = "from typing import List\n\nclass Queue:\n    def __init__(self) -> None:\n        self.items: List[int] = []\n    def enqueue(self, item: int) -> None:\n        pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_156_class_static_and_instance() {
        let code = "class MathUtil:\n    def __init__(self, base: int) -> None:\n        self.base = base\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b\n    def scale(self, factor: int) -> int:\n        return self.base * factor\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn add"));
        assert!(result.contains("fn scale"));
    }

    #[test]
    fn test_w21ic_157_class_with_string_method() {
        let code = "class Formatter:\n    def __init__(self, prefix: str) -> None:\n        self.prefix = prefix\n    def format_name(self, name: str) -> str:\n        return self.prefix\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_158_class_with_conditional_return() {
        let code = "class Checker:\n    def __init__(self, limit: int) -> None:\n        self.limit = limit\n    def check(self, val: int) -> str:\n        if val > self.limit:\n            return \"high\"\n        return \"low\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_159_class_with_loop_method() {
        let code = "class Summer:\n    def __init__(self) -> None:\n        pass\n    def total(self, nums: list) -> int:\n        result = 0\n        for n in nums:\n            result = result + n\n        return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("for"));
    }

    #[test]
    fn test_w21ic_160_empty_class() {
        let code = "class Empty:\n    pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_161_class_only_init() {
        let code = "class Simple:\n    def __init__(self) -> None:\n        self.x = 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Simple"));
    }

    #[test]
    fn test_w21ic_162_class_with_type_annotation_fields() {
        let code = "class Record:\n    def __init__(self, name: str, age: int, active: bool) -> None:\n        self.name = name\n        self.age = age\n        self.active = active\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("struct Record"));
    }

    #[test]
    fn test_w21ic_163_class_property_computed() {
        let code = "class Rectangle:\n    def __init__(self, w: int, h: int) -> None:\n        self.w = w\n        self.h = h\n    @property\n    def area(self) -> int:\n        return self.w * self.h\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn area"));
    }

    #[test]
    fn test_w21ic_164_class_multiple_properties() {
        let code = "class Shape:\n    def __init__(self, side: int) -> None:\n        self.side = side\n    @property\n    def perimeter(self) -> int:\n        return self.side * 4\n    @property\n    def area(self) -> int:\n        return self.side * self.side\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn perimeter"));
        assert!(result.contains("fn area"));
    }

    #[test]
    fn test_w21ic_165_dataclass_three_fields() {
        let code = "from dataclasses import dataclass\n\n@dataclass\nclass Employee:\n    name: str\n    age: int\n    salary: float\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Employee"));
    }

    #[test]
    fn test_w21ic_166_dataclass_with_bool_field() {
        let code = "from dataclasses import dataclass\n\n@dataclass\nclass Feature:\n    name: str\n    enabled: bool\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_167_class_method_multiple_params() {
        let code = "class Calculator:\n    def __init__(self) -> None:\n        pass\n    def compute(self, a: int, b: int, c: int) -> int:\n        return a + b + c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn compute"));
    }

    #[test]
    fn test_w21ic_168_class_method_no_params_besides_self() {
        let code = "class Greeter:\n    def __init__(self) -> None:\n        pass\n    def greet(self) -> str:\n        return \"hello\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn greet"));
    }

    #[test]
    fn test_w21ic_169_class_with_none_return() {
        let code = "class Logger:\n    def __init__(self) -> None:\n        pass\n    def log_message(self, msg: str) -> None:\n        pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_170_class_init_with_default_none() {
        let code = "from typing import Optional\n\nclass Cache:\n    def __init__(self, size: int = 100, name: Optional[str] = None) -> None:\n        self.size = size\n        self.name = name\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_171_class_with_two_init_params() {
        let code = "class Vector:\n    def __init__(self, x: float, y: float) -> None:\n        self.x = x\n        self.y = y\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("struct Vector"));
    }

    #[test]
    fn test_w21ic_172_class_method_returns_list() {
        let code = "from typing import List\n\nclass DataSource:\n    def __init__(self) -> None:\n        pass\n    def fetch(self) -> List[int]:\n        return [1, 2, 3]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Vec"));
    }

    #[test]
    fn test_w21ic_173_class_method_returns_dict() {
        let code = "from typing import Dict\n\nclass Mapper:\n    def __init__(self) -> None:\n        pass\n    def get_map(self) -> Dict[str, int]:\n        return {}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_174_class_with_self_reassignment() {
        let code = "class Accumulator:\n    def __init__(self) -> None:\n        self.total = 0\n    def add(self, x: int) -> None:\n        self.total = self.total + x\n    def reset(self) -> None:\n        self.total = 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut"));
    }

    #[test]
    fn test_w21ic_175_class_with_string_field_and_int_method() {
        let code = "class NamedCounter:\n    def __init__(self, name: str) -> None:\n        self.name = name\n        self.count = 0\n    def get_count(self) -> int:\n        return self.count\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_176_class_classmethod_returns_string() {
        let code =
            "class Info:\n    @classmethod\n    def version(cls) -> str:\n        return \"1.0\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_177_class_static_returns_bool() {
        let code = "class Util:\n    @staticmethod\n    def is_even(n: int) -> bool:\n        return n % 2 == 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn is_even"));
    }

    #[test]
    fn test_w21ic_178_class_with_comparison_method() {
        let code = "class Score:\n    def __init__(self, val: int) -> None:\n        self.val = val\n    def is_higher(self, other_val: int) -> bool:\n        return self.val > other_val\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_179_class_with_math_method() {
        let code = "class Geometry:\n    def __init__(self, radius: float) -> None:\n        self.radius = radius\n    def circumference(self) -> float:\n        return 2.0 * 3.14159 * self.radius\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_180_class_with_constant_and_method() {
        let code = "MAX_VAL = 1000\n\nclass Limiter:\n    def __init__(self, val: int) -> None:\n        self.val = val\n    def clamp(self) -> int:\n        return self.val\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_181_class_with_import_and_method() {
        let code = "import math\n\nclass Circle:\n    def __init__(self, r: float) -> None:\n        self.r = r\n    def get_radius(self) -> float:\n        return self.r\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_182_two_classes_in_module() {
        let code = "class First:\n    def __init__(self) -> None:\n        self.val = 1\n\nclass Second:\n    def __init__(self) -> None:\n        self.val = 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("First"));
        assert!(result.contains("Second"));
    }

    #[test]
    fn test_w21ic_183_class_and_function_in_module() {
        let code = "class Widget:\n    def __init__(self, name: str) -> None:\n        self.name = name\n\ndef make_widget() -> str:\n    return \"widget\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Widget"));
        assert!(result.contains("fn make_widget"));
    }

    #[test]
    fn test_w21ic_184_class_with_many_fields() {
        let code = "class Config:\n    def __init__(self, host: str, port: int, debug: bool, timeout: int) -> None:\n        self.host = host\n        self.port = port\n        self.debug = debug\n        self.timeout = timeout\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("struct Config"));
    }

    #[test]
    fn test_w21ic_185_class_method_with_early_return() {
        let code = "class Guard:\n    def __init__(self, active: bool) -> None:\n        self.active = active\n    def check(self) -> str:\n        if not self.active:\n            return \"inactive\"\n        return \"active\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_186_class_method_with_while_loop() {
        let code = "class Countdown:\n    def __init__(self, n: int) -> None:\n        self.n = n\n    def count(self) -> int:\n        result = 0\n        i = self.n\n        while i > 0:\n            result = result + i\n            i = i - 1\n        return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("while"));
    }

    #[test]
    fn test_w21ic_187_dataclass_single_field() {
        let code =
            "from dataclasses import dataclass\n\n@dataclass\nclass Wrapper:\n    value: int\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_188_dataclass_with_string_default() {
        let code = "from dataclasses import dataclass\n\n@dataclass\nclass Label:\n    text: str = \"default\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_189_class_method_string_concat() {
        let code = "class Joiner:\n    def __init__(self, sep: str) -> None:\n        self.sep = sep\n    def join_two(self, a: str, b: str) -> str:\n        return a + b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_190_class_with_init_only_pass() {
        let code = "class Marker:\n    def __init__(self) -> None:\n        pass\n    def mark(self) -> bool:\n        return True\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_191_class_with_multiple_mutating_methods() {
        let code = "class Bank:\n    def __init__(self, balance: int) -> None:\n        self.balance = balance\n    def deposit(self, amount: int) -> None:\n        self.balance = self.balance + amount\n    def withdraw(self, amount: int) -> None:\n        self.balance = self.balance - amount\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut"));
    }

    #[test]
    fn test_w21ic_192_class_with_int_class_var_and_instance_var() {
        let code = "class Tracker:\n    COUNT = 0\n    def __init__(self, name: str) -> None:\n        self.name = name\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_193_class_with_method_using_if_else() {
        let code = "class Classifier:\n    def __init__(self, threshold: float) -> None:\n        self.threshold = threshold\n    def classify(self, score: float) -> str:\n        if score >= self.threshold:\n            return \"pass\"\n        else:\n            return \"fail\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_194_class_with_method_using_elif() {
        let code = "class Grader:\n    def __init__(self) -> None:\n        pass\n    def grade(self, score: int) -> str:\n        if score >= 90:\n            return \"A\"\n        elif score >= 80:\n            return \"B\"\n        elif score >= 70:\n            return \"C\"\n        else:\n            return \"F\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_195_class_with_field_default_zero() {
        let code = "class Position:\n    def __init__(self, x: int = 0, y: int = 0) -> None:\n        self.x = x\n        self.y = y\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_196_class_with_property_and_method() {
        let code = "class Temperature:\n    def __init__(self, celsius: float) -> None:\n        self.celsius = celsius\n    @property\n    def fahrenheit(self) -> float:\n        return self.celsius * 1.8 + 32.0\n    def is_freezing(self) -> bool:\n        return self.celsius <= 0.0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn fahrenheit"));
        assert!(result.contains("fn is_freezing"));
    }

    #[test]
    fn test_w21ic_197_three_classes_in_module() {
        let code = "class A:\n    def __init__(self) -> None:\n        self.x = 1\n\nclass B:\n    def __init__(self) -> None:\n        self.y = 2\n\nclass C:\n    def __init__(self) -> None:\n        self.z = 3\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_198_class_with_complex_init() {
        let code = "class Matrix:\n    def __init__(self, rows: int, cols: int) -> None:\n        self.rows = rows\n        self.cols = cols\n        self.size = rows * cols\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ic_199_class_staticmethod_and_classmethod() {
        let code = "class Toolkit:\n    @staticmethod\n    def helper(x: int) -> int:\n        return x + 1\n    @classmethod\n    def build(cls) -> int:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn helper"));
        assert!(result.contains("fn build"));
    }

    #[test]
    fn test_w21ic_200_class_with_all_features_combined() {
        let code = "from typing import Optional\n\nclass Service:\n    VERSION = 1\n    def __init__(self, name: str, port: int = 8080) -> None:\n        self.name = name\n        self.port = port\n        self.running = False\n    def start(self) -> None:\n        self.running = True\n    def stop(self) -> None:\n        self.running = False\n    @property\n    def address(self) -> str:\n        return self.name\n    @staticmethod\n    def default_port() -> int:\n        return 8080\n    def is_running(self) -> bool:\n        return self.running\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Service"));
        assert!(result.contains("fn start"));
        assert!(result.contains("fn stop"));
    }
}
