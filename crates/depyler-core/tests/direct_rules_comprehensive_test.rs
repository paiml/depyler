//! Comprehensive tests for direct_rules.rs to increase coverage

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile_contains(code: &str, needle: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    if let Ok(result) = pipeline.transpile(code) {
        result.contains(needle)
    } else {
        false
    }
}

// ============ String methods ============

#[test]
fn test_str_upper() {
    assert!(transpile_ok("def f(s: str) -> str: return s.upper()"));
}

#[test]
fn test_str_lower() {
    assert!(transpile_ok("def f(s: str) -> str: return s.lower()"));
}

#[test]
fn test_str_strip() {
    assert!(transpile_ok("def f(s: str) -> str: return s.strip()"));
}

#[test]
fn test_str_lstrip() {
    assert!(transpile_ok("def f(s: str) -> str: return s.lstrip()"));
}

#[test]
fn test_str_rstrip() {
    assert!(transpile_ok("def f(s: str) -> str: return s.rstrip()"));
}

#[test]
fn test_str_split_no_arg() {
    assert!(transpile_ok("def f(s: str) -> list: return s.split()"));
}

#[test]
fn test_str_split_with_arg() {
    assert!(transpile_ok("def f(s: str) -> list: return s.split(',')"));
}

#[test]
fn test_str_join() {
    assert!(transpile_ok("def f(items: list) -> str: return ','.join(items)"));
}

#[test]
fn test_str_replace() {
    assert!(transpile_ok("def f(s: str) -> str: return s.replace('a', 'b')"));
}

#[test]
fn test_str_find() {
    assert!(transpile_ok("def f(s: str) -> int: return s.find('x')"));
}

#[test]
fn test_str_rfind() {
    assert!(transpile_ok("def f(s: str) -> int: return s.rfind('x')"));
}

#[test]
fn test_str_startswith() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.startswith('x')"));
}

#[test]
fn test_str_endswith() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.endswith('x')"));
}

#[test]
fn test_str_count() {
    assert!(transpile_ok("def f(s: str) -> int: return s.count('x')"));
}

#[test]
fn test_str_isalpha() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.isalpha()"));
}

#[test]
fn test_str_isdigit() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.isdigit()"));
}

#[test]
fn test_str_isalnum() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.isalnum()"));
}

#[test]
fn test_str_isspace() {
    assert!(transpile_ok("def f(s: str) -> bool: return s.isspace()"));
}

#[test]
fn test_str_capitalize() {
    assert!(transpile_ok("def f(s: str) -> str: return s.capitalize()"));
}

#[test]
fn test_str_title() {
    assert!(transpile_ok("def f(s: str) -> str: return s.title()"));
}

#[test]
fn test_str_center() {
    assert!(transpile_ok("def f(s: str) -> str: return s.center(10)"));
}

#[test]
fn test_str_ljust() {
    assert!(transpile_ok("def f(s: str) -> str: return s.ljust(10)"));
}

#[test]
fn test_str_rjust() {
    assert!(transpile_ok("def f(s: str) -> str: return s.rjust(10)"));
}

#[test]
fn test_str_zfill() {
    assert!(transpile_ok("def f(s: str) -> str: return s.zfill(5)"));
}

#[test]
fn test_str_encode() {
    assert!(transpile_ok("def f(s: str) -> bytes: return s.encode()"));
}

#[test]
fn test_str_format_method() {
    assert!(transpile_ok("def f() -> str: return '{}'.format(42)"));
}

// ============ List methods ============

#[test]
fn test_list_append() {
    assert!(transpile_ok("def f(lst: list):\n    lst.append(1)"));
}

#[test]
fn test_list_extend() {
    assert!(transpile_ok("def f(lst: list):\n    lst.extend([1, 2])"));
}

#[test]
fn test_list_insert() {
    assert!(transpile_ok("def f(lst: list):\n    lst.insert(0, 1)"));
}

#[test]
fn test_list_remove() {
    assert!(transpile_ok("def f(lst: list):\n    lst.remove(1)"));
}

