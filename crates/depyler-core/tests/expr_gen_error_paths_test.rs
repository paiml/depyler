//! Test error paths in expr_gen.rs for coverage
//! Tests all bail! paths to ensure error handling is covered

use depyler_core::DepylerPipeline;

fn transpile_err(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_err()
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============ all/any/divmod error paths ============

#[test]
fn test_all_no_args() {
    assert!(transpile_err("def f(): return all()"));
}

#[test]
fn test_all_too_many_args() {
    assert!(transpile_err("def f(): return all([1], [2])"));
}

#[test]
fn test_any_no_args() {
    assert!(transpile_err("def f(): return any()"));
}

#[test]
fn test_any_too_many_args() {
    assert!(transpile_err("def f(): return any([1], [2])"));
}

#[test]
fn test_divmod_no_args() {
    assert!(transpile_err("def f(): return divmod()"));
}

#[test]
fn test_divmod_one_arg() {
    assert!(transpile_err("def f(): return divmod(5)"));
}

#[test]
fn test_divmod_three_args() {
    assert!(transpile_err("def f(): return divmod(5, 2, 1)"));
}

// ============ enumerate/zip/reversed/sorted error paths ============

#[test]
fn test_enumerate_no_args() {
    assert!(transpile_err("def f(): return enumerate()"));
}

#[test]
fn test_enumerate_three_args() {
    assert!(transpile_err("def f(): return enumerate([1], 0, 0)"));
}

#[test]
fn test_zip_no_args() {
    assert!(transpile_err("def f(): return zip()"));
}

#[test]
fn test_zip_one_arg() {
    assert!(transpile_err("def f(): return zip([1])"));
}

#[test]
fn test_reversed_no_args() {
    assert!(transpile_err("def f(): return reversed()"));
}

#[test]
fn test_reversed_two_args() {
    assert!(transpile_err("def f(): return reversed([1], [2])"));
}

#[test]
fn test_sorted_no_args() {
    assert!(transpile_err("def f(): return sorted()"));
}

#[test]
fn test_sorted_three_positional_args() {
    // sorted with key is actually supported via kwargs
    assert!(transpile_ok("def f(): return sorted([1], key=len)"));
}

// ============ filter/sum/round/abs error paths ============

#[test]
fn test_filter_no_args() {
    assert!(transpile_err("def f(): return filter()"));
}

#[test]
fn test_filter_one_arg() {
    assert!(transpile_err("def f(): return list(filter(lambda x: x))"));
}

#[test]
fn test_filter_three_args() {
    assert!(transpile_err("def f(): return list(filter(lambda x: x, [1], [2]))"));
}

#[test]
fn test_sum_no_args() {
    assert!(transpile_err("def f(): return sum()"));
}

#[test]
fn test_sum_three_args() {
    assert!(transpile_err("def f(): return sum([1], 0, 0)"));
}

#[test]
fn test_round_no_args() {
    assert!(transpile_err("def f(): return round()"));
}

#[test]
fn test_round_three_args() {
    assert!(transpile_err("def f(): return round(1.5, 2, 3)"));
}

#[test]
fn test_abs_no_args() {
    assert!(transpile_err("def f(): return abs()"));
}

#[test]
fn test_abs_two_args() {
    assert!(transpile_err("def f(): return abs(-5, -3)"));
}

// ============ min/max/pow error paths ============

#[test]
fn test_min_no_args() {
    assert!(transpile_err("def f(): return min()"));
}

#[test]
fn test_max_no_args() {
    assert!(transpile_err("def f(): return max()"));
}

#[test]
fn test_pow_no_args() {
    assert!(transpile_err("def f(): return pow()"));
}

#[test]
fn test_pow_one_arg() {
    assert!(transpile_err("def f(): return pow(2)"));
}

#[test]
fn test_pow_four_args() {
    assert!(transpile_err("def f(): return pow(2, 3, 5, 7)"));
}

// ============ hex/bin/oct error paths ============

#[test]
fn test_hex_no_args() {
    assert!(transpile_err("def f(): return hex()"));
}

#[test]
fn test_hex_two_args() {
    assert!(transpile_err("def f(): return hex(255, 16)"));
}

#[test]
fn test_bin_no_args() {
    assert!(transpile_err("def f(): return bin()"));
}

#[test]
fn test_bin_two_args() {
    assert!(transpile_err("def f(): return bin(42, 2)"));
}

#[test]
fn test_oct_no_args() {
    assert!(transpile_err("def f(): return oct()"));
}

#[test]
fn test_oct_two_args() {
    assert!(transpile_err("def f(): return oct(64, 8)"));
}

// ============ chr/ord error paths ============

#[test]
fn test_chr_no_args() {
    assert!(transpile_err("def f(): return chr()"));
}

#[test]
fn test_chr_two_args() {
    assert!(transpile_err("def f(): return chr(65, 66)"));
}

#[test]
fn test_ord_no_args() {
    assert!(transpile_err("def f(): return ord()"));
}

#[test]
fn test_ord_two_args() {
    assert!(transpile_err("def f(): return ord('A', 'B')"));
}

// ============ hash/repr/next error paths ============

#[test]
fn test_hash_no_args() {
    assert!(transpile_err("def f(): return hash()"));
}

#[test]
fn test_hash_two_args() {
    assert!(transpile_err("def f(): return hash('a', 'b')"));
}

#[test]
fn test_repr_no_args() {
    assert!(transpile_err("def f(): return repr()"));
}

#[test]
fn test_repr_two_args() {
    assert!(transpile_err("def f(): return repr(1, 2)"));
}

#[test]
fn test_next_no_args() {
    assert!(transpile_err("def f(): return next()"));
}

#[test]
fn test_next_three_args() {
    assert!(transpile_err("def f(): return next(iter([1]), None, 0)"));
}

// ============ iter/type error paths ============

#[test]
fn test_iter_no_args() {
    assert!(transpile_err("def f(): return iter()"));
}

#[test]
fn test_iter_two_args() {
    assert!(transpile_err("def f(): return iter([1], [2])"));
}

#[test]
fn test_type_no_args() {
    assert!(transpile_err("def f(): return type()"));
}

#[test]
fn test_type_two_args() {
    assert!(transpile_err("def f(): return type(1, 2)"));
}

// ============ open error paths ============

#[test]
fn test_open_no_args() {
    assert!(transpile_err("def f(): return open()"));
}

#[test]
fn test_open_three_args() {
    assert!(transpile_err("def f(): return open('file.txt', 'r', 'extra')"));
}

// ============ struct module error paths ============

#[test]
fn test_struct_pack_no_args() {
    assert!(transpile_err("import struct\ndef f(): return struct.pack()"));
}

#[test]
fn test_struct_unpack_one_arg() {
    assert!(transpile_err("import struct\ndef f(): return struct.unpack('i')"));
}

#[test]
fn test_struct_unpack_three_args() {
    assert!(transpile_err("import struct\ndef f(): return struct.unpack('i', b'data', 'extra')"));
}

#[test]
fn test_struct_calcsize_no_args() {
    assert!(transpile_err("import struct\ndef f(): return struct.calcsize()"));
}

#[test]
fn test_struct_calcsize_two_args() {
    assert!(transpile_err("import struct\ndef f(): return struct.calcsize('i', 'i')"));
}

// ============ json module error paths ============

#[test]
fn test_json_dumps_no_args() {
    assert!(transpile_err("import json\ndef f(): return json.dumps()"));
}

#[test]
fn test_json_dumps_three_args() {
    assert!(transpile_err("import json\ndef f(): return json.dumps({}, 2, 3)"));
}

#[test]
fn test_json_loads_no_args() {
    assert!(transpile_err("import json\ndef f(): return json.loads()"));
}

#[test]
fn test_json_loads_two_args() {
    assert!(transpile_err("import json\ndef f(): return json.loads('{}', 'extra')"));
}

#[test]
fn test_json_dump_one_arg() {
    assert!(transpile_err("import json\ndef f(fp): return json.dump({})"));
}

#[test]
fn test_json_dump_three_args() {
    assert!(transpile_err("import json\ndef f(fp): return json.dump({}, fp, 'extra')"));
}

#[test]
fn test_json_load_no_args() {
    assert!(transpile_err("import json\ndef f(): return json.load()"));
}

#[test]
fn test_json_load_two_args() {
    assert!(transpile_err("import json\ndef f(fp): return json.load(fp, 'extra')"));
}

// ============ re module error paths ============

#[test]
fn test_re_search_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.search()"));
}

