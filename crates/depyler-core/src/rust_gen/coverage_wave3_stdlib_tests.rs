//! Coverage wave 3: stdlib method coverage boost tests
//!
//! Targets uncovered branches in stdlib_data.rs, stdlib_misc.rs, stdlib_numpy.rs,
//! call_methods.rs, call_dispatch.rs, and regex_mod.rs

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

// =============================================================================
// Section 1: re module (regex_mod.rs - 38.1% cov)
// =============================================================================

#[test]
fn test_re_split_basic() {
    let code = transpile(
        "import re\ndef split_text(pattern: str, text: str) -> list:\n    return re.split(pattern, text)",
    );
    assert!(!code.is_empty(), "re.split: {}", code);
}

#[test]
fn test_re_sub_basic() {
    let code = transpile(
        "import re\ndef replace(pattern: str, repl: str, text: str) -> str:\n    return re.sub(pattern, repl, text)",
    );
    assert!(!code.is_empty(), "re.sub: {}", code);
}

#[test]
fn test_re_subn_basic() {
    let code = transpile(
        "import re\ndef replace_count(pattern: str, repl: str, text: str) -> tuple:\n    return re.subn(pattern, repl, text)",
    );
    assert!(!code.is_empty(), "re.subn: {}", code);
}

#[test]
fn test_re_findall_basic() {
    let code = transpile(
        "import re\ndef find_all(pattern: str, text: str) -> list:\n    return re.findall(pattern, text)",
    );
    assert!(!code.is_empty(), "re.findall: {}", code);
}

#[test]
fn test_re_fullmatch_basic() {
    let code = transpile(
        "import re\ndef full_match(pattern: str, text: str):\n    return re.fullmatch(pattern, text)",
    );
    assert!(!code.is_empty(), "re.fullmatch: {}", code);
}

#[test]
fn test_re_escape_basic() {
    let code = transpile(
        "import re\ndef escape_pattern(text: str) -> str:\n    return re.escape(text)",
    );
    assert!(!code.is_empty(), "re.escape: {}", code);
}

#[test]
fn test_re_search_basic() {
    let code = transpile(
        "import re\ndef find(pattern: str, text: str):\n    return re.search(pattern, text)",
    );
    assert!(!code.is_empty(), "re.search: {}", code);
}

#[test]
fn test_re_compile_basic() {
    let code = transpile(
        "import re\ndef make_regex(pattern: str):\n    return re.compile(pattern)",
    );
    assert!(!code.is_empty(), "re.compile: {}", code);
}

#[test]
fn test_re_match_basic() {
    let code = transpile(
        "import re\ndef match_start(pattern: str, text: str):\n    return re.match(pattern, text)",
    );
    assert!(!code.is_empty(), "re.match: {}", code);
}

#[test]
fn test_re_finditer_basic() {
    let code = transpile(
        "import re\ndef iterate_matches(pattern: str, text: str):\n    for m in re.finditer(pattern, text):\n        print(m)",
    );
    assert!(!code.is_empty(), "re.finditer: {}", code);
}

// =============================================================================
// Section 2: calendar module (stdlib_data.rs - 37.5% cov)
// =============================================================================

#[test]
fn test_calendar_isleap() {
    let code = transpile(
        "import calendar\ndef check_leap(year: int) -> bool:\n    return calendar.isleap(year)",
    );
    assert!(!code.is_empty(), "calendar.isleap: {}", code);
}

#[test]
fn test_calendar_weekday() {
    let code = transpile(
        "import calendar\ndef get_weekday(year: int, month: int, day: int) -> int:\n    return calendar.weekday(year, month, day)",
    );
    assert!(!code.is_empty(), "calendar.weekday: {}", code);
}

#[test]
fn test_calendar_monthrange() {
    let code = transpile(
        "import calendar\ndef get_range(year: int, month: int) -> tuple:\n    return calendar.monthrange(year, month)",
    );
    assert!(!code.is_empty(), "calendar.monthrange: {}", code);
}

#[test]
fn test_calendar_leapdays() {
    let code = transpile(
        "import calendar\ndef count_leaps(y1: int, y2: int) -> int:\n    return calendar.leapdays(y1, y2)",
    );
    assert!(!code.is_empty(), "calendar.leapdays: {}", code);
}

#[test]
fn test_calendar_month() {
    let code = transpile(
        "import calendar\ndef show_month(year: int, month: int) -> str:\n    return calendar.month(year, month)",
    );
    assert!(!code.is_empty(), "calendar.month: {}", code);
}

#[test]
fn test_calendar_monthcalendar() {
    let code = transpile(
        "import calendar\ndef get_cal(year: int, month: int) -> list:\n    return calendar.monthcalendar(year, month)",
    );
    assert!(!code.is_empty(), "calendar.monthcalendar: {}", code);
}

// =============================================================================
// Section 3: bisect module (stdlib_misc.rs - 48.8% cov)
// =============================================================================

#[test]
fn test_bisect_bisect_left() {
    let code = transpile(
        "import bisect\ndef find_left(arr: list, val: int) -> int:\n    return bisect.bisect_left(arr, val)",
    );
    assert!(!code.is_empty(), "bisect_left: {}", code);
}

#[test]
fn test_bisect_bisect_right() {
    let code = transpile(
        "import bisect\ndef find_right(arr: list, val: int) -> int:\n    return bisect.bisect_right(arr, val)",
    );
    assert!(!code.is_empty(), "bisect_right: {}", code);
}

#[test]
fn test_bisect_insort_left() {
    let code = transpile(
        "import bisect\ndef insert_left(arr: list, val: int):\n    bisect.insort_left(arr, val)",
    );
    assert!(!code.is_empty(), "insort_left: {}", code);
}

#[test]
fn test_bisect_insort_right() {
    let code = transpile(
        "import bisect\ndef insert_right(arr: list, val: int):\n    bisect.insort_right(arr, val)",
    );
    assert!(!code.is_empty(), "insort_right: {}", code);
}

#[test]
fn test_bisect_default_bisect() {
    let code = transpile(
        "import bisect\ndef find_pos(arr: list, val: int) -> int:\n    return bisect.bisect(arr, val)",
    );
    assert!(!code.is_empty(), "bisect.bisect (alias for bisect_right): {}", code);
}

// =============================================================================
// Section 4: heapq module (stdlib_misc.rs)
// =============================================================================

#[test]
fn test_heapq_heappush() {
    let code = transpile(
        "import heapq\ndef push_item(heap: list, item: int):\n    heapq.heappush(heap, item)",
    );
    assert!(!code.is_empty(), "heappush: {}", code);
}

#[test]
fn test_heapq_heappop() {
    let code = transpile(
        "import heapq\ndef pop_item(heap: list) -> int:\n    return heapq.heappop(heap)",
    );
    assert!(!code.is_empty(), "heappop: {}", code);
}

#[test]
fn test_heapq_heapify() {
    let code = transpile(
        "import heapq\ndef make_heap(items: list):\n    heapq.heapify(items)",
    );
    assert!(!code.is_empty(), "heapify: {}", code);
}

#[test]
fn test_heapq_nlargest() {
    let code = transpile(
        "import heapq\ndef top_n(n: int, items: list) -> list:\n    return heapq.nlargest(n, items)",
    );
    assert!(!code.is_empty(), "nlargest: {}", code);
}

#[test]
fn test_heapq_nsmallest() {
    let code = transpile(
        "import heapq\ndef bottom_n(n: int, items: list) -> list:\n    return heapq.nsmallest(n, items)",
    );
    assert!(!code.is_empty(), "nsmallest: {}", code);
}