#[test]
fn test_list_pop() {
    assert!(transpile_ok("def f(lst: list): return lst.pop()"));
}

#[test]
fn test_list_pop_index() {
    assert!(transpile_ok("def f(lst: list): return lst.pop(0)"));
}

#[test]
fn test_list_clear() {
    assert!(transpile_ok("def f(lst: list):\n    lst.clear()"));
}

#[test]
fn test_list_index() {
    assert!(transpile_ok("def f(lst: list) -> int: return lst.index(1)"));
}

#[test]
fn test_list_count() {
    assert!(transpile_ok("def f(lst: list) -> int: return lst.count(1)"));
}

#[test]
fn test_list_sort() {
    assert!(transpile_ok("def f(lst: list):\n    lst.sort()"));
}

#[test]
fn test_list_sort_reverse() {
    assert!(transpile_ok("def f(lst: list):\n    lst.sort(reverse=True)"));
}

#[test]
fn test_list_reverse() {
    assert!(transpile_ok("def f(lst: list):\n    lst.reverse()"));
}

#[test]
fn test_list_copy() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst.copy()"));
}

// ============ Dict methods ============

#[test]
fn test_dict_get() {
    assert!(transpile_ok("def f(d: dict): return d.get('key')"));
}

#[test]
fn test_dict_get_default() {
    assert!(transpile_ok("def f(d: dict): return d.get('key', 0)"));
}

#[test]
fn test_dict_keys() {
    assert!(transpile_ok("def f(d: dict): return d.keys()"));
}

#[test]
fn test_dict_values() {
    assert!(transpile_ok("def f(d: dict): return d.values()"));
}

#[test]
fn test_dict_items() {
    assert!(transpile_ok("def f(d: dict): return d.items()"));
}

#[test]
fn test_dict_pop() {
    assert!(transpile_ok("def f(d: dict): return d.pop('key')"));
}

#[test]
fn test_dict_pop_default() {
    assert!(transpile_ok("def f(d: dict): return d.pop('key', None)"));
}

#[test]
fn test_dict_update() {
    assert!(transpile_ok("def f(d: dict):\n    d.update({'a': 1})"));
}

#[test]
fn test_dict_clear() {
    assert!(transpile_ok("def f(d: dict):\n    d.clear()"));
}

#[test]
fn test_dict_copy() {
    assert!(transpile_ok("def f(d: dict) -> dict: return d.copy()"));
}

#[test]
fn test_dict_setdefault() {
    assert!(transpile_ok("def f(d: dict): return d.setdefault('key', 0)"));
}

// ============ Set methods ============

#[test]
fn test_set_add() {
    assert!(transpile_ok("def f(s: set):\n    s.add(1)"));
}

#[test]
fn test_set_remove() {
    assert!(transpile_ok("def f(s: set):\n    s.remove(1)"));
}

#[test]
fn test_set_discard() {
    assert!(transpile_ok("def f(s: set):\n    s.discard(1)"));
}

#[test]
fn test_set_pop() {
    assert!(transpile_ok("def f(s: set): return s.pop()"));
}

#[test]
fn test_set_clear() {
    assert!(transpile_ok("def f(s: set):\n    s.clear()"));
}

#[test]
fn test_set_union() {
    assert!(transpile_ok("def f(a: set, b: set) -> set: return a.union(b)"));
}

#[test]
fn test_set_intersection() {
    assert!(transpile_ok("def f(a: set, b: set) -> set: return a.intersection(b)"));
}

#[test]
fn test_set_difference() {
    assert!(transpile_ok("def f(a: set, b: set) -> set: return a.difference(b)"));
}

#[test]
fn test_set_symmetric_difference() {
    assert!(transpile_ok("def f(a: set, b: set) -> set: return a.symmetric_difference(b)"));
}

#[test]
fn test_set_issubset() {
    assert!(transpile_ok("def f(a: set, b: set) -> bool: return a.issubset(b)"));
}

#[test]
fn test_set_issuperset() {
    assert!(transpile_ok("def f(a: set, b: set) -> bool: return a.issuperset(b)"));
}