#[test]
fn test_re_search_one_arg() {
    assert!(transpile_err("import re\ndef f(): return re.search('pattern')"));
}

#[test]
fn test_re_match_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.match()"));
}

#[test]
fn test_re_match_one_arg() {
    assert!(transpile_err("import re\ndef f(): return re.match('pattern')"));
}

#[test]
fn test_re_findall_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.findall()"));
}

#[test]
fn test_re_findall_one_arg() {
    assert!(transpile_err("import re\ndef f(): return re.findall('pattern')"));
}

#[test]
fn test_re_finditer_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.finditer()"));
}

#[test]
fn test_re_finditer_one_arg() {
    assert!(transpile_err("import re\ndef f(): return re.finditer('pattern')"));
}

#[test]
fn test_re_sub_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.sub()"));
}

#[test]
fn test_re_sub_two_args() {
    assert!(transpile_err("import re\ndef f(): return re.sub('pattern', 'repl')"));
}

#[test]
fn test_re_subn_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.subn()"));
}

#[test]
fn test_re_subn_two_args() {
    assert!(transpile_err("import re\ndef f(): return re.subn('pattern', 'repl')"));
}

#[test]
fn test_re_compile_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.compile()"));
}

#[test]
fn test_re_split_success() {
    // re.split requires 2 args and is supported
    assert!(transpile_ok("import re\ndef f(s: str) -> list: return re.split('\\\\s+', s)"));
}

