//! Coverage tests for stdlib module methods in expr_gen.rs
//!
//! These tests ensure all Python stdlib function mappings are exercised.

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).unwrap_or_else(|e| format!("ERROR: {}", e))
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============ time module ============

#[test]
fn test_time_sleep() {
    let code = "import time\ndef f():\n    time.sleep(1.5)";
    assert!(transpile_ok(code));
    let result = transpile(code);
    assert!(result.contains("thread::sleep") || result.contains("Duration"));
}

#[test]
fn test_time_ctime() {
    let code = "import time\ndef f(ts: float) -> str:\n    return time.ctime(ts)";
    assert!(transpile_ok(code));
}

#[test]
fn test_time_strftime() {
    let code = "import time\ndef f():\n    return time.strftime('%Y-%m-%d', time.localtime())";
    assert!(transpile_ok(code));
}

#[test]
fn test_time_strptime() {
    let code = "import time\ndef f(s: str):\n    return time.strptime(s, '%Y-%m-%d')";
    assert!(transpile_ok(code));
}

#[test]
fn test_time_time() {
    let code = "import time\ndef f() -> float:\n    return time.time()";
    assert!(transpile_ok(code));
}

#[test]
fn test_time_monotonic() {
    let code = "import time\ndef f() -> float:\n    return time.monotonic()";
    assert!(transpile_ok(code));
}

#[test]
fn test_time_perf_counter() {
    let code = "import time\ndef f() -> float:\n    return time.perf_counter()";
    assert!(transpile_ok(code));
}

// ============ datetime module ============