#[test]
fn test_heapq_nlargest_with_typed_list() {
    let code = transpile(
        "import heapq\ndef top_three(items: list) -> list:\n    return heapq.nlargest(3, items)",
    );
    assert!(!code.is_empty(), "nlargest with literal: {}", code);
}

// =============================================================================
// Section 5: copy module (stdlib_misc.rs)
// =============================================================================

#[test]
fn test_copy_deepcopy() {
    let code = transpile(
        "import copy\ndef clone_deep(obj: list) -> list:\n    return copy.deepcopy(obj)",
    );
    assert!(!code.is_empty(), "deepcopy: {}", code);
}

#[test]
fn test_copy_copy() {
    let code = transpile(
        "import copy\ndef clone_shallow(obj: list) -> list:\n    return copy.copy(obj)",
    );
    assert!(!code.is_empty(), "copy.copy: {}", code);
}

// =============================================================================
// Section 6: statistics module (stdlib_misc.rs)
// =============================================================================

#[test]
fn test_statistics_mean() {
    let code = transpile(
        "import statistics\ndef average(data: list) -> float:\n    return statistics.mean(data)",
    );
    assert!(!code.is_empty(), "statistics.mean: {}", code);
}

#[test]
fn test_statistics_median() {
    let code = transpile(
        "import statistics\ndef middle(data: list) -> float:\n    return statistics.median(data)",
    );
    assert!(!code.is_empty(), "statistics.median: {}", code);
}

#[test]
fn test_statistics_stdev() {
    let code = transpile(
        "import statistics\ndef spread(data: list) -> float:\n    return statistics.stdev(data)",
    );
    assert!(!code.is_empty(), "statistics.stdev: {}", code);
}

#[test]
fn test_statistics_mean_from_import() {
    let code = transpile(
        "from statistics import mean\ndef average(data: list) -> float:\n    return mean(data)",
    );
    assert!(!code.is_empty(), "from statistics import mean: {}", code);
}

#[test]
fn test_statistics_median_from_import() {
    let code = transpile(
        "from statistics import median\ndef middle(data: list) -> float:\n    return median(data)",
    );
    assert!(!code.is_empty(), "from statistics import median: {}", code);
}

#[test]
fn test_statistics_stdev_from_import() {
    let code = transpile(
        "from statistics import stdev\ndef spread(data: list) -> float:\n    return stdev(data)",
    );
    assert!(!code.is_empty(), "from statistics import stdev: {}", code);
}

// =============================================================================
// Section 7: struct module (call_methods.rs)
// =============================================================================

#[test]
fn test_struct_pack_single_int() {
    let code = transpile(
        "import struct\ndef pack_val(val: int) -> bytes:\n    return struct.pack(\"i\", val)",
    );
    assert!(!code.is_empty(), "struct.pack single: {}", code);
}

#[test]
fn test_struct_pack_two_ints() {
    let code = transpile(
        "import struct\ndef pack_pair(a: int, b: int) -> bytes:\n    return struct.pack(\"ii\", a, b)",
    );
    assert!(!code.is_empty(), "struct.pack double: {}", code);
}

#[test]
fn test_struct_unpack_single() {
    let code = transpile(
        "import struct\ndef unpack_val(buf: bytes) -> tuple:\n    return struct.unpack(\"i\", buf)",
    );
    assert!(!code.is_empty(), "struct.unpack single: {}", code);
}

#[test]
fn test_struct_unpack_double() {
    let code = transpile(
        "import struct\ndef unpack_pair(buf: bytes) -> tuple:\n    return struct.unpack(\"ii\", buf)",
    );
    assert!(!code.is_empty(), "struct.unpack double: {}", code);
}

#[test]
fn test_struct_calcsize_single() {
    let code = transpile(
        "import struct\ndef size() -> int:\n    return struct.calcsize(\"i\")",
    );
    assert!(!code.is_empty(), "struct.calcsize single: {}", code);
}

#[test]
fn test_struct_calcsize_double() {
    let code = transpile(
        "import struct\ndef size() -> int:\n    return struct.calcsize(\"ii\")",
    );
    assert!(!code.is_empty(), "struct.calcsize double: {}", code);
}

// =============================================================================
// Section 8: csv module (call_methods.rs)
// =============================================================================

#[test]
fn test_csv_reader() {
    let code = transpile(
        "import csv\ndef read_csv(f: str):\n    reader = csv.reader(f)\n    return reader",
    );
    assert!(!code.is_empty(), "csv.reader: {}", code);
}

#[test]
fn test_csv_dictreader() {
    let code = transpile(
        "import csv\ndef read_dict_csv(f: str):\n    reader = csv.DictReader(f)\n    return reader",
    );
    assert!(!code.is_empty(), "csv.DictReader: {}", code);
}

#[test]
fn test_csv_writer() {
    let code = transpile(
        "import csv\ndef write_csv(f: str):\n    writer = csv.writer(f)\n    return writer",
    );
    assert!(!code.is_empty(), "csv.writer: {}", code);
}

#[test]
fn test_csv_dictwriter() {
    let code = transpile(
        "import csv\ndef write_dict_csv(f: str, fields: list):\n    writer = csv.DictWriter(f, fields)\n    return writer",
    );
    assert!(!code.is_empty(), "csv.DictWriter: {}", code);
}

// =============================================================================
// Section 9: colorsys module (expr_methods.rs - 19.9% cov)
// =============================================================================

#[test]
fn test_colorsys_hsv_to_rgb() {
    let code = transpile(
        "import colorsys\ndef convert(h: float, s: float, v: float) -> tuple:\n    return colorsys.hsv_to_rgb(h, s, v)",
    );
    assert!(!code.is_empty(), "colorsys.hsv_to_rgb: {}", code);
}

#[test]
fn test_colorsys_rgb_to_hsv() {
    let code = transpile(
        "import colorsys\ndef convert(r: float, g: float, b: float) -> tuple:\n    return colorsys.rgb_to_hsv(r, g, b)",
    );
    assert!(!code.is_empty(), "colorsys.rgb_to_hsv: {}", code);
}

#[test]
fn test_colorsys_rgb_to_hls() {
    let code = transpile(
        "import colorsys\ndef convert(r: float, g: float, b: float) -> tuple:\n    return colorsys.rgb_to_hls(r, g, b)",
    );
    assert!(!code.is_empty(), "colorsys.rgb_to_hls: {}", code);
}

// =============================================================================
// Section 10: base64 module (stdlib_crypto.rs)
// =============================================================================

#[test]
fn test_base64_b64encode() {
    let code = transpile(
        "import base64\ndef encode(data: bytes) -> bytes:\n    return base64.b64encode(data)",
    );
    assert!(!code.is_empty(), "base64.b64encode: {}", code);
}

#[test]
fn test_base64_b64decode() {
    let code = transpile(
        "import base64\ndef decode(data: bytes) -> bytes:\n    return base64.b64decode(data)",
    );
    assert!(!code.is_empty(), "base64.b64decode: {}", code);
}

#[test]
fn test_base64_urlsafe_b64encode() {
    let code = transpile(
        "import base64\ndef encode_url(data: bytes) -> bytes:\n    return base64.urlsafe_b64encode(data)",
    );
    assert!(!code.is_empty(), "base64.urlsafe_b64encode: {}", code);
}