#[test]
fn test_set_isdisjoint() {
    assert!(transpile_ok("def f(a: set, b: set) -> bool: return a.isdisjoint(b)"));
}

// ============ Bytes methods ============

#[test]
fn test_bytes_decode() {
    assert!(transpile_ok("def f(b: bytes) -> str: return b.decode()"));
}

// ============ File methods ============

#[test]
fn test_file_read() {
    assert!(transpile_ok("def f(fp): return fp.read()"));
}

#[test]
fn test_file_readline() {
    assert!(transpile_ok("def f(fp): return fp.readline()"));
}

#[test]
fn test_file_readlines() {
    assert!(transpile_ok("def f(fp): return fp.readlines()"));
}

#[test]
fn test_file_write() {
    assert!(transpile_ok("def f(fp, data: str): fp.write(data)"));
}

#[test]
fn test_file_close() {
    assert!(transpile_ok("def f(fp): fp.close()"));
}

// ============ Math module ============

#[test]
fn test_math_sqrt() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.sqrt(x)"));
}

#[test]
fn test_math_sin() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.sin(x)"));
}

#[test]
fn test_math_cos() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.cos(x)"));
}

#[test]
fn test_math_tan() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.tan(x)"));
}

#[test]
fn test_math_floor() {
    assert!(transpile_ok("import math\ndef f(x: float) -> int: return math.floor(x)"));
}

#[test]
fn test_math_ceil() {
    assert!(transpile_ok("import math\ndef f(x: float) -> int: return math.ceil(x)"));
}

#[test]
fn test_math_log() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.log(x)"));
}

#[test]
fn test_math_log10() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.log10(x)"));
}

#[test]
fn test_math_exp() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.exp(x)"));
}

#[test]
fn test_math_pow() {
    assert!(transpile_ok("import math\ndef f(x: float, y: float) -> float: return math.pow(x, y)"));
}

#[test]
fn test_math_fabs() {
    assert!(transpile_ok("import math\ndef f(x: float) -> float: return math.fabs(x)"));
}

// ============ Random module ============

#[test]
fn test_random_random() {
    assert!(transpile_ok("import random\ndef f() -> float: return random.random()"));
}

#[test]
fn test_random_randint() {
    assert!(transpile_ok("import random\ndef f() -> int: return random.randint(1, 10)"));
}

#[test]
fn test_random_choice() {
    assert!(transpile_ok("import random\ndef f(lst: list): return random.choice(lst)"));
}

#[test]
fn test_random_shuffle() {
    assert!(transpile_ok("import random\ndef f(lst: list): random.shuffle(lst)"));
}

// ============ Collections module ============

#[test]
fn test_collections_counter() {
    assert!(transpile_ok("from collections import Counter\ndef f(items: list): return Counter(items)"));
}

#[test]
fn test_collections_deque() {
    assert!(transpile_ok("from collections import deque\ndef f(): return deque([1, 2, 3])"));
}

#[test]
fn test_collections_defaultdict() {
    assert!(transpile_ok("from collections import defaultdict\ndef f(): return defaultdict(int)"));
}

// ============ Itertools module ============

#[test]
fn test_itertools_chain() {
    assert!(transpile_ok("import itertools\ndef f(a: list, b: list): return list(itertools.chain(a, b))"));
}

#[test]
fn test_itertools_repeat() {
    assert!(transpile_ok("import itertools\ndef f(): return list(itertools.repeat(1, 5))"));
}

// ============ Functools module ============

#[test]
fn test_functools_reduce() {
    assert!(transpile_ok("from functools import reduce\ndef f(lst: list) -> int: return reduce(lambda x, y: x + y, lst)"));
}

// ============ Datetime module ============

#[test]
fn test_datetime_now() {
    assert!(transpile_ok("from datetime import datetime\ndef f(): return datetime.now()"));
}

#[test]
fn test_datetime_strftime() {
    assert!(transpile_ok("from datetime import datetime\ndef f(dt): return dt.strftime('%Y-%m-%d')"));
}