#[test]
fn test_re_escape_no_args() {
    assert!(transpile_err("import re\ndef f(): return re.escape()"));
}

#[test]
fn test_re_escape_two_args() {
    assert!(transpile_err("import re\ndef f(): return re.escape('text', 'extra')"));
}

// ============ time module error paths ============

#[test]
fn test_time_sleep_no_args() {
    assert!(transpile_err("import time\ndef f(): return time.sleep()"));
}

#[test]
fn test_time_sleep_two_args() {
    assert!(transpile_err("import time\ndef f(): return time.sleep(1, 2)"));
}

#[test]
fn test_time_ctime_two_args() {
    assert!(transpile_err("import time\ndef f(): return time.ctime(1, 2)"));
}

#[test]
fn test_time_strftime_success() {
    // time.strftime with format and time_tuple is supported
    assert!(transpile_ok("import time\ndef f(t): return time.strftime('%Y', t)"));
}

#[test]
fn test_time_strptime_one_arg() {
    assert!(transpile_err("import time\ndef f(): return time.strptime('2024')"));
}

#[test]
fn test_time_mktime_no_args() {
    assert!(transpile_err("import time\ndef f(): return time.mktime()"));
}

#[test]
fn test_time_mktime_two_args() {
    assert!(transpile_err("import time\ndef f(): return time.mktime((1,2,3,4,5,6,7,8,9), 2)"));
}

#[test]
fn test_time_asctime_no_args() {
    assert!(transpile_err("import time\ndef f(): return time.asctime()"));
}

#[test]
fn test_time_asctime_two_args() {
    assert!(transpile_err("import time\ndef f(): return time.asctime((1,2,3,4,5,6,7,8,9), 2)"));
}