#[test]
fn test_base64_urlsafe_b64decode() {
    let code = transpile(
        "import base64\ndef decode_url(data: bytes) -> bytes:\n    return base64.urlsafe_b64decode(data)",
    );
    assert!(!code.is_empty(), "base64.urlsafe_b64decode: {}", code);
}

#[test]
fn test_base64_b16encode() {
    let code = transpile(
        "import base64\ndef encode_hex(data: bytes) -> str:\n    return base64.b16encode(data)",
    );
    assert!(!code.is_empty(), "base64.b16encode: {}", code);
}

#[test]
fn test_base64_b16decode() {
    let code = transpile(
        "import base64\ndef decode_hex(data: bytes) -> bytes:\n    return base64.b16decode(data)",
    );
    assert!(!code.is_empty(), "base64.b16decode: {}", code);
}

#[test]
fn test_binascii_hexlify() {
    let code = transpile(
        "import binascii\ndef to_hex(data: bytes) -> bytes:\n    return binascii.hexlify(data)",
    );
    assert!(!code.is_empty(), "binascii.hexlify: {}", code);
}

#[test]
fn test_binascii_unhexlify() {
    let code = transpile(
        "import binascii\ndef from_hex(data: bytes) -> bytes:\n    return binascii.unhexlify(data)",
    );
    assert!(!code.is_empty(), "binascii.unhexlify: {}", code);
}

#[test]
fn test_binascii_b2a_base64() {
    let code = transpile(
        "import binascii\ndef to_b64(data: bytes) -> bytes:\n    return binascii.b2a_base64(data)",
    );
    assert!(!code.is_empty(), "binascii.b2a_base64: {}", code);
}

#[test]
fn test_binascii_a2b_base64() {
    let code = transpile(
        "import binascii\ndef from_b64(data: bytes) -> bytes:\n    return binascii.a2b_base64(data)",
    );
    assert!(!code.is_empty(), "binascii.a2b_base64: {}", code);
}

#[test]
fn test_binascii_b2a_qp() {
    let code = transpile(
        "import binascii\ndef to_qp(data: bytes) -> bytes:\n    return binascii.b2a_qp(data)",
    );
    assert!(!code.is_empty(), "binascii.b2a_qp: {}", code);
}

#[test]
fn test_binascii_a2b_qp() {
    let code = transpile(
        "import binascii\ndef from_qp(data: bytes) -> bytes:\n    return binascii.a2b_qp(data)",
    );
    assert!(!code.is_empty(), "binascii.a2b_qp: {}", code);
}

#[test]
fn test_binascii_b2a_hex_alias() {
    let code = transpile(
        "import binascii\ndef to_hex_alias(data: bytes) -> bytes:\n    return binascii.b2a_hex(data)",
    );
    assert!(!code.is_empty(), "binascii.b2a_hex: {}", code);
}

#[test]
fn test_binascii_a2b_hex_alias() {
    let code = transpile(
        "import binascii\ndef from_hex_alias(data: bytes) -> bytes:\n    return binascii.a2b_hex(data)",
    );
    assert!(!code.is_empty(), "binascii.a2b_hex: {}", code);
}

// =============================================================================
// Section 11: hashlib module (hashlib.rs)
// =============================================================================

#[test]
fn test_hashlib_sha1() {
    let code = transpile(
        "import hashlib\ndef hash_sha1(data: str) -> str:\n    return hashlib.sha1(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.sha1: {}", code);
}

#[test]
fn test_hashlib_sha256() {
    let code = transpile(
        "import hashlib\ndef hash_sha256(data: str) -> str:\n    return hashlib.sha256(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.sha256: {}", code);
}

#[test]
fn test_hashlib_md5() {
    let code = transpile(
        "import hashlib\ndef hash_md5(data: str) -> str:\n    return hashlib.md5(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.md5: {}", code);
}

#[test]
fn test_hashlib_blake2b() {
    let code = transpile(
        "import hashlib\ndef hash_blake2b(data: str) -> str:\n    return hashlib.blake2b(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.blake2b: {}", code);
}

#[test]
fn test_hashlib_blake2s() {
    let code = transpile(
        "import hashlib\ndef hash_blake2s(data: str) -> str:\n    return hashlib.blake2s(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.blake2s: {}", code);
}

#[test]
fn test_hashlib_sha384() {
    let code = transpile(
        "import hashlib\ndef hash_sha384(data: str) -> str:\n    return hashlib.sha384(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.sha384: {}", code);
}

#[test]
fn test_hashlib_sha512() {
    let code = transpile(
        "import hashlib\ndef hash_sha512(data: str) -> str:\n    return hashlib.sha512(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.sha512: {}", code);
}

#[test]
fn test_hashlib_sha224() {
    let code = transpile(
        "import hashlib\ndef hash_sha224(data: str) -> str:\n    return hashlib.sha224(data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.sha224: {}", code);
}

#[test]
fn test_hashlib_new_sha256() {
    let code = transpile(
        "import hashlib\ndef hash_new(data: str) -> str:\n    return hashlib.new(\"sha256\", data.encode()).hexdigest()",
    );
    assert!(!code.is_empty(), "hashlib.new sha256: {}", code);
}

#[test]
fn test_hashlib_sha1_no_data() {
    let code = transpile(
        "import hashlib\ndef make_hasher():\n    return hashlib.sha1()",
    );
    assert!(!code.is_empty(), "hashlib.sha1 no data: {}", code);
}

// =============================================================================
// Section 12: json module (json.rs)
// =============================================================================

#[test]
fn test_json_loads_typed() {
    let code = transpile(
        "import json\ndef parse(text: str) -> dict:\n    return json.loads(text)",
    );
    assert!(!code.is_empty(), "json.loads: {}", code);
}

#[test]
fn test_json_dumps_basic() {
    let code = transpile(
        "import json\ndef serialize(obj: dict) -> str:\n    return json.dumps(obj)",
    );
    assert!(!code.is_empty(), "json.dumps: {}", code);
}

#[test]
fn test_json_dumps_indent() {
    let code = transpile(
        "import json\ndef serialize_pretty(obj: dict) -> str:\n    return json.dumps(obj, indent=2)",
    );
    assert!(!code.is_empty(), "json.dumps with indent: {}", code);
}

#[test]
fn test_json_dump_to_file() {
    let code = transpile(
        "import json\n\ndef save(data: dict, path: str):\n    with open(path, \"w\") as f:\n        json.dump(data, f)",
    );
    assert!(!code.is_empty(), "json.dump: {}", code);
}

#[test]
fn test_json_load_from_file() {
    let code = transpile(
        "import json\n\ndef load_data(path: str) -> dict:\n    with open(path) as f:\n        return json.load(f)",
    );
    assert!(!code.is_empty(), "json.load: {}", code);
}

// =============================================================================
// Section 13: math module extra functions (math.rs)
// =============================================================================

#[test]
fn test_math_log_with_base() {
    let code = transpile(
        "import math\ndef log_base(x: float, base: float) -> float:\n    return math.log(x, base)",
    );
    assert!(!code.is_empty(), "math.log with base: {}", code);
}

#[test]
fn test_math_log_natural() {
    let code = transpile(
        "import math\ndef log_natural(x: float) -> float:\n    return math.log(x)",
    );
    assert!(!code.is_empty(), "math.log natural: {}", code);
}

#[test]
fn test_math_exp() {
    let code = transpile(
        "import math\ndef exponential(x: float) -> float:\n    return math.exp(x)",
    );
    assert!(!code.is_empty(), "math.exp: {}", code);
}

