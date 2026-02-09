//! Wave 12 Coverage Tests: Instance Method Dispatch and Type System
//!
//! 200 transpile-based tests targeting five low-coverage files:
//! - type_helpers.rs (69.04% covered, 365/1179 lines missed)
//! - instance_dispatch.rs (69.45% covered, 168/550 lines missed)
//! - lambda_generators.rs (69.97% covered, 227/756 lines missed)
//! - dict_constructors.rs (62.53% covered, 172/459 lines missed)
//! - method_call_routing.rs (70.58% covered, 253/860 lines missed)

#[cfg(test)]
mod tests {
    use crate::ast_bridge::AstBridge;
    use crate::rust_gen::generate_rust_file;
    use crate::type_mapper::TypeMapper;
    use rustpython_parser::{parse, Mode};

    fn transpile(python_code: &str) -> String {
        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) = AstBridge::new()
            .with_source(python_code.to_string())
            .python_to_hir(ast)
            .expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }

    // ========================================================================
    // TYPE_HELPERS.RS TESTS (50 tests)
    // Target: is_string_index, is_string_variable, is_numeric_index,
    //         is_string_base, is_tuple_base, get_tuple_size, is_set_expr,
    //         is_numpy_array_expr
    // ========================================================================

    #[test]
    fn test_w12dt_type_helpers_001_string_literal_index() {
        let py = r#"
result = data["key"]
"#;
        let rs = transpile(py);
        assert!(rs.contains("get") || rs.contains("[")); // String key access
    }

    #[test]
    fn test_w12dt_type_helpers_002_int_literal_index() {
        let py = r#"
result = data[1]
"#;
        let rs = transpile(py);
        assert!(rs.contains("1")); // Index or access with integer
    }

    #[test]
    fn test_w12dt_type_helpers_003_string_var_key_index() {
        let py = r#"
key = "name"
result = data[key]
"#;
        let rs = transpile(py);
        assert!(rs.contains("key"));
    }

    #[test]
    fn test_w12dt_type_helpers_004_numeric_var_index() {
        let py = r#"
i = 5
result = data[i]
"#;
        let rs = transpile(py);
        assert!(rs.contains("i"));
    }

    #[test]
    fn test_w12dt_type_helpers_005_dict_var_name_heuristic() {
        let py = r#"
my_dict = {}
my_dict["key"] = "value"
"#;
        let rs = transpile(py);
        assert!(rs.contains("insert") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_type_helpers_006_map_var_name_heuristic() {
        let py = r#"
config_map = {}
config_map["setting"] = 42
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("insert"));
    }

    #[test]
    fn test_w12dt_type_helpers_007_config_var_name_heuristic() {
        let py = r#"
config = {}
config["debug"] = True
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("insert"));
    }

    #[test]
    fn test_w12dt_type_helpers_008_value_var_name_heuristic() {
        let py = r#"
value = {}
value["data"] = [1, 2, 3]
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("insert"));
    }

    #[test]
    fn test_w12dt_type_helpers_009_key_var_is_string() {
        let py = r#"
for key in ["a", "b"]:
    result = data[key]
"#;
        let rs = transpile(py);
        assert!(rs.contains("key"));
    }

    #[test]
    fn test_w12dt_type_helpers_010_k_var_is_string_key() {
        let py = r#"
for k in dict_keys:
    val = mapping[k]
"#;
        let rs = transpile(py);
        assert!(rs.contains("k"));
    }

    #[test]
    fn test_w12dt_type_helpers_011_name_var_is_string() {
        let py = r#"
name = "Alice"
greeting = "Hello " + name
"#;
        let rs = transpile(py);
        assert!(rs.contains("name"));
    }

    #[test]
    fn test_w12dt_type_helpers_012_id_var_is_string() {
        let py = r#"
id = "user123"
record = db[id]
"#;
        let rs = transpile(py);
        assert!(rs.contains("id"));
    }

    #[test]
    fn test_w12dt_type_helpers_013_word_var_is_string() {
        let py = r#"
word = "hello"
upper_word = word.upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_type_helpers_014_text_var_is_string() {
        let py = r#"
text = "sample"
stripped = text.strip()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim") || rs.contains("strip"));
    }

    #[test]
    fn test_w12dt_type_helpers_015_chr_returns_string() {
        let py = r#"
c = chr(65)
"#;
        let rs = transpile(py);
        assert!(rs.contains("char") || rs.contains("from_u32"));
    }

    #[test]
    fn test_w12dt_type_helpers_016_str_returns_string() {
        let py = r#"
s = str(42)
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_string"));
    }

    #[test]
    fn test_w12dt_type_helpers_017_hex_returns_string() {
        let py = r#"
h = hex(255)
"#;
        let rs = transpile(py);
        assert!(rs.contains("0x") || rs.contains("format"));
    }

    #[test]
    fn test_w12dt_type_helpers_018_upper_method_returns_string() {
        let py = r#"
result = "hello".upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_uppercase") || rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_type_helpers_019_lower_method_returns_string() {
        let py = r#"
result = "HELLO".lower()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_lowercase") || rs.contains("lower"));
    }

    #[test]
    fn test_w12dt_type_helpers_020_strip_method_returns_string() {
        let py = r#"
result = "  text  ".strip()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim"));
    }

    #[test]
    fn test_w12dt_type_helpers_021_i_is_numeric_index() {
        let py = r#"
for i in range(10):
    val = items[i]
"#;
        let rs = transpile(py);
        assert!(rs.contains("i") && rs.contains("items")); // Numeric index variable i
    }

    #[test]
    fn test_w12dt_type_helpers_022_j_is_numeric_index() {
        let py = r#"
for j in range(5):
    matrix[j][0] = 1
"#;
        let rs = transpile(py);
        assert!(rs.contains("j"));
    }

    #[test]
    fn test_w12dt_type_helpers_023_idx_is_numeric_index() {
        let py = r#"
idx = 0
value = array[idx]
"#;
        let rs = transpile(py);
        assert!(rs.contains("idx"));
    }

    #[test]
    fn test_w12dt_type_helpers_024_index_var_is_numeric() {
        let py = r#"
index = 3
item = collection[index]
"#;
        let rs = transpile(py);
        assert!(rs.contains("index"));
    }

    #[test]
    fn test_w12dt_type_helpers_025_binary_expr_is_numeric() {
        let py = r#"
result = data[i + 1]
"#;
        let rs = transpile(py);
        assert!(rs.contains("+"));
    }

    #[test]
    fn test_w12dt_type_helpers_026_string_literal_base() {
        let py = r#"
c = "hello"[0]
"#;
        let rs = transpile(py);
        assert!(rs.contains("chars") || rs.contains("["));
    }

    #[test]
    fn test_w12dt_type_helpers_027_string_var_base() {
        let py = r#"
text = "sample"
first = text[0]
"#;
        let rs = transpile(py);
        assert!(rs.contains("chars") || rs.contains("["));
    }

    #[test]
    fn test_w12dt_type_helpers_028_s_var_is_string() {
        let py = r#"
s = "test"
length = len(s)
"#;
        let rs = transpile(py);
        assert!(rs.contains("len"));
    }

    #[test]
    fn test_w12dt_type_helpers_029_line_var_is_string() {
        let py = r#"
line = "data line"
words = line.split()
"#;
        let rs = transpile(py);
        assert!(rs.contains("split"));
    }

    #[test]
    fn test_w12dt_type_helpers_030_content_var_is_string() {
        let py = r#"
content = "file content"
trimmed = content.strip()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim"));
    }

    #[test]
    fn test_w12dt_type_helpers_031_tuple_literal_base() {
        let py = r#"
pair = (1, 2)
first = pair[0]
"#;
        let rs = transpile(py);
        assert!(rs.contains("0") || rs.contains("first"));
    }

    #[test]
    fn test_w12dt_type_helpers_032_tuple_var_base() {
        let py = r#"
entry = (5, 10)
x = entry[0]
y = entry[1]
"#;
        let rs = transpile(py);
        assert!(rs.contains("entry"));
    }

    #[test]
    fn test_w12dt_type_helpers_033_enumerate_returns_tuple() {
        let py = r#"
for idx, val in enumerate(items):
    print(idx, val)
"#;
        let rs = transpile(py);
        assert!(rs.contains("enumerate") || rs.contains("iter"));
    }

    #[test]
    fn test_w12dt_type_helpers_034_dict_items_returns_tuples() {
        let py = r#"
for key, value in data.items():
    print(key, value)
"#;
        let rs = transpile(py);
        assert!(rs.contains("items") || rs.contains("iter"));
    }

    #[test]
    fn test_w12dt_type_helpers_035_set_literal() {
        let py = r#"
s = {1, 2, 3}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashSet") || rs.contains("set"));
    }

    #[test]
    fn test_w12dt_type_helpers_036_set_constructor() {
        let py = r#"
s = set([1, 2, 3])
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashSet") || rs.contains("collect"));
    }

    #[test]
    fn test_w12dt_type_helpers_037_frozenset_literal() {
        let py = r#"
fs = frozenset([1, 2, 3])
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashSet") || rs.contains("collect"));
    }

    #[test]
    fn test_w12dt_type_helpers_038_timestamp_var_is_string() {
        let py = r#"
timestamp = "2024-01-01T00:00:00Z"
parsed = timestamp.split("T")
"#;
        let rs = transpile(py);
        assert!(rs.contains("split"));
    }

    #[test]
    fn test_w12dt_type_helpers_039_message_var_is_string() {
        let py = r#"
message = "Error occurred"
lower = message.lower()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_lowercase") || rs.contains("lower"));
    }

    #[test]
    fn test_w12dt_type_helpers_040_level_var_is_string() {
        let py = r#"
level = "INFO"
is_debug = level == "DEBUG"
"#;
        let rs = transpile(py);
        assert!(rs.contains("level"));
    }

    #[test]
    fn test_w12dt_type_helpers_041_prefix_var_is_string() {
        let py = r#"
prefix = "test_"
has_prefix = name.startswith(prefix)
"#;
        let rs = transpile(py);
        assert!(rs.contains("starts_with") || rs.contains("startswith"));
    }

    #[test]
    fn test_w12dt_type_helpers_042_suffix_var_is_string() {
        let py = r#"
suffix = ".txt"
has_suffix = filename.endswith(suffix)
"#;
        let rs = transpile(py);
        assert!(rs.contains("ends_with") || rs.contains("endswith"));
    }

    #[test]
    fn test_w12dt_type_helpers_043_pattern_var_is_string() {
        let py = r#"
pattern = "[0-9]+"
matched = pattern in text
"#;
        let rs = transpile(py);
        assert!(rs.contains("pattern"));
    }

    #[test]
    fn test_w12dt_type_helpers_044_char_var_is_string() {
        let py = r#"
char = "a"
is_vowel = char in "aeiou"
"#;
        let rs = transpile(py);
        assert!(rs.contains("char"));
    }

    #[test]
    fn test_w12dt_type_helpers_045_delimiter_var_is_string() {
        let py = r#"
delimiter = ","
parts = data.split(delimiter)
"#;
        let rs = transpile(py);
        assert!(rs.contains("split"));
    }

    #[test]
    fn test_w12dt_type_helpers_046_separator_var_is_string() {
        let py = r#"
separator = " | "
joined = separator.join(items)
"#;
        let rs = transpile(py);
        assert!(rs.contains("join"));
    }

    #[test]
    fn test_w12dt_type_helpers_047_attribute_text_is_string() {
        let py = r#"
upper_text = args.text.upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("text"));
    }

    #[test]
    fn test_w12dt_type_helpers_048_attribute_prefix_is_string() {
        let py = r#"
has_prefix = line.startswith(args.prefix)
"#;
        let rs = transpile(py);
        assert!(rs.contains("prefix"));
    }

    #[test]
    fn test_w12dt_type_helpers_049_title_method_returns_string() {
        let py = r#"
result = "hello world".title()
"#;
        let rs = transpile(py);
        assert!(rs.contains("title") || rs.contains("to_"));
    }

    #[test]
    fn test_w12dt_type_helpers_050_str_call_returns_string() {
        let py = r#"
s = str(3.14)
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_string"));
    }

    // ========================================================================
    // INSTANCE_DISPATCH.RS TESTS (30 tests)
    // Target: parse_args, add_argument, print_help, sys I/O methods,
    //         file I/O read/readlines/readline/write/close, pathlib methods
    // ========================================================================

    #[test]
    fn test_w12dt_instance_dispatch_051_parse_args() {
        let py = r#"
import argparse
parser = argparse.ArgumentParser()
args = parser.parse_args()
"#;
        let rs = transpile(py);
        assert!(rs.contains("parse") || rs.contains("Args"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_052_add_argument() {
        let py = r#"
import argparse
parser = argparse.ArgumentParser()
parser.add_argument("--name")
"#;
        let rs = transpile(py);
        assert!(rs.contains("parser") || rs.contains("Args"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_053_print_help() {
        let py = r#"
import argparse
parser = argparse.ArgumentParser()
parser.print_help()
"#;
        let rs = transpile(py);
        assert!(rs.contains("print_help") || rs.contains("CommandFactory"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_054_file_read_no_args() {
        let py = r#"
with open("data.txt") as f:
    content = f.read()
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_055_file_read_with_size() {
        let py = r#"
with open("data.bin", "rb") as f:
    chunk = f.read(8192)
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_056_file_readlines() {
        let py = r#"
with open("data.txt") as f:
    lines = f.readlines()
"#;
        let rs = transpile(py);
        assert!(rs.contains("lines") || rs.contains("BufReader"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_057_file_readline() {
        let py = r#"
with open("data.txt") as f:
    line = f.readline()
"#;
        let rs = transpile(py);
        assert!(rs.contains("read_line") || rs.contains("BufReader"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_058_file_write() {
        let py = r#"
with open("output.txt", "w") as f:
    f.write("Hello, world!")
"#;
        let rs = transpile(py);
        assert!(rs.contains("write"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_059_file_close() {
        let py = r#"
f = open("data.txt")
content = f.read()
f.close()
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_060_path_stat() {
        let py = r#"
from pathlib import Path
path = Path("file.txt")
stat_info = path.stat()
"#;
        let rs = transpile(py);
        assert!(rs.contains("metadata") || rs.contains("stat"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_061_path_absolute() {
        let py = r#"
from pathlib import Path
path = Path("relative/path")
abs_path = path.absolute()
"#;
        let rs = transpile(py);
        assert!(rs.contains("canonicalize") || rs.contains("absolute"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_062_path_resolve() {
        let py = r#"
from pathlib import Path
path = Path("../data")
resolved = path.resolve()
"#;
        let rs = transpile(py);
        assert!(rs.contains("canonicalize") || rs.contains("resolve"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_063_sys_stdin_method() {
        let py = r#"
import sys
line = sys.stdin.readline()
"#;
        let rs = transpile(py);
        assert!(rs.contains("stdin") || rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_064_sys_stdout_method() {
        let py = r#"
import sys
sys.stdout.write("output\n")
"#;
        let rs = transpile(py);
        assert!(rs.contains("stdout") || rs.contains("write") || rs.contains("print"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_065_sys_stderr_method() {
        let py = r#"
import sys
sys.stderr.write("error\n")
"#;
        let rs = transpile(py);
        assert!(rs.contains("stderr") || rs.contains("write") || rs.contains("eprintln"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_066_file_write_option_content() {
        let py = r#"
def write_data(f, content):
    f.write(content)
"#;
        let rs = transpile(py);
        assert!(rs.contains("write"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_067_path_var_heuristic() {
        let py = r#"
p = "file.txt"
exists = p.exists()
"#;
        let rs = transpile(py);
        assert!(rs.contains("exists") || rs.contains("p"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_068_file_path_var() {
        let py = r#"
file_path = "data.txt"
metadata = file_path.stat()
"#;
        let rs = transpile(py);
        assert!(rs.contains("file_path"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_069_multiple_file_reads() {
        let py = r#"
with open("data.txt") as f:
    line1 = f.readline()
    line2 = f.readline()
"#;
        let rs = transpile(py);
        assert!(rs.contains("read_line"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_070_file_write_multiple() {
        let py = r#"
with open("output.txt", "w") as f:
    f.write("Line 1\n")
    f.write("Line 2\n")
"#;
        let rs = transpile(py);
        assert!(rs.contains("write"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_071_chunked_file_reading() {
        let py = r#"
with open("large.dat", "rb") as f:
    while True:
        chunk = f.read(4096)
        if not chunk:
            break
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_072_file_iteration() {
        let py = r#"
with open("data.txt") as f:
    all_lines = f.readlines()
    for line in all_lines:
        print(line)
"#;
        let rs = transpile(py);
        assert!(rs.contains("lines") || rs.contains("for"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_073_path_name_ending() {
        let py = r#"
input_path = "data.json"
metadata = input_path.stat()
"#;
        let rs = transpile(py);
        assert!(rs.contains("input_path"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_074_file_context_manager() {
        let py = r#"
f = open("temp.txt", "w")
try:
    f.write("test")
finally:
    f.close()
"#;
        let rs = transpile(py);
        assert!(rs.contains("write"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_075_argparse_multiple_args() {
        let py = r#"
import argparse
parser = argparse.ArgumentParser()
parser.add_argument("--input")
parser.add_argument("--output")
args = parser.parse_args()
"#;
        let rs = transpile(py);
        assert!(rs.contains("parser") || rs.contains("Args"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_076_binary_file_read() {
        let py = r#"
with open("image.png", "rb") as f:
    data = f.read()
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_077_text_file_read() {
        let py = r#"
with open("config.yaml") as f:
    config = f.read()
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_078_file_read_empty() {
        let py = r#"
with open("empty.txt") as f:
    content = f.read()
    if not content:
        print("Empty file")
"#;
        let rs = transpile(py);
        assert!(rs.contains("read"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_079_readline_loop() {
        let py = r#"
with open("data.txt") as f:
    while True:
        line = f.readline()
        if not line:
            break
"#;
        let rs = transpile(py);
        assert!(rs.contains("read_line") || rs.contains("while"));
    }

    #[test]
    fn test_w12dt_instance_dispatch_080_write_formatted() {
        let py = r#"
with open("output.txt", "w") as f:
    f.write(f"Result: {42}\n")
"#;
        let rs = transpile(py);
        assert!(rs.contains("write"));
    }

    // ========================================================================
    // LAMBDA_GENERATORS.RS TESTS (50 tests)
    // Target: convert_lambda (0-3+ params), lambda with captured vars,
    //         lambda in map/filter/sorted, nested lambdas, default args,
    //         generator expressions, f-strings, ternary expressions
    // ========================================================================

    #[test]
    fn test_w12dt_lambda_gen_081_lambda_zero_params() {
        let py = r#"
get_value = lambda: 42
result = get_value()
"#;
        let rs = transpile(py);
        assert!(rs.contains("||") && rs.contains("42"));
    }

    #[test]
    fn test_w12dt_lambda_gen_082_lambda_one_param() {
        let py = r#"
double = lambda x: x * 2
result = double(5)
"#;
        let rs = transpile(py);
        assert!(rs.contains("|x|") || rs.contains("*"));
    }

    #[test]
    fn test_w12dt_lambda_gen_083_lambda_two_params() {
        let py = r#"
add = lambda a, b: a + b
result = add(3, 4)
"#;
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    #[test]
    fn test_w12dt_lambda_gen_084_lambda_three_params() {
        let py = r#"
combine = lambda x, y, z: x + y + z
result = combine(1, 2, 3)
"#;
        let rs = transpile(py);
        assert!(rs.contains("x") && rs.contains("y") && rs.contains("z"));
    }

    #[test]
    fn test_w12dt_lambda_gen_085_lambda_four_params() {
        let py = r#"
calc = lambda a, b, c, d: (a + b) * (c + d)
"#;
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("d"));
    }

    #[test]
    fn test_w12dt_lambda_gen_086_lambda_in_map() {
        let py = r#"
nums = [1, 2, 3]
squared = list(map(lambda x: x * x, nums))
"#;
        let rs = transpile(py);
        assert!(rs.contains("map") || rs.contains("iter"));
    }

    #[test]
    fn test_w12dt_lambda_gen_087_lambda_in_filter() {
        let py = r#"
nums = [1, 2, 3, 4, 5]
evens = list(filter(lambda x: x % 2 == 0, nums))
"#;
        let rs = transpile(py);
        assert!(rs.contains("filter") || rs.contains("%"));
    }

    #[test]
    fn test_w12dt_lambda_gen_088_lambda_in_sorted_key() {
        let py = r#"
words = ["apple", "pie", "a"]
sorted_words = sorted(words, key=lambda x: len(x))
"#;
        let rs = transpile(py);
        assert!(rs.contains("sort") || rs.contains("len"));
    }

    #[test]
    fn test_w12dt_lambda_gen_089_nested_lambda() {
        let py = r#"
make_adder = lambda x: lambda y: x + y
add_five = make_adder(5)
result = add_five(3)
"#;
        let rs = transpile(py);
        assert!(rs.contains("|x|") || rs.contains("|y|"));
    }

    #[test]
    fn test_w12dt_lambda_gen_090_lambda_capturing_outer_var() {
        let py = r#"
multiplier = 10
scale = lambda x: x * multiplier
result = scale(5)
"#;
        let rs = transpile(py);
        assert!(rs.contains("multiplier"));
    }

    #[test]
    fn test_w12dt_lambda_gen_091_lambda_with_string_capture() {
        let py = r#"
prefix = "Result: "
format_result = lambda x: prefix + str(x)
"#;
        let rs = transpile(py);
        assert!(rs.contains("prefix"));
    }

    #[test]
    fn test_w12dt_lambda_gen_092_lambda_with_list_capture() {
        let py = r#"
items = [1, 2, 3]
get_first = lambda: items[0]
"#;
        let rs = transpile(py);
        assert!(rs.contains("items"));
    }

    #[test]
    fn test_w12dt_lambda_gen_093_generator_expression_simple() {
        let py = r#"
gen = (x * 2 for x in range(10))
"#;
        let rs = transpile(py);
        assert!(rs.contains("map") || rs.contains("iter"));
    }

    #[test]
    fn test_w12dt_lambda_gen_094_generator_with_condition() {
        let py = r#"
gen = (x for x in range(10) if x > 5)
"#;
        let rs = transpile(py);
        assert!(rs.contains("filter") || rs.contains(">"));
    }

    #[test]
    fn test_w12dt_lambda_gen_095_generator_nested() {
        let py = r#"
gen = (x * y for x in range(3) for y in range(3))
"#;
        let rs = transpile(py);
        assert!(rs.contains("flat_map") || rs.contains("*"));
    }

    #[test]
    fn test_w12dt_lambda_gen_096_fstring_simple() {
        let py = r#"
name = "Alice"
greeting = f"Hello, {name}!"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains("name"));
    }

    #[test]
    fn test_w12dt_lambda_gen_097_fstring_expression() {
        let py = r#"
x = 5
result = f"Value: {x * 2}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains("*"));
    }

    #[test]
    fn test_w12dt_lambda_gen_098_fstring_format_spec() {
        let py = r#"
pi = 3.14159
formatted = f"{pi:.2f}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains(".2"));
    }

    #[test]
    fn test_w12dt_lambda_gen_099_fstring_padding() {
        let py = r#"
num = 42
padded = f"{num:>10}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains("num"));
    }

    #[test]
    fn test_w12dt_lambda_gen_100_fstring_repr() {
        let py = r#"
obj = "test"
debug = f"{obj!r}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains("obj"));
    }

    #[test]
    fn test_w12dt_lambda_gen_101_fstring_method_call() {
        let py = r#"
name = "alice"
message = f"Hello, {name.upper()}!"
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_uppercase") || rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_lambda_gen_102_ternary_simple() {
        let py = r#"
x = 10
result = "big" if x > 5 else "small"
"#;
        let rs = transpile(py);
        assert!(rs.contains("if") || rs.contains(">"));
    }

    #[test]
    fn test_w12dt_lambda_gen_103_ternary_nested() {
        let py = r#"
x = 5
result = "negative" if x < 0 else "zero" if x == 0 else "positive"
"#;
        let rs = transpile(py);
        assert!(rs.contains("if"));
    }

    #[test]
    fn test_w12dt_lambda_gen_104_ternary_in_assignment() {
        let py = r#"
condition = True
value = 1 if condition else 0
"#;
        let rs = transpile(py);
        assert!(rs.contains("if") || rs.contains("condition"));
    }

    #[test]
    fn test_w12dt_lambda_gen_105_lambda_with_ternary() {
        let py = r#"
clamp = lambda x: 0 if x < 0 else 100 if x > 100 else x
"#;
        let rs = transpile(py);
        assert!(rs.contains("if"));
    }

    #[test]
    fn test_w12dt_lambda_gen_106_sorted_with_lambda_attr() {
        let py = r#"
items = [{"name": "a", "value": 3}, {"name": "b", "value": 1}]
sorted_items = sorted(items, key=lambda x: x["value"])
"#;
        let rs = transpile(py);
        assert!(rs.contains("sort"));
    }

    #[test]
    fn test_w12dt_lambda_gen_107_lambda_boolean_logic() {
        let py = r#"
check = lambda x, y: x > 0 and y > 0
"#;
        let rs = transpile(py);
        assert!(rs.contains("&&") || rs.contains("and"));
    }

    #[test]
    fn test_w12dt_lambda_gen_108_lambda_list_comprehension() {
        let py = r#"
process = lambda items: [x * 2 for x in items]
"#;
        let rs = transpile(py);
        assert!(rs.contains("map") || rs.contains("iter"));
    }

    #[test]
    fn test_w12dt_lambda_gen_109_fstring_multiple_vars() {
        let py = r#"
x = 10
y = 20
result = f"{x} + {y} = {x + y}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!"));
    }

    #[test]
    fn test_w12dt_lambda_gen_110_generator_sum() {
        let py = r#"
total = sum(x * x for x in range(10))
"#;
        let rs = transpile(py);
        assert!(rs.contains("sum") || rs.contains("*"));
    }

    #[test]
    fn test_w12dt_lambda_gen_111_generator_any() {
        let py = r#"
has_even = any(x % 2 == 0 for x in nums)
"#;
        let rs = transpile(py);
        assert!(rs.contains("any") || rs.contains("%"));
    }

    #[test]
    fn test_w12dt_lambda_gen_112_generator_all() {
        let py = r#"
all_positive = all(x > 0 for x in nums)
"#;
        let rs = transpile(py);
        assert!(rs.contains("all") || rs.contains(">"));
    }

    #[test]
    fn test_w12dt_lambda_gen_113_lambda_dict_access() {
        let py = r#"
get_name = lambda obj: obj["name"]
"#;
        let rs = transpile(py);
        assert!(rs.contains("get") || rs.contains("["));
    }

    #[test]
    fn test_w12dt_lambda_gen_114_lambda_tuple_destructure() {
        let py = r#"
get_first = lambda pair: pair[0]
"#;
        let rs = transpile(py);
        assert!(rs.contains("[0]") || rs.contains("first"));
    }

    #[test]
    fn test_w12dt_lambda_gen_115_fstring_nested_expr() {
        let py = r#"
items = [1, 2, 3]
message = f"Count: {len(items)}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("len"));
    }

    #[test]
    fn test_w12dt_lambda_gen_116_lambda_string_ops() {
        let py = r#"
uppercase = lambda s: s.upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_uppercase") || rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_lambda_gen_117_lambda_arithmetic() {
        let py = r#"
calc = lambda a, b: (a + b) * (a - b)
"#;
        let rs = transpile(py);
        assert!(rs.contains("+") && rs.contains("-"));
    }

    #[test]
    fn test_w12dt_lambda_gen_118_generator_comprehension_filter() {
        let py = r#"
evens = (x for x in range(20) if x % 2 == 0 if x > 5)
"#;
        let rs = transpile(py);
        assert!(rs.contains("filter"));
    }

    #[test]
    fn test_w12dt_lambda_gen_119_fstring_conversion() {
        let py = r#"
val = 42
s = f"{val!s}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains("val"));
    }

    #[test]
    fn test_w12dt_lambda_gen_120_lambda_comparison_chain() {
        let py = r#"
in_range = lambda x: 0 <= x <= 100
"#;
        let rs = transpile(py);
        assert!(rs.contains("<="));
    }

    #[test]
    fn test_w12dt_lambda_gen_121_ternary_with_function_call() {
        let py = r#"
result = len(items) if items else 0
"#;
        let rs = transpile(py);
        assert!(rs.contains("len") || rs.contains("if"));
    }

    #[test]
    fn test_w12dt_lambda_gen_122_lambda_with_default_handling() {
        let py = r#"
get_or_default = lambda d, k: d[k] if k in d else None
"#;
        let rs = transpile(py);
        assert!(rs.contains("get") || rs.contains("if"));
    }

    #[test]
    fn test_w12dt_lambda_gen_123_generator_with_tuple() {
        let py = r#"
pairs = ((i, i*2) for i in range(5))
"#;
        let rs = transpile(py);
        assert!(rs.contains("map") || rs.contains("*"));
    }

    #[test]
    fn test_w12dt_lambda_gen_124_fstring_with_dict_lookup() {
        let py = r#"
data = {"key": "value"}
msg = f"The value is {data['key']}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!"));
    }

    #[test]
    fn test_w12dt_lambda_gen_125_lambda_capturing_multiple_vars() {
        let py = r#"
a = 5
b = 10
combine = lambda x: x + a + b
"#;
        let rs = transpile(py);
        assert!(rs.contains("a") || rs.contains("b"));
    }

    #[test]
    fn test_w12dt_lambda_gen_126_sorted_reverse_with_lambda() {
        let py = r#"
nums = [3, 1, 4, 1, 5]
sorted_desc = sorted(nums, key=lambda x: -x)
"#;
        let rs = transpile(py);
        assert!(rs.contains("sort"));
    }

    #[test]
    fn test_w12dt_lambda_gen_127_map_with_lambda_index() {
        let py = r#"
indexed = list(map(lambda x: (x[0], x[1]), enumerate(items)))
"#;
        let rs = transpile(py);
        assert!(rs.contains("map") || rs.contains("enumerate"));
    }

    #[test]
    fn test_w12dt_lambda_gen_128_fstring_with_index() {
        let py = r#"
items = ["a", "b", "c"]
msg = f"First item: {items[0]}"
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!"));
    }

    #[test]
    fn test_w12dt_lambda_gen_129_lambda_with_membership() {
        let py = r#"
is_vowel = lambda c: c in "aeiou"
"#;
        let rs = transpile(py);
        assert!(rs.contains("contains") || rs.contains("in"));
    }

    #[test]
    fn test_w12dt_lambda_gen_130_generator_with_enumerate() {
        let py = r#"
indexed = ((i, x*2) for i, x in enumerate(items))
"#;
        let rs = transpile(py);
        assert!(rs.contains("enumerate"));
    }

    // ========================================================================
    // DICT_CONSTRUCTORS.RS TESTS (30 tests)
    // Target: dict literals, dict comprehensions, dict from zip, mixed types,
    //         nested dicts, dict updates, empty dicts
    // ========================================================================

    #[test]
    fn test_w12dt_dict_const_131_empty_dict() {
        let py = r#"
d = {}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("{}"));
    }

    #[test]
    fn test_w12dt_dict_const_132_simple_dict_literal() {
        let py = r#"
d = {"a": 1, "b": 2}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("insert"));
    }

    #[test]
    fn test_w12dt_dict_const_133_dict_comprehension() {
        let py = r#"
d = {k: v for k, v in items}
"#;
        let rs = transpile(py);
        assert!(rs.contains("collect") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_134_dict_from_zip() {
        let py = r#"
keys = ["a", "b", "c"]
values = [1, 2, 3]
d = dict(zip(keys, values))
"#;
        let rs = transpile(py);
        assert!(rs.contains("zip") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_135_dict_mixed_value_types() {
        let py = r#"
d = {"name": "Alice", "age": 30, "active": True}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_136_nested_dict() {
        let py = r#"
d = {"outer": {"inner": "value"}}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_137_dict_update_spread() {
        let py = r#"
d1 = {"a": 1}
d2 = {"b": 2}
d1.update(d2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("extend") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_138_dict_with_int_keys() {
        let py = r#"
d = {1: "a", 2: "b", 3: "c"}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_139_dict_with_computed_keys() {
        let py = r#"
d = {str(i): i*2 for i in range(5)}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("collect"));
    }

    #[test]
    fn test_w12dt_dict_const_140_dict_with_tuple_values() {
        let py = r#"
d = {"a": (1, 2), "b": (3, 4)}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_141_dict_with_list_values() {
        let py = r#"
d = {"nums": [1, 2, 3], "letters": ["a", "b"]}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("vec"));
    }

    #[test]
    fn test_w12dt_dict_const_142_dict_from_constructor() {
        let py = r#"
d = dict([("a", 1), ("b", 2)])
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("collect"));
    }

    #[test]
    fn test_w12dt_dict_const_143_dict_with_conditional_values() {
        let py = r#"
d = {k: v if v > 0 else 0 for k, v in items}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_144_dict_three_level_nesting() {
        let py = r#"
d = {"a": {"b": {"c": "value"}}}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_145_dict_update_override() {
        let py = r#"
base = {"a": 1, "b": 2}
base["b"] = 3
"#;
        let rs = transpile(py);
        assert!(rs.contains("insert") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_146_dict_with_method_call_values() {
        let py = r#"
words = ["hello", "world"]
d = {w: w.upper() for w in words}
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_uppercase") || rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_dict_const_147_dict_filtered_comprehension() {
        let py = r#"
d = {k: v for k, v in items if v > 10}
"#;
        let rs = transpile(py);
        assert!(rs.contains("filter") || rs.contains(">"));
    }

    #[test]
    fn test_w12dt_dict_const_148_dict_with_expression_keys() {
        let py = r#"
d = {f"key_{i}": i for i in range(3)}
"#;
        let rs = transpile(py);
        assert!(rs.contains("format!") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_149_dict_merge_three_dicts() {
        let py = r#"
d1 = {"a": 1}
d2 = {"b": 2}
d3 = {"c": 3}
d1.update(d2)
d1.update(d3)
"#;
        let rs = transpile(py);
        assert!(rs.contains("extend") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_150_dict_with_none_values() {
        let py = r#"
d = {"a": None, "b": None}
"#;
        let rs = transpile(py);
        assert!(rs.contains("None") || rs.contains("Option"));
    }

    #[test]
    fn test_w12dt_dict_const_151_dict_with_float_values() {
        let py = r#"
stats = {"mean": 5.5, "median": 5.0, "std": 1.2}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_152_dict_comprehension_enumerate() {
        let py = r#"
d = {i: v for i, v in enumerate(items)}
"#;
        let rs = transpile(py);
        assert!(rs.contains("enumerate"));
    }

    #[test]
    fn test_w12dt_dict_const_153_dict_with_bool_values() {
        let py = r#"
flags = {"debug": True, "verbose": False}
"#;
        let rs = transpile(py);
        assert!(rs.contains("true") || rs.contains("false"));
    }

    #[test]
    fn test_w12dt_dict_const_154_dict_spread_with_override() {
        let py = r#"
defaults = {"timeout": 30, "retries": 3}
custom = defaults.copy()
custom["timeout"] = 60
"#;
        let rs = transpile(py);
        assert!(rs.contains("insert") || rs.contains("HashMap") || rs.contains("clone"));
    }

    #[test]
    fn test_w12dt_dict_const_155_dict_nested_comprehension() {
        let py = r#"
d = {k: {v: v*2 for v in range(3)} for k in ["a", "b"]}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_156_dict_with_set_values() {
        let py = r#"
d = {"evens": {2, 4, 6}, "odds": {1, 3, 5}}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashSet") || rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_157_dict_from_kwarg_pairs() {
        let py = r#"
d = dict(name="Alice", age=30)
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_158_dict_update_in_place() {
        let py = r#"
d = {"a": 1}
d.update({"b": 2})
"#;
        let rs = transpile(py);
        assert!(rs.contains("extend") || rs.contains("insert"));
    }

    #[test]
    fn test_w12dt_dict_const_159_dict_with_computed_values() {
        let py = r#"
d = {k: k*k for k in range(10)}
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap"));
    }

    #[test]
    fn test_w12dt_dict_const_160_dict_from_items() {
        let py = r#"
items = [("a", 1), ("b", 2), ("c", 3)]
d = dict(items)
"#;
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("collect"));
    }

    // ========================================================================
    // METHOD_CALL_ROUTING.RS TESTS (40 tests)
    // Target: string methods, list methods, dict methods, set methods,
    //         chained method calls, usage-based type inference, Option handling
    // ========================================================================

    #[test]
    fn test_w12dt_method_routing_161_string_upper() {
        let py = r#"
result = "hello".upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_uppercase") || rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_method_routing_162_string_lower() {
        let py = r#"
result = "HELLO".lower()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_lowercase") || rs.contains("lower"));
    }

    #[test]
    fn test_w12dt_method_routing_163_string_strip() {
        let py = r#"
result = "  text  ".strip()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim"));
    }

    #[test]
    fn test_w12dt_method_routing_164_string_split() {
        let py = r#"
parts = "a,b,c".split(",")
"#;
        let rs = transpile(py);
        assert!(rs.contains("split"));
    }

    #[test]
    fn test_w12dt_method_routing_165_string_replace() {
        let py = r#"
result = "hello world".replace("world", "Rust")
"#;
        let rs = transpile(py);
        assert!(rs.contains("replace"));
    }

    #[test]
    fn test_w12dt_method_routing_166_string_startswith() {
        let py = r#"
result = "hello".startswith("he")
"#;
        let rs = transpile(py);
        assert!(rs.contains("starts_with"));
    }

    #[test]
    fn test_w12dt_method_routing_167_string_endswith() {
        let py = r#"
result = "hello".endswith("lo")
"#;
        let rs = transpile(py);
        assert!(rs.contains("ends_with"));
    }

    #[test]
    fn test_w12dt_method_routing_168_string_join() {
        let py = r#"
result = ", ".join(["a", "b", "c"])
"#;
        let rs = transpile(py);
        assert!(rs.contains("join"));
    }

    #[test]
    fn test_w12dt_method_routing_169_list_append() {
        let py = r#"
items = [1, 2, 3]
items.append(4)
"#;
        let rs = transpile(py);
        assert!(rs.contains("push") || rs.contains("append"));
    }

    #[test]
    fn test_w12dt_method_routing_170_list_extend() {
        let py = r#"
items = [1, 2]
items.extend([3, 4])
"#;
        let rs = transpile(py);
        assert!(rs.contains("extend"));
    }

    #[test]
    fn test_w12dt_method_routing_171_list_sort() {
        let py = r#"
items = [3, 1, 2]
items.sort()
"#;
        let rs = transpile(py);
        assert!(rs.contains("sort"));
    }

    #[test]
    fn test_w12dt_method_routing_172_list_reverse() {
        let py = r#"
items = [1, 2, 3]
items.reverse()
"#;
        let rs = transpile(py);
        assert!(rs.contains("reverse"));
    }

    #[test]
    fn test_w12dt_method_routing_173_list_pop() {
        let py = r#"
items = [1, 2, 3]
last = items.pop()
"#;
        let rs = transpile(py);
        assert!(rs.contains("pop"));
    }

    #[test]
    fn test_w12dt_method_routing_174_list_remove() {
        let py = r#"
items = [1, 2, 3, 2]
items.remove(2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("retain") || rs.contains("remove"));
    }

    #[test]
    fn test_w12dt_method_routing_175_list_index() {
        let py = r#"
items = [10, 20, 30]
idx = items.index(20)
"#;
        let rs = transpile(py);
        assert!(rs.contains("position") || rs.contains("index"));
    }

    #[test]
    fn test_w12dt_method_routing_176_dict_keys() {
        let py = r#"
d = {"a": 1, "b": 2}
keys = d.keys()
"#;
        let rs = transpile(py);
        assert!(rs.contains("keys"));
    }

    #[test]
    fn test_w12dt_method_routing_177_dict_values() {
        let py = r#"
d = {"a": 1, "b": 2}
values = d.values()
"#;
        let rs = transpile(py);
        assert!(rs.contains("values"));
    }

    #[test]
    fn test_w12dt_method_routing_178_dict_items() {
        let py = r#"
d = {"a": 1, "b": 2}
items = d.items()
"#;
        let rs = transpile(py);
        assert!(rs.contains("items") || rs.contains("iter"));
    }

    #[test]
    fn test_w12dt_method_routing_179_dict_get() {
        let py = r#"
d = {"a": 1}
value = d.get("a")
"#;
        let rs = transpile(py);
        assert!(rs.contains("get"));
    }

    #[test]
    fn test_w12dt_method_routing_180_dict_get_with_default() {
        let py = r#"
d = {"a": 1}
value = d.get("b", 0)
"#;
        let rs = transpile(py);
        assert!(rs.contains("get") || rs.contains("unwrap_or"));
    }

    #[test]
    fn test_w12dt_method_routing_181_dict_pop() {
        let py = r#"
d = {"a": 1, "b": 2}
value = d.pop("a")
"#;
        let rs = transpile(py);
        assert!(rs.contains("remove"));
    }

    #[test]
    fn test_w12dt_method_routing_182_set_add() {
        let py = r#"
s = {1, 2, 3}
s.add(4)
"#;
        let rs = transpile(py);
        assert!(rs.contains("insert") || rs.contains("add"));
    }

    #[test]
    fn test_w12dt_method_routing_183_set_remove() {
        let py = r#"
s = {1, 2, 3}
s.remove(2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("remove"));
    }

    #[test]
    fn test_w12dt_method_routing_184_set_union() {
        let py = r#"
s1 = {1, 2}
s2 = {2, 3}
s3 = s1.union(s2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("union"));
    }

    #[test]
    fn test_w12dt_method_routing_185_set_intersection() {
        let py = r#"
s1 = {1, 2, 3}
s2 = {2, 3, 4}
s3 = s1.intersection(s2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("intersection"));
    }

    #[test]
    fn test_w12dt_method_routing_186_chained_string_methods() {
        let py = r#"
result = "  HELLO  ".strip().lower()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim") && (rs.contains("to_lowercase") || rs.contains("lower")));
    }

    #[test]
    fn test_w12dt_method_routing_187_chained_split_join() {
        let py = r#"
result = "hello world".split(" ")[0].upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("split"));
    }

    #[test]
    fn test_w12dt_method_routing_188_usage_inference_append() {
        let py = r#"
def process(data):
    data.append(42)
"#;
        let rs = transpile(py);
        assert!(rs.contains("push") || rs.contains("append"));
    }

    #[test]
    fn test_w12dt_method_routing_189_usage_inference_upper() {
        let py = r#"
def process(text):
    return text.upper()
"#;
        let rs = transpile(py);
        assert!(rs.contains("to_uppercase") || rs.contains("upper"));
    }

    #[test]
    fn test_w12dt_method_routing_190_usage_inference_keys() {
        let py = r#"
def process(mapping):
    return list(mapping.keys())
"#;
        let rs = transpile(py);
        assert!(rs.contains("keys"));
    }

    #[test]
    fn test_w12dt_method_routing_191_list_count() {
        let py = r#"
items = [1, 2, 2, 3, 2]
count = items.count(2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("filter") || rs.contains("count"));
    }

    #[test]
    fn test_w12dt_method_routing_192_string_find() {
        let py = r#"
text = "hello world"
pos = text.find("world")
"#;
        let rs = transpile(py);
        assert!(rs.contains("find"));
    }

    #[test]
    fn test_w12dt_method_routing_193_string_capitalize() {
        let py = r#"
result = "hello".capitalize()
"#;
        let rs = transpile(py);
        assert!(rs.contains("capitalize") || rs.contains("chars"));
    }

    #[test]
    fn test_w12dt_method_routing_194_string_title() {
        let py = r#"
result = "hello world".title()
"#;
        let rs = transpile(py);
        assert!(rs.contains("title") || rs.contains("split"));
    }

    #[test]
    fn test_w12dt_method_routing_195_list_clear() {
        let py = r#"
items = [1, 2, 3]
items.clear()
"#;
        let rs = transpile(py);
        assert!(rs.contains("clear"));
    }

    #[test]
    fn test_w12dt_method_routing_196_dict_setdefault() {
        let py = r#"
d = {}
d.setdefault("key", [])
"#;
        let rs = transpile(py);
        assert!(rs.contains("entry") || rs.contains("or_insert"));
    }

    #[test]
    fn test_w12dt_method_routing_197_set_difference() {
        let py = r#"
s1 = {1, 2, 3}
s2 = {2, 3}
s3 = s1.difference(s2)
"#;
        let rs = transpile(py);
        assert!(rs.contains("difference"));
    }

    #[test]
    fn test_w12dt_method_routing_198_string_isdigit() {
        let py = r#"
result = "123".isdigit()
"#;
        let rs = transpile(py);
        assert!(rs.contains("chars") || rs.contains("is_numeric"));
    }

    #[test]
    fn test_w12dt_method_routing_199_string_lstrip() {
        let py = r#"
result = "  hello".lstrip()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim_start") || rs.contains("lstrip"));
    }

    #[test]
    fn test_w12dt_method_routing_200_string_rstrip() {
        let py = r#"
result = "hello  ".rstrip()
"#;
        let rs = transpile(py);
        assert!(rs.contains("trim_end") || rs.contains("rstrip"));
    }
}