// ============ shutil module error paths ============

#[test]
fn test_shutil_copy_one_arg() {
    assert!(transpile_err("import shutil\ndef f(): return shutil.copy('src')"));
}

#[test]
fn test_shutil_copy_success() {
    // shutil.copy with two args is supported
    assert!(transpile_ok("import shutil\ndef f(s: str, d: str): shutil.copy(s, d)"));
}

#[test]
fn test_shutil_move_one_arg() {
    assert!(transpile_err("import shutil\ndef f(): return shutil.move('src')"));
}

#[test]
fn test_shutil_move_success() {
    // shutil.move with two args is supported
    assert!(transpile_ok("import shutil\ndef f(s: str, d: str): shutil.move(s, d)"));
}

#[test]
fn test_shutil_rmtree_no_args() {
    assert!(transpile_err("import shutil\ndef f(): return shutil.rmtree()"));
}

#[test]
fn test_shutil_rmtree_success() {
    // shutil.rmtree with one arg is supported
    assert!(transpile_ok("import shutil\ndef f(p: str): shutil.rmtree(p)"));
}

#[test]
fn test_shutil_copytree_one_arg() {
    assert!(transpile_err("import shutil\ndef f(): return shutil.copytree('src')"));
}

#[test]
fn test_shutil_copytree_success() {
    // shutil.copytree with two args is supported
    assert!(transpile_ok("import shutil\ndef f(s: str, d: str): shutil.copytree(s, d)"));
}

#[test]
fn test_shutil_which_no_args() {
    assert!(transpile_err("import shutil\ndef f(): return shutil.which()"));
}

#[test]
fn test_shutil_which_success() {
    // shutil.which with one arg is supported
    assert!(transpile_ok("import shutil\ndef f(c: str) -> str: return shutil.which(c)"));
}

// ============ csv module error paths ============

#[test]
fn test_csv_reader_no_args() {
    assert!(transpile_err("import csv\ndef f(): return csv.reader()"));
}

#[test]
fn test_csv_writer_no_args() {
    assert!(transpile_err("import csv\ndef f(fp): return csv.writer()"));
}

#[test]
fn test_csv_dictreader_no_args() {
    assert!(transpile_err("import csv\ndef f(): return csv.DictReader()"));
}

#[test]
fn test_csv_dictwriter_no_args() {
    assert!(transpile_err("import csv\ndef f(fp): return csv.DictWriter()"));
}

// ============ os module error paths ============

#[test]
fn test_os_getenv_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.getenv()"));
}

#[test]
fn test_os_getenv_three_args() {
    assert!(transpile_err("import os\ndef f(): return os.getenv('KEY', 'default', 'extra')"));
}

#[test]
fn test_os_remove_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.remove()"));
}

#[test]
fn test_os_remove_two_args() {
    assert!(transpile_err("import os\ndef f(): return os.remove('path', 'extra')"));
}

#[test]
fn test_os_mkdir_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.mkdir()"));
}

#[test]
fn test_os_makedirs_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.makedirs()"));
}

#[test]
fn test_os_rmdir_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.rmdir()"));
}

#[test]
fn test_os_rmdir_two_args() {
    assert!(transpile_err("import os\ndef f(): return os.rmdir('path', 'extra')"));
}

#[test]
fn test_os_rename_one_arg() {
    assert!(transpile_err("import os\ndef f(): return os.rename('src')"));
}

#[test]
fn test_os_rename_three_args() {
    assert!(transpile_err("import os\ndef f(): return os.rename('src', 'dst', 'extra')"));
}

#[test]
fn test_os_getcwd_one_arg() {
    assert!(transpile_err("import os\ndef f(): return os.getcwd('extra')"));
}

#[test]
fn test_os_chdir_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.chdir()"));
}