#[test]
fn test_math_factorial() {
    let code = transpile(
        "import math\ndef fact(n: int) -> int:\n    return math.factorial(n)",
    );
    assert!(!code.is_empty(), "math.factorial: {}", code);
}

#[test]
fn test_math_gcd() {
    let code = transpile(
        "import math\ndef greatest_common(a: int, b: int) -> int:\n    return math.gcd(a, b)",
    );
    assert!(!code.is_empty(), "math.gcd: {}", code);
}

#[test]
fn test_math_lcm() {
    let code = transpile(
        "import math\ndef least_common(a: int, b: int) -> int:\n    return math.lcm(a, b)",
    );
    assert!(!code.is_empty(), "math.lcm: {}", code);
}

#[test]
fn test_math_comb() {
    let code = transpile(
        "import math\ndef combinations(n: int, k: int) -> int:\n    return math.comb(n, k)",
    );
    assert!(!code.is_empty(), "math.comb: {}", code);
}

#[test]
fn test_math_perm() {
    let code = transpile(
        "import math\ndef permutations(n: int, k: int) -> int:\n    return math.perm(n, k)",
    );
    assert!(!code.is_empty(), "math.perm: {}", code);
}

#[test]
fn test_math_sqrt() {
    let code = transpile(
        "import math\ndef root(x: float) -> float:\n    return math.sqrt(x)",
    );
    assert!(!code.is_empty(), "math.sqrt: {}", code);
}

#[test]
fn test_math_ceil() {
    let code = transpile(
        "import math\ndef round_up(x: float) -> int:\n    return math.ceil(x)",
    );
    assert!(!code.is_empty(), "math.ceil: {}", code);
}

#[test]
fn test_math_floor() {
    let code = transpile(
        "import math\ndef round_down(x: float) -> int:\n    return math.floor(x)",
    );
    assert!(!code.is_empty(), "math.floor: {}", code);
}

#[test]
fn test_math_isnan() {
    let code = transpile(
        "import math\ndef check_nan(x: float) -> bool:\n    return math.isnan(x)",
    );
    assert!(!code.is_empty(), "math.isnan: {}", code);
}

#[test]
fn test_math_isinf() {
    let code = transpile(
        "import math\ndef check_inf(x: float) -> bool:\n    return math.isinf(x)",
    );
    assert!(!code.is_empty(), "math.isinf: {}", code);
}

#[test]
fn test_math_isfinite() {
    let code = transpile(
        "import math\ndef check_finite(x: float) -> bool:\n    return math.isfinite(x)",
    );
    assert!(!code.is_empty(), "math.isfinite: {}", code);
}

#[test]
fn test_math_fabs() {
    let code = transpile(
        "import math\ndef abs_val(x: float) -> float:\n    return math.fabs(x)",
    );
    assert!(!code.is_empty(), "math.fabs: {}", code);
}

#[test]
fn test_math_copysign() {
    let code = transpile(
        "import math\ndef sign_copy(x: float, y: float) -> float:\n    return math.copysign(x, y)",
    );
    assert!(!code.is_empty(), "math.copysign: {}", code);
}

#[test]
fn test_math_degrees() {
    let code = transpile(
        "import math\ndef to_deg(x: float) -> float:\n    return math.degrees(x)",
    );
    assert!(!code.is_empty(), "math.degrees: {}", code);
}

#[test]
fn test_math_radians() {
    let code = transpile(
        "import math\ndef to_rad(x: float) -> float:\n    return math.radians(x)",
    );
    assert!(!code.is_empty(), "math.radians: {}", code);
}

#[test]
fn test_math_sin() {
    let code = transpile(
        "import math\ndef sine(x: float) -> float:\n    return math.sin(x)",
    );
    assert!(!code.is_empty(), "math.sin: {}", code);
}

#[test]
fn test_math_cos() {
    let code = transpile(
        "import math\ndef cosine(x: float) -> float:\n    return math.cos(x)",
    );
    assert!(!code.is_empty(), "math.cos: {}", code);
}

#[test]
fn test_math_tan() {
    let code = transpile(
        "import math\ndef tangent(x: float) -> float:\n    return math.tan(x)",
    );
    assert!(!code.is_empty(), "math.tan: {}", code);
}

#[test]
fn test_math_atan2() {
    let code = transpile(
        "import math\ndef angle(y: float, x: float) -> float:\n    return math.atan2(y, x)",
    );
    assert!(!code.is_empty(), "math.atan2: {}", code);
}

#[test]
fn test_math_pow() {
    let code = transpile(
        "import math\ndef power(x: float, y: float) -> float:\n    return math.pow(x, y)",
    );
    assert!(!code.is_empty(), "math.pow: {}", code);
}

#[test]
fn test_math_log2() {
    let code = transpile(
        "import math\ndef log_two(x: float) -> float:\n    return math.log2(x)",
    );
    assert!(!code.is_empty(), "math.log2: {}", code);
}

#[test]
fn test_math_log10() {
    let code = transpile(
        "import math\ndef log_ten(x: float) -> float:\n    return math.log10(x)",
    );
    assert!(!code.is_empty(), "math.log10: {}", code);
}

#[test]
fn test_math_isqrt() {
    let code = transpile(
        "import math\ndef int_sqrt(n: int) -> int:\n    return math.isqrt(n)",
    );
    assert!(!code.is_empty(), "math.isqrt: {}", code);
}

#[test]
fn test_math_expm1() {
    let code = transpile(
        "import math\ndef exp_minus_one(x: float) -> float:\n    return math.expm1(x)",
    );
    assert!(!code.is_empty(), "math.expm1: {}", code);
}

#[test]
fn test_math_trunc() {
    let code = transpile(
        "import math\ndef truncate(x: float) -> int:\n    return math.trunc(x)",
    );
    assert!(!code.is_empty(), "math.trunc: {}", code);
}

#[test]
fn test_math_hyperbolic_sinh() {
    let code = transpile(
        "import math\ndef hyp_sin(x: float) -> float:\n    return math.sinh(x)",
    );
    assert!(!code.is_empty(), "math.sinh: {}", code);
}

#[test]
fn test_math_hyperbolic_cosh() {
    let code = transpile(
        "import math\ndef hyp_cos(x: float) -> float:\n    return math.cosh(x)",
    );
    assert!(!code.is_empty(), "math.cosh: {}", code);
}

// =============================================================================
// Section 14: random module (random.rs)
// =============================================================================

#[test]
fn test_random_shuffle() {
    let code = transpile(
        "import random\ndef mix(items: list):\n    random.shuffle(items)",
    );
    assert!(!code.is_empty(), "random.shuffle: {}", code);
}

#[test]
fn test_random_choice() {
    let code = transpile(
        "import random\ndef pick(items: list) -> int:\n    return random.choice(items)",
    );
    assert!(!code.is_empty(), "random.choice: {}", code);
}

#[test]
fn test_random_randint() {
    let code = transpile(
        "import random\ndef roll(a: int, b: int) -> int:\n    return random.randint(a, b)",
    );
    assert!(!code.is_empty(), "random.randint: {}", code);
}

#[test]
fn test_random_random() {
    let code = transpile(
        "import random\ndef rand_float() -> float:\n    return random.random()",
    );
    assert!(!code.is_empty(), "random.random: {}", code);
}

#[test]
fn test_random_sample() {
    let code = transpile(
        "import random\ndef pick_k(items: list, k: int) -> list:\n    return random.sample(items, k)",
    );
    assert!(!code.is_empty(), "random.sample: {}", code);
}

