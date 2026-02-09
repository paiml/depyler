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

    // Augmented assignments - Dict augmented (tests 1-20)
    #[test]
    fn test_w22ac_001() {
        let code = r#"
def update_dict() -> int:
    d = {"key": 10}
    d["key"] += 1
    return d["key"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_002() {
        let code = r#"
def update_dict() -> int:
    d = {"count": 100}
    d["count"] -= 5
    return d["count"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_003() {
        let code = r#"
def update_dict() -> int:
    d = {"value": 7}
    d["value"] *= 2
    return d["value"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_004() {
        let code = r#"
def update_dict() -> int:
    d = {"total": 20}
    d["total"] /= 4
    return d["total"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_005() {
        let code = r#"
def update_dict() -> int:
    d = {"remainder": 17}
    d["remainder"] %= 3
    return d["remainder"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_006() {
        let code = r#"
def update_dict() -> int:
    d = {"score": 50, "bonus": 10}
    d["score"] += d["bonus"]
    return d["score"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_007() {
        let code = r#"
def update_dict() -> int:
    d = {"x": 25}
    d["x"] -= 10
    d["x"] += 5
    return d["x"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_008() {
        let code = r#"
def update_dict() -> int:
    d = {"product": 3}
    d["product"] *= 5
    d["product"] *= 2
    return d["product"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_009() {
        let code = r#"
def update_dict() -> int:
    d = {"dividend": 100}
    d["dividend"] /= 2
    d["dividend"] /= 5
    return d["dividend"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_010() {
        let code = r#"
def update_dict() -> int:
    d = {"mod": 50}
    d["mod"] %= 7
    return d["mod"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_011() {
        let code = r#"
def update_dict_multiple() -> int:
    d = {"a": 1, "b": 2, "c": 3}
    d["a"] += 10
    d["b"] -= 1
    d["c"] *= 2
    return d["a"] + d["b"] + d["c"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_012() {
        let code = r#"
def update_dict() -> int:
    d = {"n": 8}
    d["n"] //= 2
    return d["n"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_013() {
        let code = r#"
def update_dict() -> int:
    d = {"power": 2}
    d["power"] **= 3
    return d["power"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_014() {
        let code = r#"
def update_dict() -> int:
    d = {"idx": 0}
    d["idx"] += 1
    d["idx"] += 1
    d["idx"] += 1
    return d["idx"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_015() {
        let code = r#"
def update_dict() -> int:
    d = {"counter": 10}
    d["counter"] -= 2
    d["counter"] -= 3
    return d["counter"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_016() {
        let code = r#"
def update_dict() -> int:
    d = {"factor": 5}
    d["factor"] *= 3
    return d["factor"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_017() {
        let code = r#"
def update_dict() -> int:
    d = {"quotient": 50}
    d["quotient"] //= 3
    return d["quotient"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_018() {
        let code = r#"
def update_dict() -> int:
    d = {"base": 3}
    d["base"] **= 2
    return d["base"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_019() {
        let code = r#"
def update_dict() -> int:
    d = {"result": 15}
    d["result"] %= 4
    return d["result"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_020() {
        let code = r#"
def update_dict() -> int:
    d = {"num": 7}
    d["num"] += 3
    d["num"] *= 2
    return d["num"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // List augmented assignments (tests 21-35)
    #[test]
    fn test_w22ac_021() {
        let code = r#"
def update_list() -> int:
    lst = [1, 2, 3]
    lst[0] += 10
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_022() {
        let code = r#"
def update_list() -> int:
    lst = [10, 20, 30]
    i = 1
    lst[i] -= 1
    return lst[i]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_023() {
        let code = r#"
def update_list() -> int:
    lst = [5, 10, 15]
    lst[2] *= 2
    return lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_024() {
        let code = r#"
def update_list() -> int:
    lst = [100, 200]
    lst[0] /= 5
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_025() {
        let code = r#"
def update_list() -> int:
    lst = [17, 23]
    lst[0] %= 5
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_026() {
        let code = r#"
def update_list() -> int:
    lst = [1, 2, 3, 4, 5]
    lst[0] += 1
    lst[1] += 2
    lst[2] += 3
    return lst[0] + lst[1] + lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_027() {
        let code = r#"
def update_list() -> int:
    lst = [10, 20, 30]
    lst[-1] += 5
    return lst[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_028() {
        let code = r#"
def update_list() -> int:
    lst = [8, 16, 24]
    lst[1] //= 4
    return lst[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_029() {
        let code = r#"
def update_list() -> int:
    lst = [2, 3, 4]
    lst[0] **= 3
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_030() {
        let code = r#"
def update_list() -> int:
    lst = [50, 60, 70]
    lst[0] -= 10
    lst[1] -= 20
    return lst[0] + lst[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_031() {
        let code = r#"
def update_list() -> int:
    lst = [3, 6, 9]
    lst[0] *= 2
    lst[1] *= 2
    lst[2] *= 2
    return lst[0] + lst[1] + lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_032() {
        let code = r#"
def update_list() -> int:
    lst = [100, 50, 25]
    lst[0] /= 10
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_033() {
        let code = r#"
def update_list() -> int:
    lst = [13, 27, 41]
    lst[1] %= 10
    return lst[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_034() {
        let code = r#"
def update_list() -> int:
    lst = [10]
    idx = 0
    lst[idx] += 5
    return lst[idx]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_035() {
        let code = r#"
def update_list() -> int:
    lst = [1, 2, 3, 4]
    lst[0] += lst[1]
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // Variable augmented assignments (tests 36-50)
    #[test]
    fn test_w22ac_036() {
        let code = r#"
def increment() -> int:
    x = 10
    x += 1
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_037() {
        let code = r#"
def decrement() -> int:
    x = 10
    x -= 1
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_038() {
        let code = r#"
def multiply() -> int:
    x = 5
    x *= 2
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_039() {
        let code = r#"
def divide() -> int:
    x = 20
    x /= 2
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_040() {
        let code = r#"
def modulo() -> int:
    x = 17
    x %= 3
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_041() {
        let code = r#"
def floor_divide() -> int:
    x = 25
    x //= 2
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_042() {
        let code = r#"
def power() -> int:
    x = 2
    x **= 3
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_043() {
        let code = r#"
def string_augment() -> str:
    s = "hello"
    s += " world"
    return s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_044() {
        let code = r#"
def list_augment() -> int:
    result = [1, 2]
    item = 3
    result += [item]
    return len(result)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_045() {
        let code = r#"
def float_augment() -> float:
    f = 1.5
    f += 0.5
    return f
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_046() {
        let code = r#"
def float_subtract() -> float:
    f = 5.0
    f -= 1.0
    return f
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_047() {
        let code = r#"
def float_multiply() -> float:
    f = 2.5
    f *= 2.0
    return f
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_048() {
        let code = r#"
def multiple_augments() -> int:
    x = 10
    x += 5
    x -= 3
    x *= 2
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_049() {
        let code = r#"
def chain_augments() -> int:
    a = 1
    a += 1
    a += 1
    a += 1
    a += 1
    return a
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_050() {
        let code = r#"
def mixed_augments() -> int:
    x = 100
    x //= 5
    x **= 2
    x %= 17
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // Tuple unpacking - Simple (tests 51-70)
    #[test]
    fn test_w22ac_051() {
        let code = r#"
def simple_unpack() -> int:
    a, b = (1, 2)
    return a + b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_052() {
        let code = r#"
def triple_unpack() -> int:
    x, y, z = (1, 2, 3)
    return x + y + z
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_053() {
        let code = r#"
def pair() -> tuple:
    return (1, 2)

def from_function() -> int:
    a, b = pair()
    return a + b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_054() {
        let code = r#"
def enumerate_unpack() -> int:
    lst = [10, 20, 30]
    total = 0
    for i, x in enumerate(lst):
        total += x
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_055() {
        let code = r#"
def dict_items_unpack() -> int:
    d = {"a": 1, "b": 2}
    total = 0
    for k, v in d.items():
        total += v
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_056() {
        let code = r#"
def zip_unpack() -> int:
    xs = [1, 2, 3]
    ys = [4, 5, 6]
    total = 0
    for a, b in zip(xs, ys):
        total += a + b
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_057() {
        let code = r#"
def quad_unpack() -> int:
    a, b, c, d = (1, 2, 3, 4)
    return a + b + c + d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_058() {
        let code = r#"
def pairs_loop() -> int:
    pairs = [(1, 2), (3, 4), (5, 6)]
    total = 0
    for x, y in pairs:
        total += x + y
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_059() {
        let code = r#"
def swap() -> int:
    a = 1
    b = 2
    a, b = b, a
    return a - b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_060() {
        let code = r#"
def nested_tuple() -> int:
    x, y = (10, 20)
    return x * y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_061() {
        let code = r#"
def five_unpack() -> int:
    a, b, c, d, e = (1, 2, 3, 4, 5)
    return a + b + c + d + e
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_062() {
        let code = r#"
def string_unpack() -> str:
    s1, s2 = ("hello", "world")
    return s1 + s2
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_063() {
        let code = r#"
def mixed_types() -> int:
    i, s, f = (42, "test", 2.5)
    return i
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_064() {
        let code = r#"
def triple() -> tuple:
    return (7, 8, 9)

def from_triple() -> int:
    x, y, z = triple()
    return x + y + z
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_065() {
        let code = r#"
def enumerate_with_start() -> int:
    lst = [100, 200, 300]
    total = 0
    for idx, val in enumerate(lst):
        total += idx + val
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_066() {
        let code = r#"
def dict_keys_values() -> int:
    d = {"x": 10, "y": 20}
    total = 0
    for key, value in d.items():
        total += value
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_067() {
        let code = r#"
def zip_three() -> int:
    a = [1, 2]
    b = [3, 4]
    c = [5, 6]
    total = 0
    for x, y in zip(a, b):
        total += x + y
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_068() {
        let code = r#"
def multi_swap() -> int:
    a, b, c = 1, 2, 3
    a, b, c = c, a, b
    return a + b + c
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_069() {
        let code = r#"
def unpack_in_loop() -> int:
    data = [(1, 10), (2, 20), (3, 30)]
    result = 0
    for i, val in data:
        result += i * val
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_070() {
        let code = r#"
def simple_pair() -> int:
    x, y = 5, 7
    return x + y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // Tuple unpacking with type annotations (tests 71-100)
    #[test]
    fn test_w22ac_071() {
        let code = r#"
def typed_unpack() -> int:
    a: int
    b: int
    a, b = (10, 20)
    return a + b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_072() {
        let code = r#"
def typed_triple() -> int:
    x: int
    y: int
    z: int
    x, y, z = (1, 2, 3)
    return x + y + z
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_073() {
        let code = r#"
def typed_strings() -> str:
    s1: str
    s2: str
    s1, s2 = ("foo", "bar")
    return s1 + s2
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_074() {
        let code = r#"
def typed_floats() -> float:
    f1: float
    f2: float
    f1, f2 = (1.5, 2.5)
    return f1 + f2
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_075() {
        let code = r#"
def typed_quad() -> int:
    a: int
    b: int
    c: int
    d: int
    a, b, c, d = (10, 20, 30, 40)
    return a + b + c + d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_076() {
        let code = r#"
def coordinate() -> int:
    x: int
    y: int
    x, y = (5, 10)
    return x * y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_077() {
        let code = r#"
def dimensions() -> int:
    width: int
    height: int
    depth: int
    width, height, depth = (10, 20, 30)
    return width * height * depth
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_078() {
        let code = r#"
def rgb() -> int:
    r: int
    g: int
    b: int
    r, g, b = (255, 128, 64)
    return r + g + b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_079() {
        let code = r#"
def range_unpack() -> int:
    start: int
    end: int
    start, end = (0, 100)
    return end - start
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_080() {
        let code = r#"
def min_max() -> int:
    minimum: int
    maximum: int
    minimum, maximum = (1, 100)
    return maximum - minimum
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_081() {
        let code = r#"
def first_last() -> int:
    first: int
    last: int
    first, last = (10, 90)
    return first + last
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_082() {
        let code = r#"
def left_right() -> int:
    left: int
    right: int
    left, right = (5, 15)
    return right - left
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_083() {
        let code = r#"
def top_bottom() -> int:
    top: int
    bottom: int
    top, bottom = (100, 0)
    return top - bottom
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_084() {
        let code = r#"
def before_after() -> int:
    before: int
    after: int
    before, after = (50, 75)
    return after - before
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_085() {
        let code = r#"
def old_new() -> int:
    old: int
    new: int
    old, new = (10, 20)
    return new - old
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_086() {
        let code = r#"
def prev_curr() -> int:
    prev: int
    curr: int
    prev, curr = (1, 2)
    return curr - prev
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_087() {
        let code = r#"
def low_high() -> int:
    low: int
    high: int
    low, high = (10, 100)
    return high - low
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_088() {
        let code = r#"
def head_tail() -> int:
    head: int
    tail: int
    head, tail = (1, 9)
    return head + tail
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_089() {
        let code = r#"
def alpha_beta() -> int:
    alpha: int
    beta: int
    alpha, beta = (5, 10)
    return alpha * beta
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_090() {
        let code = r#"
def src_dst() -> int:
    src: int
    dst: int
    src, dst = (0, 100)
    return dst - src
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_091() {
        let code = r#"
def in_out() -> int:
    inp: int
    out: int
    inp, out = (10, 20)
    return out - inp
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_092() {
        let code = r#"
def upper_lower() -> int:
    upper: int
    lower: int
    upper, lower = (100, 50)
    return upper - lower
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_093() {
        let code = r#"
def base_offset() -> int:
    base: int
    offset: int
    base, offset = (100, 5)
    return base + offset
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_094() {
        let code = r#"
def row_col() -> int:
    row: int
    col: int
    row, col = (3, 7)
    return row * col
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_095() {
        let code = r#"
def year_month() -> int:
    year: int
    month: int
    year, month = (2024, 12)
    return year + month
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_096() {
        let code = r#"
def hour_minute() -> int:
    hour: int
    minute: int
    hour, minute = (14, 30)
    return hour * 60 + minute
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_097() {
        let code = r#"
def lat_lon() -> int:
    lat: int
    lon: int
    lat, lon = (40, 74)
    return lat + lon
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_098() {
        let code = r#"
def real_imag() -> int:
    real: int
    imag: int
    real, imag = (3, 4)
    return real * real + imag * imag
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_099() {
        let code = r#"
def num_den() -> int:
    num: int
    den: int
    num, den = (10, 2)
    return num / den
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_100() {
        let code = r#"
def major_minor() -> int:
    major: int
    minor: int
    major, minor = (1, 5)
    return major * 10 + minor
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // Attribute assignments (tests 101-130)
    #[test]
    fn test_w22ac_101() {
        let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_102() {
        let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count = self.count + 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_103() {
        let code = r#"
class Rectangle:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.area = width * height
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_104() {
        let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_105() {
        let code = r#"
class Account:
    def __init__(self, balance: int):
        self.balance = balance

    def deposit(self, amount: int):
        self.balance = self.balance + amount
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_106() {
        let code = r#"
class Circle:
    def __init__(self, radius: int):
        self.radius = radius
        self.diameter = radius * 2
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_107() {
        let code = r#"
class Student:
    def __init__(self, student_id: int, grade: int):
        self.student_id = student_id
        self.grade = grade
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_108() {
        let code = r#"
class Book:
    def __init__(self, title: str, pages: int):
        self.title = title
        self.pages = pages
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_109() {
        let code = r#"
class Car:
    def __init__(self, make: str, year: int):
        self.make = make
        self.year = year
        self.mileage = 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_110() {
        let code = r#"
class Temperature:
    def __init__(self, celsius: int):
        self.celsius = celsius
        self.fahrenheit = celsius * 9 / 5 + 32
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_111() {
        let code = r#"
class Vector:
    def __init__(self):
        self.x = 0
        self.y = 0
        self.z = 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_112() {
        let code = r#"
class Timer:
    def __init__(self):
        self.seconds = 0

    def tick(self):
        self.seconds = self.seconds + 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_113() {
        let code = r#"
class Score:
    def __init__(self):
        self.points = 0

    def add_points(self, pts: int):
        self.points = self.points + pts
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_114() {
        let code = r#"
class Config:
    def __init__(self):
        self.setting1 = 10
        self.setting2 = 20
        self.setting3 = 30
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_115() {
        let code = r#"
class Player:
    def __init__(self, name: str):
        self.name = name
        self.score = 0
        self.level = 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_116() {
        let code = r#"
class Stats:
    def __init__(self):
        self.total = 0
        self.count = 0

    def update(self, value: int):
        self.total = self.total + value
        self.count = self.count + 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_117() {
        let code = r#"
class Item:
    def __init__(self, name: str, price: int):
        self.name = name
        self.price = price
        self.quantity = 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_118() {
        let code = r#"
class Position:
    def __init__(self):
        self.x = 0
        self.y = 0

    def move(self, dx: int, dy: int):
        self.x = self.x + dx
        self.y = self.y + dy
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_119() {
        let code = r#"
class Dimension:
    def __init__(self, w: int, h: int, d: int):
        self.width = w
        self.height = h
        self.depth = d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_120() {
        let code = r#"
class Range:
    def __init__(self, minimum: int, maximum: int):
        self.min = minimum
        self.max = maximum
        self.span = maximum - minimum
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_121() {
        let code = r#"
class Color:
    def __init__(self, r: int, g: int, b: int):
        self.red = r
        self.green = g
        self.blue = b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_122() {
        let code = r#"
class Date:
    def __init__(self, y: int, m: int, d: int):
        self.year = y
        self.month = m
        self.day = d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_123() {
        let code = r#"
class Time:
    def __init__(self, h: int, m: int, s: int):
        self.hours = h
        self.minutes = m
        self.seconds = s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_124() {
        let code = r#"
class Box:
    def __init__(self):
        self.length = 10
        self.width = 5
        self.height = 3
        self.volume = 10 * 5 * 3
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_125() {
        let code = r#"
class Pair:
    def __init__(self, first: int, second: int):
        self.first = first
        self.second = second

    def swap(self):
        temp = self.first
        self.first = self.second
        self.second = temp
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_126() {
        let code = r#"
class Accumulator:
    def __init__(self):
        self.sum = 0

    def add(self, n: int):
        self.sum = self.sum + n

    def subtract(self, n: int):
        self.sum = self.sum - n
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_127() {
        let code = r#"
class Boundary:
    def __init__(self, top: int, bottom: int, left: int, right: int):
        self.top = top
        self.bottom = bottom
        self.left = left
        self.right = right
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_128() {
        let code = r#"
class Fraction:
    def __init__(self, numerator: int, denominator: int):
        self.num = numerator
        self.den = denominator
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_129() {
        let code = r#"
class Complex:
    def __init__(self, real: int, imag: int):
        self.real = real
        self.imag = imag
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_130() {
        let code = r#"
class Version:
    def __init__(self, major: int, minor: int, patch: int):
        self.major = major
        self.minor = minor
        self.patch = patch
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // Index assignments (tests 131-170)
    #[test]
    fn test_w22ac_131() {
        let code = r#"
def set_list_index() -> int:
    lst = [1, 2, 3]
    lst[0] = 10
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_132() {
        let code = r#"
def set_last_index() -> int:
    lst = [1, 2, 3]
    lst[-1] = 99
    return lst[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_133() {
        let code = r#"
def set_dict_string_key() -> str:
    d = {}
    d["key"] = "value"
    return d["key"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_134() {
        let code = r#"
def set_dict_variable_key() -> int:
    d = {}
    key = "mykey"
    d[key] = 42
    return d[key]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_135() {
        let code = r#"
def nested_index() -> int:
    matrix = [[1, 2], [3, 4]]
    matrix[0][1] = 5
    return matrix[0][1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_136() {
        let code = r#"
def dict_key_heuristic() -> int:
    d = {}
    k = "name"
    d[k] = 10
    return d[k]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_137() {
        let code = r#"
def list_index_heuristic() -> int:
    lst = [0, 0, 0]
    i = 1
    lst[i] = 100
    return lst[i]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_138() {
        let code = r#"
def dict_id_key() -> str:
    d = {}
    id_val = "user123"
    d[id_val] = "John"
    return d[id_val]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_139() {
        let code = r#"
def list_idx_var() -> int:
    lst = [10, 20, 30]
    idx = 2
    lst[idx] = 99
    return lst[idx]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_140() {
        let code = r#"
def dict_name_key() -> str:
    d = {}
    name = "Alice"
    d[name] = "Engineer"
    return d[name]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_141() {
        let code = r#"
def list_index_expr() -> int:
    lst = [1, 2, 3, 4, 5]
    i = 1
    lst[i + 1] = 100
    return lst[i + 1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_142() {
        let code = r#"
def dict_word_key() -> int:
    d = {}
    word = "count"
    d[word] = 5
    return d[word]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_143() {
        let code = r#"
def multiple_dict_assigns() -> int:
    d = {}
    d["a"] = 1
    d["b"] = 2
    d["c"] = 3
    return d["a"] + d["b"] + d["c"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_144() {
        let code = r#"
def multiple_list_assigns() -> int:
    lst = [0, 0, 0, 0]
    lst[0] = 10
    lst[1] = 20
    lst[2] = 30
    return lst[0] + lst[1] + lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_145() {
        let code = r#"
def dict_numeric_value() -> int:
    d = {}
    d["total"] = 100
    return d["total"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_146() {
        let code = r#"
def list_middle_index() -> int:
    lst = [1, 2, 3, 4, 5]
    lst[2] = 99
    return lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_147() {
        let code = r#"
def dict_update_existing() -> int:
    d = {"count": 5}
    d["count"] = 10
    return d["count"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_148() {
        let code = r#"
def list_update_existing() -> int:
    lst = [1, 2, 3]
    lst[1] = 99
    return lst[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_149() {
        let code = r#"
def nested_dict_assign() -> int:
    d = {"inner": {}}
    d["inner"]["value"] = 42
    return d["inner"]["value"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_150() {
        let code = r#"
def nested_list_assign() -> int:
    matrix = [[0, 0], [0, 0]]
    matrix[1][0] = 5
    return matrix[1][0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_151() {
        let code = r#"
def loop_index_assign() -> int:
    lst = [0, 0, 0, 0, 0]
    for i in range(5):
        lst[i] = i * 10
    return lst[0] + lst[4]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_152() {
        let code = r#"
def dict_loop_assign() -> int:
    d = {}
    for i in range(3):
        key = str(i)
        d[key] = i * 10
    return len(d)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_153() {
        let code = r#"
def conditional_list_assign() -> int:
    lst = [1, 2, 3]
    if lst[0] > 0:
        lst[0] = 100
    return lst[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_154() {
        let code = r#"
def conditional_dict_assign() -> int:
    d = {"key": 5}
    if d["key"] < 10:
        d["key"] = 20
    return d["key"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_155() {
        let code = r#"
def list_swap_indices() -> int:
    lst = [1, 2, 3]
    temp = lst[0]
    lst[0] = lst[2]
    lst[2] = temp
    return lst[0] + lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_156() {
        let code = r#"
def dict_chain_assign() -> int:
    d = {}
    d["a"] = 1
    d["b"] = d["a"] + 1
    d["c"] = d["b"] + 1
    return d["c"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_157() {
        let code = r#"
def list_chain_assign() -> int:
    lst = [1, 0, 0]
    lst[1] = lst[0] + 1
    lst[2] = lst[1] + 1
    return lst[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_158() {
        let code = r#"
def dict_key_from_var() -> str:
    d = {}
    key_name = "result"
    d[key_name] = "success"
    return d[key_name]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_159() {
        let code = r#"
def list_index_from_var() -> int:
    lst = [10, 20, 30, 40]
    index = 3
    lst[index] = 99
    return lst[index]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_160() {
        let code = r#"
def dict_multiple_keys() -> int:
    d = {}
    key1 = "first"
    key2 = "second"
    d[key1] = 10
    d[key2] = 20
    return d[key1] + d[key2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_161() {
        let code = r#"
def list_multiple_indices() -> int:
    lst = [0, 0, 0, 0]
    i = 0
    j = 2
    lst[i] = 5
    lst[j] = 15
    return lst[i] + lst[j]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_162() {
        let code = r#"
def dict_overwrite() -> int:
    d = {"value": 1}
    d["value"] = 2
    d["value"] = 3
    return d["value"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_163() {
        let code = r#"
def list_overwrite() -> int:
    lst = [1, 2, 3]
    lst[1] = 10
    lst[1] = 20
    return lst[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_164() {
        let code = r#"
def three_d_array() -> int:
    arr = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
    arr[0][0][0] = 99
    return arr[0][0][0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_165() {
        let code = r#"
def dict_of_lists() -> int:
    d = {"nums": [1, 2, 3]}
    d["nums"][0] = 10
    return d["nums"][0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_166() {
        let code = r#"
def list_of_dicts() -> int:
    lst = [{"val": 1}, {"val": 2}]
    lst[0]["val"] = 99
    return lst[0]["val"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_167() {
        let code = r#"
def dict_computed_key() -> int:
    d = {}
    prefix = "key"
    suffix = "1"
    d[prefix + suffix] = 42
    return d["key1"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_168() {
        let code = r#"
def list_computed_index() -> int:
    lst = [0, 0, 0, 0, 0]
    base = 2
    offset = 1
    lst[base + offset] = 77
    return lst[3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_169() {
        let code = r#"
def dict_empty_then_fill() -> int:
    d = {}
    for i in range(5):
        d[str(i)] = i * i
    return len(d)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_170() {
        let code = r#"
def list_fill_pattern() -> int:
    lst = [0] * 10
    for i in range(0, 10, 2):
        lst[i] = 1
    return sum(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // Control flow in assignments (tests 171-200)
    #[test]
    fn test_w22ac_171() {
        let code = r#"
def conditional_assignment() -> int:
    x = 10
    if x > 5:
        y = 100
    else:
        y = 50
    return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_172() {
        let code = r#"
def ternary_assignment() -> int:
    x = 10
    y = 100 if x > 5 else 50
    return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_173() {
        let code = r#"
def loop_accumulator() -> int:
    total = 0
    for i in range(10):
        total = total + i
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_174() {
        let code = r#"
def while_counter() -> int:
    count = 0
    while count < 10:
        count = count + 1
    return count
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_175() {
        let code = r#"
def nested_loop_assign() -> int:
    result = 0
    for i in range(5):
        for j in range(5):
            result = result + 1
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_176() {
        let code = r#"
def conditional_in_loop() -> int:
    total = 0
    for i in range(10):
        if i % 2 == 0:
            total = total + i
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_177() {
        let code = r#"
CONSTANT = 100

def use_constant() -> int:
    x = CONSTANT
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_178() {
        let code = r#"
def multiple_returns() -> int:
    x = 10
    if x > 5:
        y = 100
        return y
    else:
        y = 50
        return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_179() {
        let code = r#"
def early_return() -> int:
    x = 10
    if x > 100:
        return 0
    y = x * 2
    return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_180() {
        let code = r#"
def nested_conditionals() -> int:
    x = 10
    if x > 5:
        if x > 8:
            y = 100
        else:
            y = 50
    else:
        y = 25
    return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_181() {
        let code = r#"
def loop_with_break() -> int:
    result = 0
    for i in range(100):
        result = result + 1
        if i == 10:
            break
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_182() {
        let code = r#"
def loop_with_continue() -> int:
    total = 0
    for i in range(10):
        if i % 2 == 0:
            continue
        total = total + i
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_183() {
        let code = r#"
def while_with_break() -> int:
    count = 0
    while True:
        count = count + 1
        if count >= 10:
            break
    return count
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_184() {
        let code = r#"
def multiple_assignments_in_func() -> int:
    a = 1
    b = 2
    c = 3
    d = 4
    e = 5
    return a + b + c + d + e
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_185() {
        let code = r#"
def chain_assignments() -> int:
    a = 1
    b = a + 1
    c = b + 1
    d = c + 1
    return d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_186() {
        let code = r#"
def reassignment_in_loop() -> int:
    x = 0
    for i in range(5):
        x = i
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_187() {
        let code = r#"
def fibonacci_like() -> int:
    a = 1
    b = 1
    for i in range(5):
        temp = a + b
        a = b
        b = temp
    return b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_188() {
        let code = r#"
def max_in_loop() -> int:
    maximum = 0
    for i in range(10):
        if i > maximum:
            maximum = i
    return maximum
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_189() {
        let code = r#"
def min_in_loop() -> int:
    minimum = 100
    for i in range(10):
        if i < minimum:
            minimum = i
    return minimum
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_190() {
        let code = r#"
def product_in_loop() -> int:
    product = 1
    for i in range(1, 6):
        product = product * i
    return product
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_191() {
        let code = r#"
def conditional_accumulator() -> int:
    even_sum = 0
    odd_sum = 0
    for i in range(10):
        if i % 2 == 0:
            even_sum = even_sum + i
        else:
            odd_sum = odd_sum + i
    return even_sum + odd_sum
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_192() {
        let code = r#"
def nested_loops_product() -> int:
    result = 0
    for i in range(3):
        for j in range(3):
            result = result + i * j
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_193() {
        let code = r#"
def while_accumulator() -> int:
    total = 0
    i = 0
    while i < 10:
        total = total + i
        i = i + 1
    return total
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_194() {
        let code = r#"
def assignment_after_loop() -> int:
    x = 0
    for i in range(5):
        x = x + 1
    y = x * 2
    return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_195() {
        let code = r#"
def multiple_conditionals() -> int:
    x = 10
    if x > 20:
        result = 1
    elif x > 10:
        result = 2
    elif x > 5:
        result = 3
    else:
        result = 4
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_196() {
        let code = r#"
def loop_with_multiple_updates() -> int:
    a = 0
    b = 0
    for i in range(5):
        a = a + i
        b = b + i * 2
    return a + b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_197() {
        let code = r#"
def assignment_from_function_call() -> int:
    def helper() -> int:
        return 42
    x = helper()
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_198() {
        let code = r#"
def complex_expression_assign() -> int:
    a = 10
    b = 20
    c = 30
    result = a * b + c
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_199() {
        let code = r#"
def list_comprehension_assign() -> int:
    nums = [i * 2 for i in range(5)]
    return len(nums)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22ac_200() {
        let code = r#"
def dict_comprehension_assign() -> int:
    d = {i: i * i for i in range(5)}
    return len(d)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