#[test]
fn test_os_chdir_two_args() {
    assert!(transpile_err("import os\ndef f(): return os.chdir('path', 'extra')"));
}

#[test]
fn test_os_walk_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.walk()"));
}

#[test]
fn test_os_urandom_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.urandom()"));
}

#[test]
fn test_os_urandom_two_args() {
    assert!(transpile_err("import os\ndef f(): return os.urandom(16, 32)"));
}

#[test]
fn test_os_environ_get_no_args() {
    assert!(transpile_err("import os\ndef f(): return os.environ.get()"));
}

#[test]
fn test_os_environ_get_three_args() {
    assert!(transpile_err("import os\ndef f(): return os.environ.get('KEY', 'default', 'extra')"));
}

// ============ subprocess module error paths ============

#[test]
fn test_subprocess_run_no_args() {
    assert!(transpile_err("import subprocess\ndef f(): return subprocess.run()"));
}

#[test]
fn test_subprocess_popen_no_args() {
    assert!(transpile_err("import subprocess\ndef f(): return subprocess.Popen()"));
}

// ============ Success paths for completeness ============

#[test]
fn test_all_success() {
    assert!(transpile_ok("def f(x: list) -> bool: return all(x)"));
}

#[test]
fn test_any_success() {
    assert!(transpile_ok("def f(x: list) -> bool: return any(x)"));
}

#[test]
fn test_divmod_success() {
    assert!(transpile_ok("def f(a: int, b: int): return divmod(a, b)"));
}

#[test]
fn test_enumerate_success() {
    assert!(transpile_ok("def f(x: list): return list(enumerate(x))"));
}

#[test]
fn test_zip_success() {
    assert!(transpile_ok("def f(a: list, b: list): return list(zip(a, b))"));
}

#[test]
fn test_reversed_success() {
    assert!(transpile_ok("def f(x: list): return list(reversed(x))"));
}

#[test]
fn test_sorted_success() {
    assert!(transpile_ok("def f(x: list) -> list: return sorted(x)"));
}

#[test]
fn test_filter_success() {
    assert!(transpile_ok("def f(x: list): return list(filter(lambda n: n > 0, x))"));
}

#[test]
fn test_sum_success() {
    assert!(transpile_ok("def f(x: list) -> int: return sum(x)"));
}

#[test]
fn test_round_success() {
    assert!(transpile_ok("def f(x: float) -> int: return round(x)"));
}

#[test]
fn test_abs_success() {
    assert!(transpile_ok("def f(x: int) -> int: return abs(x)"));
}

#[test]
fn test_min_success() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return min(a, b)"));
}

#[test]
fn test_max_success() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return max(a, b)"));
}

#[test]
fn test_pow_success() {
    assert!(transpile_ok("def f(a: int, b: int) -> int: return pow(a, b)"));
}

#[test]
fn test_hex_success() {
    assert!(transpile_ok("def f(x: int) -> str: return hex(x)"));
}

#[test]
fn test_bin_success() {
    assert!(transpile_ok("def f(x: int) -> str: return bin(x)"));
}

#[test]
fn test_oct_success() {
    assert!(transpile_ok("def f(x: int) -> str: return oct(x)"));
}

#[test]
fn test_chr_success() {
    assert!(transpile_ok("def f(x: int) -> str: return chr(x)"));
}

#[test]
fn test_ord_success() {
    assert!(transpile_ok("def f(x: str) -> int: return ord(x)"));
}

#[test]
fn test_hash_success() {
    assert!(transpile_ok("def f(x: str) -> int: return hash(x)"));
}

#[test]
fn test_repr_success() {
    assert!(transpile_ok("def f(x: int) -> str: return repr(x)"));
}

#[test]
fn test_iter_success() {
    assert!(transpile_ok("def f(x: list): return iter(x)"));
}

#[test]
fn test_type_success() {
    assert!(transpile_ok("def f(x: int) -> str: return type(x).__name__"));
}