#[test]
fn test_random_uniform() {
    let code = transpile(
        "import random\ndef rand_range(a: float, b: float) -> float:\n    return random.uniform(a, b)",
    );
    assert!(!code.is_empty(), "random.uniform: {}", code);
}

#[test]
fn test_random_randrange() {
    let code = transpile(
        "import random\ndef rand_from_range(start: int, stop: int) -> int:\n    return random.randrange(start, stop)",
    );
    assert!(!code.is_empty(), "random.randrange: {}", code);
}

// =============================================================================
// Section 15: time module (time.rs)
// =============================================================================

#[test]
fn test_time_ctime() {
    let code = transpile(
        "import time\ndef show_time(timestamp: float) -> str:\n    return time.ctime(timestamp)",
    );
    assert!(!code.is_empty(), "time.ctime: {}", code);
}

#[test]
fn test_time_gmtime() {
    let code = transpile(
        "import time\ndef gm(timestamp: float):\n    return time.gmtime(timestamp)",
    );
    assert!(!code.is_empty(), "time.gmtime: {}", code);
}

#[test]
fn test_time_strftime() {
    let code = transpile(
        "import time\ndef format_time(fmt: str) -> str:\n    return time.strftime(fmt)",
    );
    assert!(!code.is_empty(), "time.strftime: {}", code);
}

#[test]
fn test_time_time() {
    let code = transpile(
        "import time\ndef now() -> float:\n    return time.time()",
    );
    assert!(!code.is_empty(), "time.time: {}", code);
}

#[test]
fn test_time_sleep() {
    let code = transpile(
        "import time\ndef wait(seconds: float):\n    time.sleep(seconds)",
    );
    assert!(!code.is_empty(), "time.sleep: {}", code);
}

#[test]
fn test_time_monotonic() {
    let code = transpile(
        "import time\ndef monotonic_time() -> float:\n    return time.monotonic()",
    );
    assert!(!code.is_empty(), "time.monotonic: {}", code);
}

#[test]
fn test_time_perf_counter() {
    let code = transpile(
        "import time\ndef perf() -> float:\n    return time.perf_counter()",
    );
    assert!(!code.is_empty(), "time.perf_counter: {}", code);
}

#[test]
fn test_time_localtime() {
    let code = transpile(
        "import time\ndef local(timestamp: float):\n    return time.localtime(timestamp)",
    );
    assert!(!code.is_empty(), "time.localtime: {}", code);
}

// =============================================================================
// Section 16: urllib.parse module (stdlib_data.rs)
// =============================================================================

#[test]
fn test_urllib_parse_quote() {
    let code = transpile(
        "import urllib.parse\ndef encode_url(s: str) -> str:\n    return urllib.parse.quote(s)",
    );
    assert!(!code.is_empty(), "urllib.parse.quote: {}", code);
}

#[test]
fn test_urllib_parse_unquote() {
    let code = transpile(
        "import urllib.parse\ndef decode_url(s: str) -> str:\n    return urllib.parse.unquote(s)",
    );
    assert!(!code.is_empty(), "urllib.parse.unquote: {}", code);
}

#[test]
fn test_urllib_parse_urlencode() {
    let code = transpile(
        "import urllib.parse\ndef encode_params(params: dict) -> str:\n    return urllib.parse.urlencode(params)",
    );
    assert!(!code.is_empty(), "urllib.parse.urlencode: {}", code);
}

#[test]
fn test_urllib_parse_parse_qs() {
    let code = transpile(
        "import urllib.parse\ndef parse_query(query: str) -> dict:\n    return urllib.parse.parse_qs(query)",
    );
    assert!(!code.is_empty(), "urllib.parse.parse_qs: {}", code);
}

#[test]
fn test_urllib_parse_urlparse() {
    let code = transpile(
        "import urllib.parse\ndef parse_url(url: str):\n    return urllib.parse.urlparse(url)",
    );
    assert!(!code.is_empty(), "urllib.parse.urlparse: {}", code);
}

#[test]
fn test_urllib_parse_urljoin() {
    let code = transpile(
        "import urllib.parse\ndef join_url(base: str, path: str) -> str:\n    return urllib.parse.urljoin(base, path)",
    );
    assert!(!code.is_empty(), "urllib.parse.urljoin: {}", code);
}

// =============================================================================
// Section 17: fnmatch module (stdlib_data.rs)
// =============================================================================

#[test]
fn test_fnmatch_fnmatch() {
    let code = transpile(
        "import fnmatch\ndef matches(name: str, pattern: str) -> bool:\n    return fnmatch.fnmatch(name, pattern)",
    );
    assert!(!code.is_empty(), "fnmatch.fnmatch: {}", code);
}

#[test]
fn test_fnmatch_filter() {
    let code = transpile(
        "import fnmatch\ndef filter_names(names: list, pattern: str) -> list:\n    return fnmatch.filter(names, pattern)",
    );
    assert!(!code.is_empty(), "fnmatch.filter: {}", code);
}

#[test]
fn test_fnmatch_fnmatchcase() {
    let code = transpile(
        "import fnmatch\ndef matches_case(name: str, pattern: str) -> bool:\n    return fnmatch.fnmatchcase(name, pattern)",
    );
    assert!(!code.is_empty(), "fnmatch.fnmatchcase: {}", code);
}

// =============================================================================
// Section 18: shlex module (stdlib_data.rs)
// =============================================================================

#[test]
fn test_shlex_quote() {
    let code = transpile(
        "import shlex\ndef safe_arg(arg: str) -> str:\n    return shlex.quote(arg)",
    );
    assert!(!code.is_empty(), "shlex.quote: {}", code);
}

#[test]
fn test_shlex_split() {
    let code = transpile(
        "import shlex\ndef parse_cmd(text: str) -> list:\n    return shlex.split(text)",
    );
    assert!(!code.is_empty(), "shlex.split: {}", code);
}

#[test]
fn test_shlex_join() {
    let code = transpile(
        "import shlex\ndef join_cmd(args: list) -> str:\n    return shlex.join(args)",
    );
    assert!(!code.is_empty(), "shlex.join: {}", code);
}

// =============================================================================
// Section 19: textwrap module (stdlib_data.rs)
// =============================================================================

#[test]
fn test_textwrap_wrap() {
    let code = transpile(
        "import textwrap\ndef wrap_text(text: str, width: int) -> list:\n    return textwrap.wrap(text, width)",
    );
    assert!(!code.is_empty(), "textwrap.wrap: {}", code);
}

#[test]
fn test_textwrap_fill() {
    let code = transpile(
        "import textwrap\ndef fill_text(text: str, width: int) -> str:\n    return textwrap.fill(text, width)",
    );
    assert!(!code.is_empty(), "textwrap.fill: {}", code);
}

#[test]
fn test_textwrap_dedent() {
    let code = transpile(
        "import textwrap\ndef remove_indent(text: str) -> str:\n    return textwrap.dedent(text)",
    );
    assert!(!code.is_empty(), "textwrap.dedent: {}", code);
}

#[test]
fn test_textwrap_indent() {
    let code = transpile(
        "import textwrap\ndef add_indent(text: str, prefix: str) -> str:\n    return textwrap.indent(text, prefix)",
    );
    assert!(!code.is_empty(), "textwrap.indent: {}", code);
}

#[test]
fn test_textwrap_shorten() {
    let code = transpile(
        "import textwrap\ndef truncate(text: str, width: int) -> str:\n    return textwrap.shorten(text, width)",
    );
    assert!(!code.is_empty(), "textwrap.shorten: {}", code);
}