// ============ Pathlib module ============

#[test]
fn test_pathlib_path() {
    assert!(transpile_ok("from pathlib import Path\ndef f() -> Path: return Path('.')"));
}

#[test]
fn test_pathlib_exists() {
    assert!(transpile_ok("from pathlib import Path\ndef f(p: Path) -> bool: return p.exists()"));
}

#[test]
fn test_pathlib_is_file() {
    assert!(transpile_ok("from pathlib import Path\ndef f(p: Path) -> bool: return p.is_file()"));
}

#[test]
fn test_pathlib_is_dir() {
    assert!(transpile_ok("from pathlib import Path\ndef f(p: Path) -> bool: return p.is_dir()"));
}

// ============ OS module ============

#[test]
fn test_os_getcwd() {
    assert!(transpile_ok("import os\ndef f() -> str: return os.getcwd()"));
}

#[test]
fn test_os_listdir() {
    assert!(transpile_ok("import os\ndef f(p: str) -> list: return os.listdir(p)"));
}

#[test]
fn test_os_getenv() {
    assert!(transpile_ok("import os\ndef f() -> str: return os.getenv('HOME', '')"));
}

#[test]
fn test_os_path_join() {
    assert!(transpile_ok("import os\ndef f(a: str, b: str) -> str: return os.path.join(a, b)"));
}

#[test]
fn test_os_path_exists() {
    assert!(transpile_ok("import os\ndef f(p: str) -> bool: return os.path.exists(p)"));
}

#[test]
fn test_os_path_dirname() {
    assert!(transpile_ok("import os\ndef f(p: str) -> str: return os.path.dirname(p)"));
}

#[test]
fn test_os_path_basename() {
    assert!(transpile_ok("import os\ndef f(p: str) -> str: return os.path.basename(p)"));
}

// ============ Sys module ============

#[test]
fn test_sys_argv() {
    assert!(transpile_ok("import sys\ndef f() -> list: return sys.argv"));
}

#[test]
fn test_sys_exit() {
    assert!(transpile_ok("import sys\ndef f(): sys.exit(0)"));
}

// ============ JSON module ============

#[test]
fn test_json_dumps() {
    assert!(transpile_ok("import json\ndef f(obj) -> str: return json.dumps(obj)"));
}

#[test]
fn test_json_loads() {
    assert!(transpile_ok("import json\ndef f(s: str): return json.loads(s)"));
}

// ============ Regex module ============

#[test]
fn test_re_search() {
    assert!(transpile_ok("import re\ndef f(p: str, s: str): return re.search(p, s)"));
}

#[test]
fn test_re_match() {
    assert!(transpile_ok("import re\ndef f(p: str, s: str): return re.match(p, s)"));
}

#[test]
fn test_re_findall() {
    assert!(transpile_ok("import re\ndef f(p: str, s: str) -> list: return re.findall(p, s)"));
}

#[test]
fn test_re_sub() {
    assert!(transpile_ok("import re\ndef f(p: str, r: str, s: str) -> str: return re.sub(p, r, s)"));
}

// ============ Hashlib module ============

#[test]
fn test_hashlib_md5() {
    assert!(transpile_ok("import hashlib\ndef f(): return hashlib.md5()"));
}

#[test]
fn test_hashlib_sha256() {
    assert!(transpile_ok("import hashlib\ndef f(): return hashlib.sha256()"));
}

// ============ Base64 module ============

#[test]
fn test_base64_b64encode() {
    assert!(transpile_ok("import base64\ndef f(data: bytes) -> bytes: return base64.b64encode(data)"));
}

#[test]
fn test_base64_b64decode() {
    assert!(transpile_ok("import base64\ndef f(data: bytes) -> bytes: return base64.b64decode(data)"));
}

// ============ Builtins ============

#[test]
fn test_len() {
    assert!(transpile_ok("def f(x: list) -> int: return len(x)"));
}

#[test]
fn test_range_one_arg() {
    assert!(transpile_ok("def f(n: int): return range(n)"));
}