#[test]
fn test_datetime_datetime_now() {
    let code = "from datetime import datetime\ndef f():\n    return datetime.now()";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_datetime_constructor() {
    let code = "from datetime import datetime\ndef f():\n    return datetime(2024, 1, 15)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_datetime_constructor_full() {
    let code = "from datetime import datetime\ndef f():\n    return datetime(2024, 1, 15, 10, 30, 45)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_date() {
    let code = "from datetime import date\ndef f():\n    return date(2024, 1, 15)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timedelta() {
    let code = "from datetime import timedelta\ndef f():\n    return timedelta(days=5)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timedelta_hours() {
    let code = "from datetime import timedelta\ndef f():\n    return timedelta(hours=2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timedelta_minutes() {
    let code = "from datetime import timedelta\ndef f():\n    return timedelta(minutes=30)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timedelta_seconds() {
    let code = "from datetime import timedelta\ndef f():\n    return timedelta(seconds=60)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timedelta_weeks() {
    let code = "from datetime import timedelta\ndef f():\n    return timedelta(weeks=1)";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_strptime() {
    let code = "from datetime import datetime\ndef f(s: str):\n    return datetime.strptime(s, '%Y-%m-%d')";
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_strftime() {
    let code = "from datetime import datetime\ndef f(dt):\n    return dt.strftime('%Y-%m-%d')";
    assert!(transpile_ok(code));
}

// ============ pathlib module ============

#[test]
fn test_pathlib_path() {
    let code = "from pathlib import Path\ndef f(s: str):\n    return Path(s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_exists() {
    let code = "from pathlib import Path\ndef f(s: str) -> bool:\n    return Path(s).exists()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_is_file() {
    let code = "from pathlib import Path\ndef f(s: str) -> bool:\n    return Path(s).is_file()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_is_dir() {
    let code = "from pathlib import Path\ndef f(s: str) -> bool:\n    return Path(s).is_dir()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_read_text() {
    let code = "from pathlib import Path\ndef f(s: str) -> str:\n    return Path(s).read_text()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_read_bytes() {
    let code = "from pathlib import Path\ndef f(s: str):\n    return Path(s).read_bytes()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_write_text() {
    let code = "from pathlib import Path\ndef f(p: str, content: str):\n    Path(p).write_text(content)";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_write_bytes() {
    let code = "from pathlib import Path\ndef f(p: str, data: bytes):\n    Path(p).write_bytes(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_mkdir() {
    let code = "from pathlib import Path\ndef f(p: str):\n    Path(p).mkdir()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_mkdir_parents() {
    let code = "from pathlib import Path\ndef f(p: str):\n    Path(p).mkdir(parents=True)";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_unlink() {
    let code = "from pathlib import Path\ndef f(p: str):\n    Path(p).unlink()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_rmdir() {
    let code = "from pathlib import Path\ndef f(p: str):\n    Path(p).rmdir()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_parent() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return Path(p).parent";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_name() {
    let code = "from pathlib import Path\ndef f(p: str) -> str:\n    return Path(p).name";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_stem() {
    let code = "from pathlib import Path\ndef f(p: str) -> str:\n    return Path(p).stem";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_suffix() {
    let code = "from pathlib import Path\ndef f(p: str) -> str:\n    return Path(p).suffix";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_with_suffix() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return Path(p).with_suffix('.txt')";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_joinpath() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return Path(p).joinpath('subdir')";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_glob() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return list(Path(p).glob('*.txt'))";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_rglob() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return list(Path(p).rglob('*.py'))";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_iterdir() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return list(Path(p).iterdir())";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_resolve() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return Path(p).resolve()";
    assert!(transpile_ok(code));
}

#[test]
fn test_path_absolute() {
    let code = "from pathlib import Path\ndef f(p: str):\n    return Path(p).absolute()";
    assert!(transpile_ok(code));
}

// ============ os module ============

#[test]
fn test_os_path_exists() {
    let code = "import os\ndef f(p: str) -> bool:\n    return os.path.exists(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_isfile() {
    let code = "import os\ndef f(p: str) -> bool:\n    return os.path.isfile(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_isdir() {
    let code = "import os\ndef f(p: str) -> bool:\n    return os.path.isdir(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_join() {
    let code = "import os\ndef f(a: str, b: str) -> str:\n    return os.path.join(a, b)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_dirname() {
    let code = "import os\ndef f(p: str) -> str:\n    return os.path.dirname(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_basename() {
    let code = "import os\ndef f(p: str) -> str:\n    return os.path.basename(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_splitext() {
    let code = "import os\ndef f(p: str):\n    return os.path.splitext(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_getcwd() {
    let code = "import os\ndef f() -> str:\n    return os.getcwd()";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_chdir() {
    let code = "import os\ndef f(p: str):\n    os.chdir(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_listdir() {
    let code = "import os\ndef f(p: str):\n    return os.listdir(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_makedirs() {
    let code = "import os\ndef f(p: str):\n    os.makedirs(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_makedirs_exist_ok() {
    let code = "import os\ndef f(p: str):\n    os.makedirs(p, exist_ok=True)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_remove() {
    let code = "import os\ndef f(p: str):\n    os.remove(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_rmdir() {
    let code = "import os\ndef f(p: str):\n    os.rmdir(p)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_rename() {
    let code = "import os\ndef f(src: str, dst: str):\n    os.rename(src, dst)";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_environ_get() {
    let code = "import os\ndef f(key: str) -> str:\n    return os.environ.get(key, '')";
    assert!(transpile_ok(code));
}

#[test]
fn test_os_getenv() {
    let code = "import os\ndef f(key: str) -> str:\n    return os.getenv(key, '')";
    assert!(transpile_ok(code));
}

// ============ json module ============

#[test]
fn test_json_dumps() {
    let code = "import json\ndef f(data: dict) -> str:\n    return json.dumps(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_json_dumps_indent() {
    let code = "import json\ndef f(data: dict) -> str:\n    return json.dumps(data, indent=2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_json_loads() {
    let code = "import json\ndef f(s: str):\n    return json.loads(s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_json_load() {
    let code = "import json\ndef f():\n    with open('data.json') as f:\n        return json.load(f)";
    assert!(transpile_ok(code));
}

#[test]
fn test_json_dump() {
    let code = "import json\ndef f(data: dict):\n    with open('out.json', 'w') as f:\n        json.dump(data, f)";
    assert!(transpile_ok(code));
}

// ============ re module ============

#[test]
fn test_re_match() {
    let code = "import re\ndef f(pattern: str, s: str):\n    return re.match(pattern, s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_re_search() {
    let code = "import re\ndef f(pattern: str, s: str):\n    return re.search(pattern, s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_re_findall() {
    let code = "import re\ndef f(pattern: str, s: str):\n    return re.findall(pattern, s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_re_split() {
    let code = "import re\ndef f(pattern: str, s: str):\n    return re.split(pattern, s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_re_sub() {
    let code = "import re\ndef f(pattern: str, repl: str, s: str) -> str:\n    return re.sub(pattern, repl, s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_re_compile() {
    let code = "import re\ndef f(pattern: str):\n    return re.compile(pattern)";
    assert!(transpile_ok(code));
}

// ============ hashlib module ============

#[test]
fn test_hashlib_sha256() {
    let code = "import hashlib\ndef f(data: bytes):\n    return hashlib.sha256(data).hexdigest()";
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_md5() {
    let code = "import hashlib\ndef f(data: bytes):\n    return hashlib.md5(data).hexdigest()";
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha1() {
    let code = "import hashlib\ndef f(data: bytes):\n    return hashlib.sha1(data).hexdigest()";
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha512() {
    let code = "import hashlib\ndef f(data: bytes):\n    return hashlib.sha512(data).hexdigest()";
    assert!(transpile_ok(code));
}

// ============ base64 module ============

#[test]
fn test_base64_b64encode() {
    let code = "import base64\ndef f(data: bytes):\n    return base64.b64encode(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_base64_b64decode() {
    let code = "import base64\ndef f(data: bytes):\n    return base64.b64decode(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_base64_urlsafe_b64encode() {
    let code = "import base64\ndef f(data: bytes):\n    return base64.urlsafe_b64encode(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_base64_urlsafe_b64decode() {
    let code = "import base64\ndef f(data: bytes):\n    return base64.urlsafe_b64decode(data)";
    assert!(transpile_ok(code));
}

// ============ random module ============

#[test]
fn test_random_random() {
    let code = "import random\ndef f() -> float:\n    return random.random()";
    assert!(transpile_ok(code));
}

#[test]
fn test_random_randint() {
    let code = "import random\ndef f(a: int, b: int) -> int:\n    return random.randint(a, b)";
    assert!(transpile_ok(code));
}

#[test]
fn test_random_choice() {
    let code = "import random\ndef f(items: list):\n    return random.choice(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_random_shuffle() {
    let code = "import random\ndef f(items: list):\n    random.shuffle(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_random_sample() {
    let code = "import random\ndef f(items: list, k: int):\n    return random.sample(items, k)";
    assert!(transpile_ok(code));
}

#[test]
fn test_random_uniform() {
    let code = "import random\ndef f(a: float, b: float) -> float:\n    return random.uniform(a, b)";
    assert!(transpile_ok(code));
}

#[test]
fn test_random_seed() {
    let code = "import random\ndef f(s: int):\n    random.seed(s)";
    assert!(transpile_ok(code));
}

// ============ math module ============

#[test]
fn test_math_sqrt() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.sqrt(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_floor() {
    let code = "import math\ndef f(x: float) -> int:\n    return math.floor(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_ceil() {
    let code = "import math\ndef f(x: float) -> int:\n    return math.ceil(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_sin() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.sin(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_cos() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.cos(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_tan() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.tan(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_exp() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.exp(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.log(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log10() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.log10(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_pow() {
    let code = "import math\ndef f(x: float, y: float) -> float:\n    return math.pow(x, y)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_fabs() {
    let code = "import math\ndef f(x: float) -> float:\n    return math.fabs(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_isnan() {
    let code = "import math\ndef f(x: float) -> bool:\n    return math.isnan(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_isinf() {
    let code = "import math\ndef f(x: float) -> bool:\n    return math.isinf(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_pi_constant() {
    let code = "import math\ndef f() -> float:\n    return math.pi";
    assert!(transpile_ok(code));
}

#[test]
fn test_math_e_constant() {
    let code = "import math\ndef f() -> float:\n    return math.e";
    assert!(transpile_ok(code));
}

// ============ itertools module ============

#[test]
fn test_itertools_chain() {
    let code = "import itertools\ndef f(a: list, b: list):\n    return list(itertools.chain(a, b))";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_zip_longest() {
    let code = "import itertools\ndef f(a: list, b: list):\n    return list(itertools.zip_longest(a, b))";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_product() {
    let code = "import itertools\ndef f(a: list, b: list):\n    return list(itertools.product(a, b))";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_permutations() {
    let code = "import itertools\ndef f(items: list):\n    return list(itertools.permutations(items))";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_combinations() {
    let code = "import itertools\ndef f(items: list, r: int):\n    return list(itertools.combinations(items, r))";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_count() {
    let code = "import itertools\ndef f():\n    return itertools.count()";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_cycle() {
    let code = "import itertools\ndef f(items: list):\n    return itertools.cycle(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_repeat() {
    let code = "import itertools\ndef f(x: int):\n    return itertools.repeat(x, 5)";
    assert!(transpile_ok(code));
}

// ============ collections module ============

#[test]
fn test_collections_counter() {
    let code = "from collections import Counter\ndef f(items: list):\n    return Counter(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_collections_defaultdict() {
    let code = "from collections import defaultdict\ndef f():\n    return defaultdict(list)";
    assert!(transpile_ok(code));
}

#[test]
fn test_collections_deque() {
    let code = "from collections import deque\ndef f(items: list):\n    return deque(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_collections_ordereddict() {
    let code = "from collections import OrderedDict\ndef f():\n    return OrderedDict()";
    assert!(transpile_ok(code));
}

// ============ functools module ============

#[test]
fn test_functools_reduce() {
    let code = "from functools import reduce\ndef f(items: list) -> int:\n    return reduce(lambda x, y: x + y, items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_functools_partial() {
    let code = "from functools import partial\ndef add(x: int, y: int) -> int:\n    return x + y\ndef f():\n    add5 = partial(add, 5)";
    assert!(transpile_ok(code));
}

// ============ sys module ============

#[test]
fn test_sys_exit() {
    let code = "import sys\ndef f():\n    sys.exit(0)";
    assert!(transpile_ok(code));
}

#[test]
fn test_sys_exit_message() {
    let code = "import sys\ndef f():\n    sys.exit('Error')";
    assert!(transpile_ok(code));
}

#[test]
fn test_sys_argv() {
    let code = "import sys\ndef f():\n    return sys.argv";
    assert!(transpile_ok(code));
}

// ============ urllib module ============

#[test]
fn test_urllib_parse_urlparse() {
    let code = "from urllib.parse import urlparse\ndef f(url: str):\n    return urlparse(url)";
    assert!(transpile_ok(code));
}

#[test]
fn test_urllib_parse_urlencode() {
    let code = "from urllib.parse import urlencode\ndef f(params: dict) -> str:\n    return urlencode(params)";
    assert!(transpile_ok(code));
}

#[test]
fn test_urllib_parse_quote() {
    let code = "from urllib.parse import quote\ndef f(s: str) -> str:\n    return quote(s)";
    assert!(transpile_ok(code));
}

#[test]
fn test_urllib_parse_unquote() {
    let code = "from urllib.parse import unquote\ndef f(s: str) -> str:\n    return unquote(s)";
    assert!(transpile_ok(code));
}

// ============ copy module ============

#[test]
fn test_copy_copy() {
    let code = "import copy\ndef f(x: list):\n    return copy.copy(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_copy_deepcopy() {
    let code = "import copy\ndef f(x: list):\n    return copy.deepcopy(x)";
    assert!(transpile_ok(code));
}

// ============ tempfile module ============

#[test]
fn test_tempfile_namedtemporaryfile() {
    let code = "import tempfile\ndef f():\n    return tempfile.NamedTemporaryFile()";
    assert!(transpile_ok(code));
}

#[test]
fn test_tempfile_mkdtemp() {
    let code = "import tempfile\ndef f() -> str:\n    return tempfile.mkdtemp()";
    assert!(transpile_ok(code));
}

#[test]
fn test_tempfile_gettempdir() {
    let code = "import tempfile\ndef f() -> str:\n    return tempfile.gettempdir()";
    assert!(transpile_ok(code));
}

// ============ shutil module ============

#[test]
fn test_shutil_copy() {
    let code = "import shutil\ndef f(src: str, dst: str):\n    shutil.copy(src, dst)";
    assert!(transpile_ok(code));
}

#[test]
fn test_shutil_copy2() {
    let code = "import shutil\ndef f(src: str, dst: str):\n    shutil.copy2(src, dst)";
    assert!(transpile_ok(code));
}

#[test]
fn test_shutil_copytree() {
    let code = "import shutil\ndef f(src: str, dst: str):\n    shutil.copytree(src, dst)";
    assert!(transpile_ok(code));
}

#[test]
fn test_shutil_move() {
    let code = "import shutil\ndef f(src: str, dst: str):\n    shutil.move(src, dst)";
    assert!(transpile_ok(code));
}

#[test]
fn test_shutil_rmtree() {
    let code = "import shutil\ndef f(path: str):\n    shutil.rmtree(path)";
    assert!(transpile_ok(code));
}

#[test]
fn test_shutil_which() {
    let code = "import shutil\ndef f(cmd: str):\n    return shutil.which(cmd)";
    assert!(transpile_ok(code));
}

// ============ subprocess module ============

#[test]
fn test_subprocess_run() {
    let code = "import subprocess\ndef f(cmd: list):\n    return subprocess.run(cmd)";
    assert!(transpile_ok(code));
}

#[test]
fn test_subprocess_run_capture() {
    let code = "import subprocess\ndef f(cmd: list):\n    return subprocess.run(cmd, capture_output=True)";
    assert!(transpile_ok(code));
}

#[test]
fn test_subprocess_run_text() {
    let code = "import subprocess\ndef f(cmd: list):\n    return subprocess.run(cmd, text=True)";
    assert!(transpile_ok(code));
}

#[test]
fn test_subprocess_check_output() {
    let code = "import subprocess\ndef f(cmd: list):\n    return subprocess.check_output(cmd)";
    assert!(transpile_ok(code));
}

#[test]
fn test_subprocess_popen() {
    let code = "import subprocess\ndef f(cmd: list):\n    return subprocess.Popen(cmd)";
    assert!(transpile_ok(code));
}

// ============ string methods on method calls ============

#[test]
fn test_str_count() {
    let code = "def f(s: str) -> int:\n    return s.count('a')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_find() {
    let code = "def f(s: str) -> int:\n    return s.find('a')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rfind() {
    let code = "def f(s: str) -> int:\n    return s.rfind('a')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_index() {
    let code = "def f(s: str) -> int:\n    return s.index('a')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rindex() {
    let code = "def f(s: str) -> int:\n    return s.rindex('a')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isalpha() {
    let code = "def f(s: str) -> bool:\n    return s.isalpha()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isdigit() {
    let code = "def f(s: str) -> bool:\n    return s.isdigit()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isalnum() {
    let code = "def f(s: str) -> bool:\n    return s.isalnum()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isspace() {
    let code = "def f(s: str) -> bool:\n    return s.isspace()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isupper() {
    let code = "def f(s: str) -> bool:\n    return s.isupper()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_islower() {
    let code = "def f(s: str) -> bool:\n    return s.islower()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_istitle() {
    let code = "def f(s: str) -> bool:\n    return s.istitle()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_center() {
    let code = "def f(s: str) -> str:\n    return s.center(20)";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_ljust() {
    let code = "def f(s: str) -> str:\n    return s.ljust(20)";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rjust() {
    let code = "def f(s: str) -> str:\n    return s.rjust(20)";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_zfill() {
    let code = "def f(s: str) -> str:\n    return s.zfill(10)";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_swapcase() {
    let code = "def f(s: str) -> str:\n    return s.swapcase()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_capitalize() {
    let code = "def f(s: str) -> str:\n    return s.capitalize()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_title() {
    let code = "def f(s: str) -> str:\n    return s.title()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_encode() {
    let code = "def f(s: str) -> bytes:\n    return s.encode()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_encode_utf8() {
    let code = "def f(s: str) -> bytes:\n    return s.encode('utf-8')";
    assert!(transpile_ok(code));
}

// ============ bytes methods ============

#[test]
fn test_bytes_decode() {
    let code = "def f(b: bytes) -> str:\n    return b.decode()";
    assert!(transpile_ok(code));
}

#[test]
fn test_bytes_decode_utf8() {
    let code = "def f(b: bytes) -> str:\n    return b.decode('utf-8')";
    assert!(transpile_ok(code));
}

// ============ list methods ============

#[test]
fn test_list_index() {
    let code = "def f(items: list, x: int) -> int:\n    return items.index(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_count() {
    let code = "def f(items: list, x: int) -> int:\n    return items.count(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_insert() {
    let code = "def f(items: list, i: int, x: int):\n    items.insert(i, x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_remove() {
    let code = "def f(items: list, x: int):\n    items.remove(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_reverse() {
    let code = "def f(items: list):\n    items.reverse()";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_copy() {
    let code = "def f(items: list):\n    return items.copy()";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_clear() {
    let code = "def f(items: list):\n    items.clear()";
    assert!(transpile_ok(code));
}

// ============ dict methods ============

#[test]
fn test_dict_keys() {
    let code = "def f(d: dict):\n    return d.keys()";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_values() {
    let code = "def f(d: dict):\n    return d.values()";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_items() {
    let code = "def f(d: dict):\n    return d.items()";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_get() {
    let code = "def f(d: dict, key: str):\n    return d.get(key)";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_get_default() {
    let code = "def f(d: dict, key: str):\n    return d.get(key, 'default')";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_pop() {
    let code = "def f(d: dict, key: str):\n    return d.pop(key)";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_pop_default() {
    let code = "def f(d: dict, key: str):\n    return d.pop(key, None)";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_setdefault() {
    let code = "def f(d: dict, key: str, val: int):\n    return d.setdefault(key, val)";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_update() {
    let code = "def f(d1: dict, d2: dict):\n    d1.update(d2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_clear() {
    let code = "def f(d: dict):\n    d.clear()";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_copy() {
    let code = "def f(d: dict):\n    return d.copy()";
    assert!(transpile_ok(code));
}

// ============ set methods ============

#[test]
fn test_set_add() {
    let code = "def f(s: set, x: int):\n    s.add(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_remove() {
    let code = "def f(s: set, x: int):\n    s.remove(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_discard() {
    let code = "def f(s: set, x: int):\n    s.discard(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_pop() {
    let code = "def f(s: set):\n    return s.pop()";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_clear() {
    let code = "def f(s: set):\n    s.clear()";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_union() {
    let code = "def f(s1: set, s2: set):\n    return s1.union(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_intersection() {
    let code = "def f(s1: set, s2: set):\n    return s1.intersection(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_difference() {
    let code = "def f(s1: set, s2: set):\n    return s1.difference(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_symmetric_difference() {
    let code = "def f(s1: set, s2: set):\n    return s1.symmetric_difference(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_issubset() {
    let code = "def f(s1: set, s2: set) -> bool:\n    return s1.issubset(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_issuperset() {
    let code = "def f(s1: set, s2: set) -> bool:\n    return s1.issuperset(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_isdisjoint() {
    let code = "def f(s1: set, s2: set) -> bool:\n    return s1.isdisjoint(s2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_set_copy() {
    let code = "def f(s: set):\n    return s.copy()";
    assert!(transpile_ok(code));
}

// ============ builtin functions edge cases ============

#[test]
fn test_abs_float() {
    let code = "def f(x: float) -> float:\n    return abs(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_round_no_digits() {
    let code = "def f(x: float) -> int:\n    return round(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_round_with_digits() {
    let code = "def f(x: float) -> float:\n    return round(x, 2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_divmod() {
    let code = "def f(a: int, b: int):\n    return divmod(a, b)";
    assert!(transpile_ok(code));
}

#[test]
fn test_pow_two_args() {
    let code = "def f(base: int, exp: int) -> int:\n    return pow(base, exp)";
    assert!(transpile_ok(code));
}

#[test]
fn test_pow_three_args() {
    let code = "def f(base: int, exp: int, mod: int) -> int:\n    return pow(base, exp, mod)";
    assert!(transpile_ok(code));
}

#[test]
fn test_hex() {
    let code = "def f(n: int) -> str:\n    return hex(n)";
    assert!(transpile_ok(code));
}

#[test]
fn test_oct() {
    let code = "def f(n: int) -> str:\n    return oct(n)";
    assert!(transpile_ok(code));
}

#[test]
fn test_bin() {
    let code = "def f(n: int) -> str:\n    return bin(n)";
    assert!(transpile_ok(code));
}

#[test]
fn test_ord() {
    let code = "def f(c: str) -> int:\n    return ord(c)";
    assert!(transpile_ok(code));
}

#[test]
fn test_chr() {
    let code = "def f(n: int) -> str:\n    return chr(n)";
    assert!(transpile_ok(code));
}

#[test]
fn test_isinstance() {
    let code = "def f(x) -> bool:\n    return isinstance(x, int)";
    assert!(transpile_ok(code));
}

#[test]
fn test_type() {
    let code = "def f(x):\n    return type(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_id() {
    let code = "def f(x) -> int:\n    return id(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_hash() {
    let code = "def f(x) -> int:\n    return hash(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_callable() {
    let code = "def f(x) -> bool:\n    return callable(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_iter() {
    let code = "def f(items: list):\n    return iter(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_next() {
    let code = "def f(it):\n    return next(it)";
    assert!(transpile_ok(code));
}

#[test]
fn test_next_with_default() {
    let code = "def f(it):\n    return next(it, None)";
    assert!(transpile_ok(code));
}

#[test]
fn test_all() {
    let code = "def f(items: list) -> bool:\n    return all(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_any() {
    let code = "def f(items: list) -> bool:\n    return any(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_filter() {
    let code = "def f(items: list):\n    return list(filter(None, items))";
    assert!(transpile_ok(code));
}

#[test]
fn test_filter_lambda() {
    let code = "def f(items: list):\n    return list(filter(lambda x: x > 0, items))";
    assert!(transpile_ok(code));
}

#[test]
fn test_map() {
    let code = "def f(items: list):\n    return list(map(str, items))";
    assert!(transpile_ok(code));
}

#[test]
fn test_map_lambda() {
    let code = "def f(items: list):\n    return list(map(lambda x: x * 2, items))";
    assert!(transpile_ok(code));
}

#[test]
fn test_zip() {
    let code = "def f(a: list, b: list):\n    return list(zip(a, b))";
    assert!(transpile_ok(code));
}

#[test]
fn test_zip_three() {
    let code = "def f(a: list, b: list, c: list):\n    return list(zip(a, b, c))";
    assert!(transpile_ok(code));
}

#[test]
fn test_reversed() {
    let code = "def f(items: list):\n    return list(reversed(items))";
    assert!(transpile_ok(code));
}

#[test]
fn test_sorted_reverse() {
    let code = "def f(items: list):\n    return sorted(items, reverse=True)";
    assert!(transpile_ok(code));
}

#[test]
fn test_sorted_key() {
    let code = "def f(items: list):\n    return sorted(items, key=lambda x: x[0])";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice() {
    let code = "def f(items: list):\n    return items[1:5]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_step() {
    let code = "def f(items: list):\n    return items[::2]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_negative() {
    let code = "def f(items: list):\n    return items[-3:]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_reverse() {
    let code = "def f(items: list):\n    return items[::-1]";
    assert!(transpile_ok(code));
}

// ============ format string variations ============

#[test]
fn test_fstring_simple() {
    let code = "def f(x: int) -> str:\n    return f'{x}'";
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_expression() {
    let code = "def f(x: int) -> str:\n    return f'{x + 1}'";
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_format_spec() {
    let code = "def f(x: float) -> str:\n    return f'{x:.2f}'";
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_multiple() {
    let code = "def f(x: int, y: int) -> str:\n    return f'{x} + {y} = {x + y}'";
    assert!(transpile_ok(code));
}

#[test]
fn test_format_method() {
    let code = "def f(x: int) -> str:\n    return '{}'.format(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_format_method_named() {
    let code = "def f(x: int) -> str:\n    return '{val}'.format(val=x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_percent_format() {
    let code = "def f(x: int) -> str:\n    return '%d' % x";
    assert!(transpile_ok(code));
}

#[test]
fn test_percent_format_multiple() {
    let code = "def f(x: int, y: int) -> str:\n    return '%d + %d' % (x, y)";
    assert!(transpile_ok(code));
}