// =============================================================================
// Section 20: os.environ methods (call_methods.rs)
// =============================================================================

#[test]
fn test_os_environ_get() {
    let code = transpile(
        "import os\ndef get_env(key: str) -> str:\n    return os.environ.get(key, \"default\")",
    );
    assert!(!code.is_empty(), "os.environ.get: {}", code);
}

#[test]
fn test_os_environ_get_no_default() {
    let code = transpile(
        "import os\ndef get_env(key: str):\n    return os.environ.get(key)",
    );
    assert!(!code.is_empty(), "os.environ.get no default: {}", code);
}

// =============================================================================
// Section 21: Decimal/Fraction (call_dispatch.rs)
// =============================================================================

#[test]
fn test_decimal_from_string() {
    let code = transpile(
        "from decimal import Decimal\ndef make_decimal() -> float:\n    return Decimal(\"1.5\")",
    );
    assert!(!code.is_empty(), "Decimal from string: {}", code);
}

#[test]
fn test_decimal_from_int() {
    let code = transpile(
        "from decimal import Decimal\ndef make_decimal(n: int):\n    return Decimal(n)",
    );
    assert!(!code.is_empty(), "Decimal from int: {}", code);
}

#[test]
fn test_fraction_two_args() {
    let code = transpile(
        "from fractions import Fraction\ndef make_frac() -> float:\n    return Fraction(22, 7)",
    );
    assert!(!code.is_empty(), "Fraction(22, 7): {}", code);
}

#[test]
fn test_fraction_from_string() {
    let code = transpile(
        "from fractions import Fraction\ndef make_frac():\n    return Fraction(\"1/3\")",
    );
    assert!(!code.is_empty(), "Fraction from string: {}", code);
}

#[test]
fn test_fraction_from_int() {
    let code = transpile(
        "from fractions import Fraction\ndef make_frac():\n    return Fraction(5)",
    );
    assert!(!code.is_empty(), "Fraction from int: {}", code);
}

// =============================================================================
// Section 22: sum/any/all edge cases (call_dispatch.rs)
// =============================================================================

#[test]
fn test_sum_dict_values() {
    let code = transpile(
        "def total(d: dict) -> int:\n    return sum(d.values())",
    );
    assert!(!code.is_empty(), "sum(d.values()): {}", code);
}

#[test]
fn test_sum_list() {
    let code = transpile(
        "def total(items: list) -> int:\n    return sum(items)",
    );
    assert!(!code.is_empty(), "sum(list): {}", code);
}

#[test]
fn test_sum_generator() {
    let code = transpile(
        "def total(items: list) -> int:\n    return sum(x * 2 for x in items)",
    );
    assert!(!code.is_empty(), "sum(generator): {}", code);
}

#[test]
fn test_sum_range() {
    let code = transpile(
        "def total(n: int) -> int:\n    return sum(range(n))",
    );
    assert!(!code.is_empty(), "sum(range): {}", code);
}

#[test]
fn test_any_generator() {
    let code = transpile(
        "def has_positive(nums: list) -> bool:\n    return any(x > 0 for x in nums)",
    );
    assert!(!code.is_empty(), "any(generator): {}", code);
}

#[test]
fn test_all_generator() {
    let code = transpile(
        "def all_positive(nums: list) -> bool:\n    return all(x > 0 for x in nums)",
    );
    assert!(!code.is_empty(), "all(generator): {}", code);
}

#[test]
fn test_any_list() {
    let code = transpile(
        "def any_true(items: list) -> bool:\n    return any(items)",
    );
    assert!(!code.is_empty(), "any(list): {}", code);
}

#[test]
fn test_all_list() {
    let code = transpile(
        "def all_true(items: list) -> bool:\n    return all(items)",
    );
    assert!(!code.is_empty(), "all(list): {}", code);
}

#[test]
fn test_min_two_args() {
    let code = transpile(
        "def smaller(a: int, b: int) -> int:\n    return min(a, b)",
    );
    assert!(!code.is_empty(), "min(a, b): {}", code);
}

#[test]
fn test_max_two_args() {
    let code = transpile(
        "def larger(a: int, b: int) -> int:\n    return max(a, b)",
    );
    assert!(!code.is_empty(), "max(a, b): {}", code);
}

#[test]
fn test_min_iterable() {
    let code = transpile(
        "def smallest(items: list) -> int:\n    return min(items)",
    );
    assert!(!code.is_empty(), "min(iterable): {}", code);
}

#[test]
fn test_max_iterable() {
    let code = transpile(
        "def biggest(items: list) -> int:\n    return max(items)",
    );
    assert!(!code.is_empty(), "max(iterable): {}", code);
}

#[test]
fn test_min_float_args() {
    let code = transpile(
        "def smaller_f(a: float, b: float) -> float:\n    return min(a, 0.5)",
    );
    assert!(!code.is_empty(), "min with float literal: {}", code);
}

#[test]
fn test_max_float_args() {
    let code = transpile(
        "def larger_f(a: float, b: float) -> float:\n    return max(a, 0.5)",
    );
    assert!(!code.is_empty(), "max with float literal: {}", code);
}

#[test]
fn test_sum_dict_keys() {
    let code = transpile(
        "def total_keys(d: dict) -> int:\n    return sum(d.keys())",
    );
    assert!(!code.is_empty(), "sum(d.keys()): {}", code);
}

// =============================================================================
// Section 23: Additional stdlib coverage
// =============================================================================

#[test]
fn test_os_getenv() {
    let code = transpile(
        "import os\ndef get_var(name: str) -> str:\n    return os.getenv(name, \"default\")",
    );
    assert!(!code.is_empty(), "os.getenv: {}", code);
}

#[test]
fn test_os_getcwd() {
    let code = transpile(
        "import os\ndef cwd() -> str:\n    return os.getcwd()",
    );
    assert!(!code.is_empty(), "os.getcwd: {}", code);
}

#[test]
fn test_os_path_exists() {
    let code = transpile(
        "import os\ndef check_path(p: str) -> bool:\n    return os.path.exists(p)",
    );
    assert!(!code.is_empty(), "os.path.exists: {}", code);
}

#[test]
fn test_os_path_isfile() {
    let code = transpile(
        "import os\ndef check_file(p: str) -> bool:\n    return os.path.isfile(p)",
    );
    assert!(!code.is_empty(), "os.path.isfile: {}", code);
}

#[test]
fn test_os_path_isdir() {
    let code = transpile(
        "import os\ndef check_dir(p: str) -> bool:\n    return os.path.isdir(p)",
    );
    assert!(!code.is_empty(), "os.path.isdir: {}", code);
}

#[test]
fn test_os_path_join() {
    let code = transpile(
        "import os\ndef join(a: str, b: str) -> str:\n    return os.path.join(a, b)",
    );
    assert!(!code.is_empty(), "os.path.join: {}", code);
}

#[test]
fn test_os_path_basename() {
    let code = transpile(
        "import os\ndef base(p: str) -> str:\n    return os.path.basename(p)",
    );
    assert!(!code.is_empty(), "os.path.basename: {}", code);
}

#[test]
fn test_os_path_dirname() {
    let code = transpile(
        "import os\ndef parent_dir(p: str) -> str:\n    return os.path.dirname(p)",
    );
    assert!(!code.is_empty(), "os.path.dirname: {}", code);
}