#[test]
fn test_range_two_args() {
    assert!(transpile_ok("def f(a: int, b: int): return range(a, b)"));
}

#[test]
fn test_range_three_args() {
    assert!(transpile_ok("def f(a: int, b: int, c: int): return range(a, b, c)"));
}

#[test]
fn test_print() {
    assert!(transpile_ok("def f(): print('hello')"));
}

#[test]
fn test_print_multiple() {
    assert!(transpile_ok("def f(): print('a', 'b', 'c')"));
}

#[test]
fn test_input() {
    assert!(transpile_ok("def f() -> str: return input()"));
}

#[test]
fn test_input_prompt() {
    assert!(transpile_ok("def f() -> str: return input('prompt: ')"));
}

#[test]
fn test_int_conversion() {
    assert!(transpile_ok("def f(s: str) -> int: return int(s)"));
}

#[test]
fn test_float_conversion() {
    assert!(transpile_ok("def f(s: str) -> float: return float(s)"));
}

#[test]
fn test_str_conversion() {
    assert!(transpile_ok("def f(n: int) -> str: return str(n)"));
}

#[test]
fn test_bool_conversion() {
    assert!(transpile_ok("def f(x) -> bool: return bool(x)"));
}

#[test]
fn test_list_constructor() {
    assert!(transpile_ok("def f(x) -> list: return list(x)"));
}

#[test]
fn test_tuple_constructor() {
    assert!(transpile_ok("def f(x): return tuple(x)"));
}

#[test]
fn test_set_constructor() {
    assert!(transpile_ok("def f(x) -> set: return set(x)"));
}

#[test]
fn test_dict_constructor() {
    assert!(transpile_ok("def f() -> dict: return dict()"));
}

// ============ Comparison operations ============

#[test]
fn test_in_list() {
    assert!(transpile_ok("def f(x: int, lst: list) -> bool: return x in lst"));
}

#[test]
fn test_in_dict() {
    assert!(transpile_ok("def f(k: str, d: dict) -> bool: return k in d"));
}

#[test]
fn test_in_set() {
    assert!(transpile_ok("def f(x: int, s: set) -> bool: return x in s"));
}

#[test]
fn test_in_str() {
    assert!(transpile_ok("def f(sub: str, s: str) -> bool: return sub in s"));
}

#[test]
fn test_not_in() {
    assert!(transpile_ok("def f(x: int, lst: list) -> bool: return x not in lst"));
}

#[test]
fn test_is_none() {
    assert!(transpile_ok("def f(x) -> bool: return x is None"));
}

#[test]
fn test_is_not_none() {
    assert!(transpile_ok("def f(x) -> bool: return x is not None"));
}

// ============ Slice operations ============

#[test]
fn test_slice_start() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst[1:]"));
}

#[test]
fn test_slice_stop() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst[:5]"));
}

#[test]
fn test_slice_both() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst[1:5]"));
}

#[test]
fn test_slice_step() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst[::2]"));
}

#[test]
fn test_slice_negative() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst[-3:]"));
}

#[test]
fn test_slice_reverse() {
    assert!(transpile_ok("def f(lst: list) -> list: return lst[::-1]"));
}

// ============ F-strings ============

#[test]
fn test_fstring_simple() {
    assert!(transpile_ok("def f(name: str) -> str: return f'Hello, {name}!'"));
}

#[test]
fn test_fstring_expr() {
    assert!(transpile_ok("def f(x: int) -> str: return f'Result: {x * 2}'"));
}

#[test]
fn test_fstring_format() {
    assert!(transpile_ok("def f(x: float) -> str: return f'{x:.2f}'"));
}

// ============ Type checking ============

#[test]
fn test_isinstance_int() {
    assert!(transpile_ok("def f(x) -> bool: return isinstance(x, int)"));
}

#[test]
fn test_isinstance_str() {
    assert!(transpile_ok("def f(x) -> bool: return isinstance(x, str)"));
}

#[test]
fn test_isinstance_list() {
    assert!(transpile_ok("def f(x) -> bool: return isinstance(x, list)"));
}