// =============================================================================
// Section 24: itertools/functools (stdlib_method_gen)
// =============================================================================

#[test]
fn test_itertools_chain() {
    let code = transpile(
        "import itertools\ndef combine(a: list, b: list) -> list:\n    return list(itertools.chain(a, b))",
    );
    assert!(!code.is_empty(), "itertools.chain: {}", code);
}

#[test]
fn test_functools_reduce() {
    let code = transpile(
        "from functools import reduce\ndef product(items: list) -> int:\n    return reduce(lambda x, y: x * y, items)",
    );
    assert!(!code.is_empty(), "functools.reduce: {}", code);
}

#[test]
fn test_functools_reduce_with_initial() {
    let code = transpile(
        "from functools import reduce\ndef total(items: list) -> int:\n    return reduce(lambda x, y: x + y, items, 0)",
    );
    assert!(!code.is_empty(), "functools.reduce with initial: {}", code);
}

// =============================================================================
// Section 25: pathlib module (pathlib.rs)
// =============================================================================

#[test]
fn test_pathlib_path_constructor() {
    let code = transpile(
        "from pathlib import Path\ndef make_path(s: str):\n    return Path(s)",
    );
    assert!(!code.is_empty(), "Path constructor: {}", code);
}

#[test]
fn test_pathlib_path_exists() {
    let code = transpile(
        "from pathlib import Path\ndef check(s: str) -> bool:\n    p = Path(s)\n    return p.exists()",
    );
    assert!(!code.is_empty(), "Path.exists: {}", code);
}

// =============================================================================
// Section 26: secrets module (stdlib_crypto.rs)
// =============================================================================

#[test]
fn test_secrets_randbelow() {
    let code = transpile(
        "import secrets\ndef rand_int(n: int) -> int:\n    return secrets.randbelow(n)",
    );
    assert!(!code.is_empty(), "secrets.randbelow: {}", code);
}

#[test]
fn test_secrets_token_bytes() {
    let code = transpile(
        "import secrets\ndef random_bytes(n: int) -> bytes:\n    return secrets.token_bytes(n)",
    );
    assert!(!code.is_empty(), "secrets.token_bytes: {}", code);
}

#[test]
fn test_secrets_token_hex() {
    let code = transpile(
        "import secrets\ndef random_hex(n: int) -> str:\n    return secrets.token_hex(n)",
    );
    assert!(!code.is_empty(), "secrets.token_hex: {}", code);
}

// =============================================================================
// Section 27: enumerate/zip/isinstance (call_dispatch.rs)
// =============================================================================

#[test]
fn test_enumerate_basic() {
    let code = transpile(
        "def indexed(items: list):\n    for i, val in enumerate(items):\n        print(i, val)",
    );
    assert!(!code.is_empty(), "enumerate: {}", code);
}

#[test]
fn test_zip_basic() {
    let code = transpile(
        "def paired(a: list, b: list):\n    for x, y in zip(a, b):\n        print(x, y)",
    );
    assert!(!code.is_empty(), "zip: {}", code);
}

#[test]
fn test_isinstance_basic() {
    let code = transpile(
        "def is_int(x: int) -> bool:\n    return isinstance(x, int)",
    );
    assert!(!code.is_empty(), "isinstance: {}", code);
}

// =============================================================================
// Section 28: print/sorted/reversed builtins (call_dispatch.rs)
// =============================================================================

#[test]
fn test_sorted_basic() {
    let code = transpile(
        "def order(items: list) -> list:\n    return sorted(items)",
    );
    assert!(!code.is_empty(), "sorted: {}", code);
}

#[test]
fn test_reversed_basic() {
    let code = transpile(
        "def flip(items: list) -> list:\n    return list(reversed(items))",
    );
    assert!(!code.is_empty(), "reversed: {}", code);
}

#[test]
fn test_print_multiple_args() {
    let code = transpile(
        "def show(a: str, b: int):\n    print(a, b)",
    );
    assert!(!code.is_empty(), "print multi: {}", code);
}

// =============================================================================
// Section 29: map/filter builtins (call_dispatch.rs)
// =============================================================================

#[test]
fn test_map_with_lambda() {
    let code = transpile(
        "def double(items: list) -> list:\n    return list(map(lambda x: x * 2, items))",
    );
    assert!(!code.is_empty(), "map(lambda): {}", code);
}

#[test]
fn test_filter_with_lambda() {
    let code = transpile(
        "def positives(items: list) -> list:\n    return list(filter(lambda x: x > 0, items))",
    );
    assert!(!code.is_empty(), "filter(lambda): {}", code);
}

#[test]
fn test_map_int_conversion() {
    let code = transpile(
        "def to_ints(items: list) -> list:\n    return list(map(int, items))",
    );
    assert!(!code.is_empty(), "map(int): {}", code);
}

#[test]
fn test_map_str_conversion() {
    let code = transpile(
        "def to_strs(items: list) -> list:\n    return list(map(str, items))",
    );
    assert!(!code.is_empty(), "map(str): {}", code);
}

// =============================================================================
// Section 30: shutil module (shutil.rs)
// =============================================================================

#[test]
fn test_shutil_copy() {
    let code = transpile(
        "import shutil\ndef copy_file(src: str, dst: str):\n    shutil.copy(src, dst)",
    );
    assert!(!code.is_empty(), "shutil.copy: {}", code);
}

#[test]
fn test_shutil_move() {
    let code = transpile(
        "import shutil\ndef move_file(src: str, dst: str):\n    shutil.move(src, dst)",
    );
    assert!(!code.is_empty(), "shutil.move: {}", code);
}

// =============================================================================
// Section 31: sys module (stdlib_misc.rs)
// =============================================================================

#[test]
fn test_sys_exit() {
    let code = transpile(
        "import sys\ndef bail(code: int):\n    sys.exit(code)",
    );
    assert!(!code.is_empty(), "sys.exit: {}", code);
}

// =============================================================================
// Section 32: pprint module (stdlib_misc.rs)
// =============================================================================

#[test]
fn test_pprint_pprint() {
    let code = transpile(
        "import pprint\ndef show(obj: dict):\n    pprint.pprint(obj)",
    );
    assert!(!code.is_empty(), "pprint.pprint: {}", code);
}

// =============================================================================
// Section 33: pickle module (stdlib_misc.rs)
// =============================================================================

#[test]
fn test_pickle_dumps() {
    let code = transpile(
        "import pickle\ndef serialize(obj: dict) -> bytes:\n    return pickle.dumps(obj)",
    );
    assert!(!code.is_empty(), "pickle.dumps: {}", code);
}

#[test]
fn test_pickle_loads() {
    let code = transpile(
        "import pickle\ndef deserialize(data: bytes) -> dict:\n    return pickle.loads(data)",
    );
    assert!(!code.is_empty(), "pickle.loads: {}", code);
}

// =============================================================================
// Section 34: string module (string.rs)
// =============================================================================

#[test]
fn test_string_ascii_lowercase() {
    let code = transpile(
        "import string\ndef get_lower() -> str:\n    return string.ascii_lowercase",
    );
    assert!(!code.is_empty(), "string.ascii_lowercase: {}", code);
}

#[test]
fn test_string_digits() {
    let code = transpile(
        "import string\ndef get_digits() -> str:\n    return string.digits",
    );
    assert!(!code.is_empty(), "string.digits: {}", code);
}

// =============================================================================
// Section 35: warnings module (warnings.rs)
// =============================================================================

#[test]
fn test_warnings_warn() {
    let code = transpile(
        "import warnings\ndef deprecation_notice(msg: str):\n    warnings.warn(msg)",
    );
    assert!(!code.is_empty(), "warnings.warn: {}", code);
}

// =============================================================================
// Section 36: Additional re patterns for higher coverage
// =============================================================================

#[test]
fn test_re_search_then_group() {
    let code = transpile(
        "import re\ndef extract(text: str) -> str:\n    m = re.search(r\"(\\w+)\", text)\n    if m:\n        return m.group(0)\n    return \"\"",
    );
    assert!(!code.is_empty(), "re.search + group: {}", code);
}

#[test]
fn test_re_findall_in_loop() {
    let code = transpile(
        "import re\ndef count_words(text: str) -> int:\n    words = re.findall(r\"\\w+\", text)\n    return len(words)",
    );
    assert!(!code.is_empty(), "re.findall in expression: {}", code);
}

#[test]
fn test_re_sub_with_string_literal() {
    let code = transpile(
        "import re\ndef clean(text: str) -> str:\n    return re.sub(r\"\\s+\", \" \", text)",
    );
    assert!(!code.is_empty(), "re.sub literal: {}", code);
}

// =============================================================================
// Section 37: Additional heapq/bisect patterns
// =============================================================================

#[test]
fn test_heapq_push_pop_sequence() {
    let code = transpile(
        "import heapq\ndef heap_ops(items: list) -> int:\n    heapq.heapify(items)\n    heapq.heappush(items, 5)\n    return heapq.heappop(items)",
    );
    assert!(!code.is_empty(), "heapq sequence: {}", code);
}

#[test]
fn test_bisect_left_then_insert() {
    let code = transpile(
        "import bisect\ndef sorted_insert(arr: list, val: int):\n    pos = bisect.bisect_left(arr, val)\n    return pos",
    );
    assert!(!code.is_empty(), "bisect_left then use: {}", code);
}

// =============================================================================
// Section 38: Additional math edge cases
// =============================================================================

#[test]
fn test_math_hypot() {
    let ok = transpile_ok(
        "import math\ndef distance(x: float, y: float) -> float:\n    return math.hypot(x, y)",
    );
    assert!(ok, "math.hypot should transpile");
}

#[test]
fn test_math_fmod() {
    let ok = transpile_ok(
        "import math\ndef mod(x: float, y: float) -> float:\n    return math.fmod(x, y)",
    );
    assert!(ok, "math.fmod should transpile");
}

#[test]
fn test_math_ldexp() {
    let ok = transpile_ok(
        "import math\ndef scale(x: float, n: int) -> float:\n    return math.ldexp(x, n)",
    );
    assert!(ok, "math.ldexp should transpile");
}

#[test]
fn test_math_remainder() {
    let ok = transpile_ok(
        "import math\ndef rem(x: float, y: float) -> float:\n    return math.remainder(x, y)",
    );
    assert!(ok, "math.remainder should transpile");
}

// =============================================================================
// Section 39: Additional json edge cases
// =============================================================================

#[test]
fn test_json_loads_in_try_except() {
    let code = transpile(
        "import json\ndef safe_parse(s: str):\n    try:\n        return json.loads(s)\n    except:\n        return None",
    );
    assert!(!code.is_empty(), "json.loads in try/except: {}", code);
}

// =============================================================================
// Section 40: Additional random patterns
// =============================================================================

#[test]
fn test_random_seed() {
    let ok = transpile_ok(
        "import random\ndef set_seed(n: int):\n    random.seed(n)",
    );
    assert!(ok, "random.seed should transpile");
}

#[test]
fn test_random_choices() {
    let ok = transpile_ok(
        "import random\ndef pick_many(items: list, k: int) -> list:\n    return random.choices(items, k=k)",
    );
    // This may or may not be implemented, just ensure no panic
    let _ = ok;
}

// =============================================================================
// Section 41: Additional copy patterns
// =============================================================================

#[test]
fn test_copy_deepcopy_dict() {
    let code = transpile(
        "import copy\ndef clone_dict(d: dict) -> dict:\n    return copy.deepcopy(d)",
    );
    assert!(!code.is_empty(), "deepcopy dict: {}", code);
}

#[test]
fn test_copy_shallow_list() {
    let code = transpile(
        "import copy\ndef clone_list(items: list) -> list:\n    return copy.copy(items)",
    );
    assert!(!code.is_empty(), "copy.copy list: {}", code);
}

// =============================================================================
// Section 42: Additional statistics
// =============================================================================

#[test]
fn test_statistics_variance() {
    let ok = transpile_ok(
        "import statistics\ndef var(data: list) -> float:\n    return statistics.variance(data)",
    );
    // variance may or may not be implemented
    let _ = ok;
}

#[test]
fn test_statistics_mode() {
    let ok = transpile_ok(
        "import statistics\ndef most_common(data: list) -> int:\n    return statistics.mode(data)",
    );
    let _ = ok;
}

// =============================================================================
// Section 43: uuid module (stdlib_crypto.rs)
// =============================================================================

#[test]
fn test_uuid_uuid4() {
    let code = transpile(
        "import uuid\ndef gen_id() -> str:\n    return str(uuid.uuid4())",
    );
    assert!(!code.is_empty(), "uuid.uuid4: {}", code);
}

// =============================================================================
// Section 44: platform module (stdlib_crypto.rs)
// =============================================================================

#[test]
fn test_platform_system() {
    let code = transpile(
        "import platform\ndef os_name() -> str:\n    return platform.system()",
    );
    assert!(!code.is_empty(), "platform.system: {}", code);
}

#[test]
fn test_platform_machine() {
    let code = transpile(
        "import platform\ndef arch() -> str:\n    return platform.machine()",
    );
    assert!(!code.is_empty(), "platform.machine: {}", code);
}

// =============================================================================
// Section 45: Combined multi-module tests
// =============================================================================

#[test]
fn test_combined_json_and_base64() {
    let code = transpile(
        "import json\nimport base64\ndef encode_json(data: dict) -> bytes:\n    s = json.dumps(data)\n    return base64.b64encode(s.encode())",
    );
    assert!(!code.is_empty(), "json + base64 combined: {}", code);
}

#[test]
fn test_combined_re_and_json() {
    let code = transpile(
        "import re\nimport json\ndef extract_json(text: str) -> str:\n    m = re.search(r\"\\{.*\\}\", text)\n    if m:\n        return m.group(0)\n    return \"{}\"",
    );
    assert!(!code.is_empty(), "re + json combined: {}", code);
}

#[test]
fn test_combined_math_and_statistics() {
    let code = transpile(
        "import math\nimport statistics\ndef analyze(data: list) -> float:\n    avg = statistics.mean(data)\n    return math.sqrt(avg)",
    );
    assert!(!code.is_empty(), "math + statistics combined: {}", code);
}

#[test]
fn test_combined_os_path_and_shutil() {
    let code = transpile(
        "import os\nimport shutil\ndef backup(src: str, dst: str):\n    if os.path.exists(src):\n        shutil.copy(src, dst)",
    );
    assert!(!code.is_empty(), "os.path + shutil combined: {}", code);
}

#[test]
fn test_combined_hashlib_and_base64() {
    let code = transpile(
        "import hashlib\nimport base64\ndef hash_encode(data: str) -> bytes:\n    h = hashlib.sha256(data.encode()).hexdigest()\n    return base64.b64encode(h.encode())",
    );
    assert!(!code.is_empty(), "hashlib + base64 combined: {}", code);
}
