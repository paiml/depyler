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

    // String methods: zfill (tests 1-10)
    #[test]
    fn test_w22md_001() {
        let result = transpile(r#"
def zfill_basic() -> str:
    s: str = "42"
    return s.zfill(5)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_002() {
        let result = transpile(r#"
def zfill_negative() -> str:
    s: str = "-42"
    return s.zfill(6)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_003() {
        let result = transpile(r#"
def zfill_positive() -> str:
    s: str = "+3"
    return s.zfill(5)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_004() {
        let result = transpile(r#"
def zfill_empty() -> str:
    s: str = ""
    return s.zfill(10)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_005() {
        let result = transpile(r#"
def zfill_literal() -> str:
    return "123".zfill(7)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_006() {
        let result = transpile(r#"
def zfill_with_var() -> str:
    width: int = 8
    s: str = "99"
    return s.zfill(width)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_007() {
        let result = transpile(r#"
def zfill_loop() -> list:
    nums: list = ["1", "22", "333"]
    result: list = []
    for n in nums:
        result.append(n.zfill(5))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_008() {
        let result = transpile(r#"
def zfill_conditional() -> str:
    s: str = "7"
    if len(s) < 3:
        return s.zfill(3)
    return s
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_009() {
        let result = transpile(r#"
def zfill_chained() -> str:
    s: str = "abc"
    return s.upper().zfill(10)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("zfill"));
    }

    #[test]
    fn test_w22md_010() {
        let result = transpile(r#"
def zfill_assign() -> None:
    x: str = "5"
    x = x.zfill(4)
"#);
        assert!(!result.is_empty());
    }

    // String methods: capitalize (tests 11-20)
    #[test]
    fn test_w22md_011() {
        let result = transpile(r#"
def capitalize_basic() -> str:
    s: str = "hello"
    return s.capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_012() {
        let result = transpile(r#"
def capitalize_empty() -> str:
    s: str = ""
    return s.capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_013() {
        let result = transpile(r#"
def capitalize_literal() -> str:
    return "world".capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_014() {
        let result = transpile(r#"
def capitalize_multiple() -> str:
    s: str = "hello WORLD"
    return s.capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_015() {
        let result = transpile(r#"
def capitalize_loop() -> list:
    words: list = ["apple", "banana", "cherry"]
    result: list = []
    for w in words:
        result.append(w.capitalize())
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_016() {
        let result = transpile(r#"
def capitalize_chained() -> str:
    s: str = "HELLO"
    return s.lower().capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_017() {
        let result = transpile(r#"
def capitalize_conditional() -> str:
    s: str = "test"
    if s:
        return s.capitalize()
    return s
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_018() {
        let result = transpile(r#"
def capitalize_assign() -> None:
    text: str = "python"
    text = text.capitalize()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_019() {
        let result = transpile(r#"
def capitalize_return_direct() -> str:
    return "rust".capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    #[test]
    fn test_w22md_020() {
        let result = transpile(r#"
def capitalize_nested() -> str:
    s: str = "outer"
    if True:
        t: str = "inner"
        return t.capitalize()
    return s.capitalize()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("capitalize"));
    }

    // String methods: swapcase (tests 21-30)
    #[test]
    fn test_w22md_021() {
        let result = transpile(r#"
def swapcase_basic() -> str:
    s: str = "Hello World"
    return s.swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_022() {
        let result = transpile(r#"
def swapcase_literal() -> str:
    return "PyThOn".swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_023() {
        let result = transpile(r#"
def swapcase_empty() -> str:
    s: str = ""
    return s.swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_024() {
        let result = transpile(r#"
def swapcase_loop() -> list:
    words: list = ["ABC", "def", "GhI"]
    result: list = []
    for w in words:
        result.append(w.swapcase())
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_025() {
        let result = transpile(r#"
def swapcase_chained() -> str:
    s: str = "test"
    return s.upper().swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_026() {
        let result = transpile(r#"
def swapcase_assign() -> None:
    text: str = "Hello"
    text = text.swapcase()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_027() {
        let result = transpile(r#"
def swapcase_conditional() -> str:
    s: str = "TeSt"
    if len(s) > 0:
        return s.swapcase()
    return s
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_028() {
        let result = transpile(r#"
def swapcase_double() -> str:
    s: str = "ABC"
    return s.swapcase().swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_029() {
        let result = transpile(r#"
def swapcase_numbers() -> str:
    s: str = "Test123"
    return s.swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    #[test]
    fn test_w22md_030() {
        let result = transpile(r#"
def swapcase_return_direct() -> str:
    return "DirectSwap".swapcase()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("swapcase"));
    }

    // String methods: expandtabs (tests 31-40)
    #[test]
    fn test_w22md_031() {
        let result = transpile(r#"
def expandtabs_basic() -> str:
    s: str = "a\tb"
    return s.expandtabs()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_032() {
        let result = transpile(r#"
def expandtabs_with_size() -> str:
    s: str = "a\tb"
    return s.expandtabs(4)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_033() {
        let result = transpile(r#"
def expandtabs_literal() -> str:
    return "x\ty\tz".expandtabs(8)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_034() {
        let result = transpile(r#"
def expandtabs_multiple() -> str:
    s: str = "a\tb\tc"
    return s.expandtabs(2)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_035() {
        let result = transpile(r#"
def expandtabs_var_size() -> str:
    s: str = "tab\there"
    tabsize: int = 6
    return s.expandtabs(tabsize)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_036() {
        let result = transpile(r#"
def expandtabs_loop() -> list:
    lines: list = ["a\tb", "c\td"]
    result: list = []
    for line in lines:
        result.append(line.expandtabs())
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_037() {
        let result = transpile(r#"
def expandtabs_chained() -> str:
    s: str = "TAB\tHERE"
    return s.expandtabs(4).lower()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_038() {
        let result = transpile(r#"
def expandtabs_assign() -> None:
    text: str = "a\tb\tc"
    text = text.expandtabs(3)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_039() {
        let result = transpile(r#"
def expandtabs_conditional() -> str:
    s: str = "test\ttabs"
    if "\t" in s:
        return s.expandtabs(8)
    return s
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    #[test]
    fn test_w22md_040() {
        let result = transpile(r#"
def expandtabs_empty() -> str:
    s: str = ""
    return s.expandtabs()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("expandtabs"));
    }

    // String methods: splitlines (tests 41-50)
    #[test]
    fn test_w22md_041() {
        let result = transpile(r#"
def splitlines_basic() -> list:
    s: str = "a\nb\nc"
    return s.splitlines()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_042() {
        let result = transpile(r#"
def splitlines_literal() -> list:
    return "line1\nline2\nline3".splitlines()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_043() {
        let result = transpile(r#"
def splitlines_empty() -> list:
    s: str = ""
    return s.splitlines()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_044() {
        let result = transpile(r#"
def splitlines_basic_use() -> list:
    s: str = "a\nb\nc"
    lines: list = s.splitlines()
    return lines
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_045() {
        let result = transpile(r#"
def splitlines_loop() -> int:
    texts: list = ["a\nb", "c\nd\ne"]
    count: int = 0
    for text in texts:
        count = count + len(text.splitlines())
    return count
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_046() {
        let result = transpile(r#"
def splitlines_assign() -> None:
    text: str = "multi\nline\ntext"
    lines: list = text.splitlines()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_047() {
        let result = transpile(r#"
def splitlines_conditional() -> list:
    s: str = "test\nlines"
    if "\n" in s:
        return s.splitlines()
    return [s]
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_048() {
        let result = transpile(r#"
def splitlines_windows() -> list:
    s: str = "a\r\nb\r\nc"
    return s.splitlines()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_049() {
        let result = transpile(r#"
def splitlines_iterate() -> str:
    s: str = "first\nsecond\nthird"
    lines: list = s.splitlines()
    result: str = ""
    for line in lines:
        result = result + line
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    #[test]
    fn test_w22md_050() {
        let result = transpile(r#"
def splitlines_single() -> list:
    s: str = "single line"
    return s.splitlines()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("splitlines"));
    }

    // String methods: partition (tests 51-60)
    #[test]
    fn test_w22md_051() {
        let result = transpile(r#"
def partition_basic() -> tuple:
    s: str = "hello-world"
    return s.partition("-")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_052() {
        let result = transpile(r#"
def partition_not_found() -> tuple:
    s: str = "hello"
    return s.partition("-")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_053() {
        let result = transpile(r#"
def partition_literal() -> tuple:
    return "a:b:c".partition(":")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_054() {
        let result = transpile(r#"
def partition_multiple() -> tuple:
    s: str = "one,two,three"
    return s.partition(",")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_055() {
        let result = transpile(r#"
def partition_var_sep() -> tuple:
    s: str = "key=value"
    sep: str = "="
    return s.partition(sep)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_056() {
        let result = transpile(r#"
def partition_loop() -> list:
    items: list = ["a:b", "c:d", "e:f"]
    result: list = []
    for item in items:
        result.append(item.partition(":"))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_057() {
        let result = transpile(r#"
def partition_unpack() -> str:
    s: str = "name=john"
    before: str
    sep: str
    after: str
    before, sep, after = s.partition("=")
    return after
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_058() {
        let result = transpile(r#"
def partition_conditional() -> tuple:
    s: str = "test@example.com"
    if "@" in s:
        return s.partition("@")
    return (s, "", "")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_059() {
        let result = transpile(r#"
def partition_empty() -> tuple:
    s: str = ""
    return s.partition(",")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("partition"));
    }

    #[test]
    fn test_w22md_060() {
        let result = transpile(r#"
def partition_assign() -> None:
    text: str = "left|right"
    parts: tuple = text.partition("|")
"#);
        assert!(!result.is_empty());
    }

    // String methods: casefold, isprintable, isupper, islower, etc. (tests 61-70)
    #[test]
    fn test_w22md_061() {
        let result = transpile(r#"
def casefold_basic() -> str:
    s: str = "HELLO"
    return s.casefold()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("casefold"));
    }

    #[test]
    fn test_w22md_062() {
        let result = transpile(r#"
def isprintable_basic() -> bool:
    s: str = "hello"
    return s.isprintable()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isprintable"));
    }

    #[test]
    fn test_w22md_063() {
        let result = transpile(r#"
def isupper_basic() -> bool:
    s: str = "HELLO"
    return s.isupper()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isupper"));
    }

    #[test]
    fn test_w22md_064() {
        let result = transpile(r#"
def islower_basic() -> bool:
    s: str = "hello"
    return s.islower()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("islower"));
    }

    #[test]
    fn test_w22md_065() {
        let result = transpile(r#"
def istitle_basic() -> bool:
    s: str = "Hello World"
    return s.istitle()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("istitle"));
    }

    #[test]
    fn test_w22md_066() {
        let result = transpile(r#"
def isnumeric_basic() -> bool:
    s: str = "12345"
    return s.isnumeric()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isnumeric"));
    }

    #[test]
    fn test_w22md_067() {
        let result = transpile(r#"
def isascii_basic() -> bool:
    s: str = "hello"
    return s.isascii()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isascii"));
    }

    #[test]
    fn test_w22md_068() {
        let result = transpile(r#"
def isdecimal_basic() -> bool:
    s: str = "123"
    return s.isdecimal()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isdecimal"));
    }

    #[test]
    fn test_w22md_069() {
        let result = transpile(r#"
def isidentifier_basic() -> bool:
    s: str = "valid_name"
    return s.isidentifier()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isidentifier"));
    }

    #[test]
    fn test_w22md_070() {
        let result = transpile(r#"
def format_basic() -> str:
    s: str = "Hello {}"
    return s.format("world")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("format"));
    }

    // Set methods: add, remove, discard (tests 71-90)
    #[test]
    fn test_w22md_071() {
        let result = transpile(r#"
def set_add_string_literal() -> None:
    s: set = {"apple", "banana"}
    s.add("cherry")
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_072() {
        let result = transpile(r#"
def set_add_variable() -> None:
    s: set = {1, 2, 3}
    item: int = 4
    s.add(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_073() {
        let result = transpile(r#"
def set_add_loop() -> None:
    s: set = set()
    items: list = [1, 2, 3]
    for item in items:
        s.add(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_074() {
        let result = transpile(r#"
def set_remove_literal() -> None:
    s: set = {"a", "b", "c"}
    s.remove("b")
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_075() {
        let result = transpile(r#"
def set_remove_variable() -> None:
    s: set = {10, 20, 30}
    item: int = 20
    s.remove(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_076() {
        let result = transpile(r#"
def set_discard_literal() -> None:
    s: set = {"x", "y", "z"}
    s.discard("y")
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_077() {
        let result = transpile(r#"
def set_discard_variable() -> None:
    s: set = {100, 200, 300}
    item: int = 200
    s.discard(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_078() {
        let result = transpile(r#"
def set_discard_not_present() -> None:
    s: set = {1, 2}
    s.discard(99)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_079() {
        let result = transpile(r#"
def set_clear() -> None:
    s: set = {1, 2, 3, 4, 5}
    s.clear()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_080() {
        let result = transpile(r#"
def set_clear_empty() -> None:
    s: set = set()
    s.clear()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_081() {
        let result = transpile(r#"
def set_add_conditional() -> None:
    s: set = {1, 2}
    x: int = 3
    if x > 0:
        s.add(x)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_082() {
        let result = transpile(r#"
def set_remove_loop() -> None:
    s: set = {1, 2, 3, 4, 5}
    to_remove: list = [2, 4]
    for item in to_remove:
        s.remove(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_083() {
        let result = transpile(r#"
def set_discard_loop() -> None:
    s: set = {"a", "b", "c"}
    to_discard: list = ["a", "d"]
    for item in to_discard:
        s.discard(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_084() {
        let result = transpile(r#"
def set_add_multiple() -> None:
    s: set = set()
    s.add(1)
    s.add(2)
    s.add(3)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_085() {
        let result = transpile(r#"
def set_add_string() -> None:
    s: set = set()
    s.add("hello")
    s.add("world")
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_086() {
        let result = transpile(r#"
def set_clear_conditional() -> None:
    s: set = {1, 2, 3}
    if len(s) > 2:
        s.clear()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_087() {
        let result = transpile(r#"
def set_operations_chained() -> None:
    s: set = {1, 2, 3}
    s.add(4)
    s.remove(1)
    s.discard(99)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_088() {
        let result = transpile(r#"
def set_add_in_expression() -> int:
    s: set = {1, 2}
    s.add(3)
    return len(s)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_089() {
        let result = transpile(r#"
def set_remove_conditional() -> None:
    s: set = {10, 20, 30}
    x: int = 20
    if x in s:
        s.remove(x)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_090() {
        let result = transpile(r#"
def set_discard_conditional() -> None:
    s: set = {"a", "b"}
    x: str = "c"
    if x not in s:
        s.discard(x)
"#);
        assert!(!result.is_empty());
    }

    // Set methods: update, union, intersection, difference (tests 91-110)
    #[test]
    fn test_w22md_091() {
        let result = transpile(r#"
def set_update() -> None:
    s: set = {1, 2, 3}
    other: set = {3, 4, 5}
    s.update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_092() {
        let result = transpile(r#"
def set_update_list() -> None:
    s: set = {1, 2}
    items: list = [3, 4, 5]
    s.update(items)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_093() {
        let result = transpile(r#"
def set_intersection_update() -> None:
    s: set = {1, 2, 3}
    other: set = {2, 3, 4}
    s.intersection_update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_094() {
        let result = transpile(r#"
def set_difference_update() -> None:
    s: set = {1, 2, 3}
    other: set = {2, 3}
    s.difference_update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_095() {
        let result = transpile(r#"
def set_symmetric_difference_result() -> set:
    s: set = {1, 2, 3}
    other: set = {3, 4, 5}
    return s.symmetric_difference(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_096() {
        let result = transpile(r#"
def set_union() -> set:
    s: set = {1, 2}
    other: set = {2, 3}
    return s.union(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w22md_097() {
        let result = transpile(r#"
def set_intersection() -> set:
    s: set = {1, 2, 3}
    other: set = {2, 3, 4}
    return s.intersection(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w22md_098() {
        let result = transpile(r#"
def set_difference() -> set:
    s: set = {1, 2, 3}
    other: set = {2, 3}
    return s.difference(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w22md_099() {
        let result = transpile(r#"
def set_symmetric_difference() -> set:
    s: set = {1, 2, 3}
    other: set = {3, 4, 5}
    return s.symmetric_difference(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w22md_100() {
        let result = transpile(r#"
def set_issubset() -> bool:
    s: set = {1, 2}
    other: set = {1, 2, 3}
    return s.issubset(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("issubset"));
    }

    #[test]
    fn test_w22md_101() {
        let result = transpile(r#"
def set_issuperset() -> bool:
    s: set = {1, 2, 3}
    other: set = {1, 2}
    return s.issuperset(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("issuperset"));
    }

    #[test]
    fn test_w22md_102() {
        let result = transpile(r#"
def set_isdisjoint() -> bool:
    s: set = {1, 2, 3}
    other: set = {4, 5, 6}
    return s.isdisjoint(other)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isdisjoint"));
    }

    #[test]
    fn test_w22md_103() {
        let result = transpile(r#"
def set_update_multiple() -> None:
    s: set = {1}
    s.update({2})
    s.update({3})
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_104() {
        let result = transpile(r#"
def set_operations_loop() -> list:
    sets: list = [{1, 2}, {2, 3}, {3, 4}]
    result: list = []
    for s in sets:
        result.append(s.union({5}))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w22md_105() {
        let result = transpile(r#"
def set_intersection_conditional() -> set:
    s1: set = {1, 2, 3}
    s2: set = {2, 3, 4}
    if len(s1) > 0:
        return s1.intersection(s2)
    return set()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w22md_106() {
        let result = transpile(r#"
def set_difference_assign() -> None:
    s: set = {1, 2, 3, 4}
    other: set = {3, 4}
    result: set = s.difference(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_107() {
        let result = transpile(r#"
def set_union_chain() -> set:
    s1: set = {1}
    s2: set = {2}
    s3: set = {3}
    return s1.union(s2).union(s3)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w22md_108() {
        let result = transpile(r#"
def set_issubset_conditional() -> str:
    s: set = {1, 2}
    other: set = {1, 2, 3, 4}
    if s.issubset(other):
        return "subset"
    return "not subset"
"#);
        assert!(!result.is_empty());
        assert!(result.contains("issubset"));
    }

    #[test]
    fn test_w22md_109() {
        let result = transpile(r#"
def set_issuperset_loop() -> list:
    sets: list = [{1, 2, 3}, {1, 2}, {1}]
    result: list = []
    base: set = {1, 2, 3, 4}
    for s in sets:
        result.append(base.issuperset(s))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("issuperset"));
    }

    #[test]
    fn test_w22md_110() {
        let result = transpile(r#"
def set_isdisjoint_check() -> bool:
    evens: set = {2, 4, 6}
    odds: set = {1, 3, 5}
    return evens.isdisjoint(odds)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("isdisjoint"));
    }

    // Set operations in loops and conditionals (tests 111-130)
    #[test]
    fn test_w22md_111() {
        let result = transpile(r#"
def set_loop_add() -> set:
    s: set = set()
    for i in range(5):
        s.add(i)
    return s
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_112() {
        let result = transpile(r#"
def set_conditional_update() -> None:
    s: set = {1, 2}
    other: set = {3, 4}
    if len(s) < 5:
        s.update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_113() {
        let result = transpile(r#"
def set_nested_loops() -> set:
    result: set = set()
    for i in range(3):
        for j in range(3):
            result.add(i + j)
    return result
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_114() {
        let result = transpile(r#"
def set_while_add() -> set:
    s: set = set()
    i: int = 0
    while i < 5:
        s.add(i)
        i = i + 1
    return s
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_115() {
        let result = transpile(r#"
def set_if_else_operations() -> None:
    s: set = {1, 2, 3}
    x: int = 4
    if x in s:
        s.remove(x)
    else:
        s.add(x)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_116() {
        let result = transpile(r#"
def set_loop_union() -> set:
    sets: list = [{1}, {2}, {3}]
    result: set = set()
    for s in sets:
        result = result.union(s)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w22md_117() {
        let result = transpile(r#"
def set_loop_intersection() -> set:
    s: set = {1, 2, 3, 4, 5}
    filters: list = [{1, 2, 3}, {2, 3, 4}]
    for f in filters:
        s = s.intersection(f)
    return s
"#);
        assert!(!result.is_empty());
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w22md_118() {
        let result = transpile(r#"
def set_comprehension_with_add() -> None:
    s: set = set()
    values: list = [1, 2, 3, 4, 5]
    for v in values:
        if v % 2 == 0:
            s.add(v)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_119() {
        let result = transpile(r#"
def set_multiple_updates() -> None:
    s: set = {1}
    others: list = [{2}, {3}, {4}]
    for other in others:
        s.update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_120() {
        let result = transpile(r#"
def set_conditional_clear() -> None:
    s: set = {1, 2, 3, 4, 5}
    if len(s) > 3:
        s.clear()
    else:
        s.add(6)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_121() {
        let result = transpile(r#"
def set_loop_discard() -> None:
    s: set = {1, 2, 3, 4, 5}
    to_remove: list = [1, 3, 5, 7]
    for item in to_remove:
        s.discard(item)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_122() {
        let result = transpile(r#"
def set_nested_conditional() -> None:
    s: set = {1, 2}
    x: int = 3
    if x > 0:
        if x not in s:
            s.add(x)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_123() {
        let result = transpile(r#"
def set_operations_mixed() -> None:
    s: set = {1, 2, 3}
    s.add(4)
    s.update({5, 6})
    s.discard(1)
    s.remove(2)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_124() {
        let result = transpile(r#"
def set_loop_check_subset() -> list:
    base: set = {1, 2, 3, 4, 5}
    candidates: list = [{1, 2}, {3, 4}, {5, 6}]
    result: list = []
    for c in candidates:
        result.append(c.issubset(base))
    return result
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_125() {
        let result = transpile(r#"
def set_while_remove() -> set:
    s: set = {1, 2, 3, 4, 5}
    while len(s) > 2:
        s.remove(len(s))
    return s
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_126() {
        let result = transpile(r#"
def set_conditional_difference() -> set:
    s1: set = {1, 2, 3}
    s2: set = {2, 3}
    if len(s2) > 0:
        return s1.difference(s2)
    return s1
"#);
        assert!(!result.is_empty());
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w22md_127() {
        let result = transpile(r#"
def set_loop_symmetric_diff() -> list:
    sets: list = [{1, 2}, {2, 3}, {3, 4}]
    result: list = []
    for s in sets:
        result.append(s.symmetric_difference({2}))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w22md_128() {
        let result = transpile(r#"
def set_if_elif_operations() -> None:
    s: set = {1, 2, 3}
    x: int = 4
    if x in s:
        s.remove(x)
    elif x < 10:
        s.add(x)
    else:
        s.clear()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_129() {
        let result = transpile(r#"
def set_loop_disjoint_check() -> int:
    sets: list = [{1, 2}, {3, 4}, {5, 6}]
    base: set = {1, 3, 5}
    count: int = 0
    for s in sets:
        if not s.isdisjoint(base):
            count = count + 1
    return count
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_130() {
        let result = transpile(r#"
def set_complex_operations() -> set:
    s1: set = {1, 2, 3}
    s2: set = {2, 3, 4}
    s3: set = {3, 4, 5}
    result: set = s1.union(s2).intersection(s3)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("union"));
        assert!(result.contains("intersection"));
    }

    // Dict methods: get, keys, values, items (tests 131-160)
    #[test]
    fn test_w22md_131() {
        let result = transpile(r#"
def dict_get_default() -> str:
    d: dict = {"a": 1, "b": 2}
    return d.get("c", "default")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_132() {
        let result = transpile(r#"
def dict_get_default_zero() -> int:
    d: dict = {"x": 10, "y": 20}
    return d.get("z", 0)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_133() {
        let result = transpile(r#"
def dict_get_variable_key() -> int:
    d: dict = {"a": 1, "b": 2}
    key: str = "b"
    return d.get(key, 0)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_134() {
        let result = transpile(r#"
def dict_get_no_default() -> int:
    d: dict = {"x": 100}
    return d.get("x")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_135() {
        let result = transpile(r#"
def dict_keys() -> list:
    d: dict = {"a": 1, "b": 2, "c": 3}
    return list(d.keys())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_136() {
        let result = transpile(r#"
def dict_values() -> list:
    d: dict = {"a": 1, "b": 2, "c": 3}
    return list(d.values())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_137() {
        let result = transpile(r#"
def dict_items() -> list:
    d: dict = {"a": 1, "b": 2}
    return list(d.items())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_138() {
        let result = transpile(r#"
def dict_keys_loop() -> list:
    d: dict = {"x": 1, "y": 2, "z": 3}
    result: list = []
    for key in d.keys():
        result.append(key)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_139() {
        let result = transpile(r#"
def dict_values_loop() -> int:
    d: dict = {"a": 10, "b": 20, "c": 30}
    total: int = 0
    for value in d.values():
        total = total + value
    return total
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_140() {
        let result = transpile(r#"
def dict_items_loop() -> list:
    d: dict = {"a": 1, "b": 2}
    result: list = []
    for key, value in d.items():
        result.append((key, value))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_141() {
        let result = transpile(r#"
def dict_update() -> None:
    d: dict = {"a": 1}
    other: dict = {"b": 2, "c": 3}
    d.update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_142() {
        let result = transpile(r#"
def dict_update_empty() -> None:
    d: dict = {}
    other: dict = {"x": 10}
    d.update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_143() {
        let result = transpile(r#"
def dict_setdefault() -> list:
    d: dict = {"a": [1, 2]}
    result: list = d.setdefault("b", [])
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_144() {
        let result = transpile(r#"
def dict_setdefault_existing() -> int:
    d: dict = {"x": 10}
    result: int = d.setdefault("x", 20)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_145() {
        let result = transpile(r#"
def dict_setdefault_zero() -> int:
    d: dict = {}
    result: int = d.setdefault("count", 0)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_146() {
        let result = transpile(r#"
def dict_popitem() -> tuple:
    d: dict = {"a": 1, "b": 2}
    return d.popitem()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("popitem"));
    }

    #[test]
    fn test_w22md_147() {
        let result = transpile(r#"
def dict_popitem_loop() -> list:
    d: dict = {"a": 1, "b": 2, "c": 3}
    result: list = []
    while len(d) > 0:
        result.append(d.popitem())
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("popitem"));
    }

    #[test]
    fn test_w22md_148() {
        let result = transpile(r#"
def dict_clear() -> None:
    d: dict = {"a": 1, "b": 2, "c": 3}
    d.clear()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_149() {
        let result = transpile(r#"
def dict_get_loop() -> list:
    d: dict = {"a": 1, "b": 2}
    keys: list = ["a", "b", "c"]
    result: list = []
    for key in keys:
        result.append(d.get(key, 0))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_150() {
        let result = transpile(r#"
def dict_get_conditional() -> int:
    d: dict = {"x": 10, "y": 20}
    key: str = "z"
    if key in d:
        return d[key]
    else:
        return d.get(key, 0)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_151() {
        let result = transpile(r#"
def dict_keys_len() -> int:
    d: dict = {"a": 1, "b": 2, "c": 3}
    return len(d.keys())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_152() {
        let result = transpile(r#"
def dict_values_max() -> int:
    d: dict = {"a": 10, "b": 30, "c": 20}
    return max(d.values())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_153() {
        let result = transpile(r#"
def dict_items_filter() -> list:
    d: dict = {"a": 1, "b": 2, "c": 3, "d": 4}
    result: list = []
    for key, value in d.items():
        if value > 2:
            result.append(key)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_154() {
        let result = transpile(r#"
def dict_update_loop() -> None:
    d: dict = {"a": 1}
    updates: list = [{"b": 2}, {"c": 3}]
    for update in updates:
        d.update(update)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_155() {
        let result = transpile(r#"
def dict_setdefault_loop() -> None:
    d: dict = {}
    keys: list = ["a", "b", "c"]
    for key in keys:
        d.setdefault(key, [])
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_156() {
        let result = transpile(r#"
def dict_get_chained() -> int:
    d: dict = {"outer": {"inner": 42}}
    outer: dict = d.get("outer", {})
    return outer.get("inner", 0)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_157() {
        let result = transpile(r#"
def dict_keys_sorted() -> list:
    d: dict = {"c": 3, "a": 1, "b": 2}
    return sorted(d.keys())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_158() {
        let result = transpile(r#"
def dict_values_sum() -> int:
    d: dict = {"a": 10, "b": 20, "c": 30}
    return sum(d.values())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_159() {
        let result = transpile(r#"
def dict_items_dict_comp() -> dict:
    d: dict = {"a": 1, "b": 2, "c": 3}
    result: dict = {}
    for k, v in d.items():
        result[k] = v * 2
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_160() {
        let result = transpile(r#"
def dict_clear_conditional() -> None:
    d: dict = {"a": 1, "b": 2}
    if len(d) > 1:
        d.clear()
"#);
        assert!(!result.is_empty());
    }

    // Dict operations in loops and conditionals (tests 161-180)
    #[test]
    fn test_w22md_161() {
        let result = transpile(r#"
def dict_nested_get() -> int:
    d: dict = {"level1": {"level2": {"level3": 100}}}
    l1: dict = d.get("level1", {})
    l2: dict = l1.get("level2", {})
    return l2.get("level3", 0)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_162() {
        let result = transpile(r#"
def dict_loop_update_values() -> None:
    d: dict = {"a": 1, "b": 2, "c": 3}
    for key in d.keys():
        d[key] = d[key] * 2
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_163() {
        let result = transpile(r#"
def dict_conditional_setdefault() -> None:
    d: dict = {}
    key: str = "items"
    if key not in d:
        d.setdefault(key, [])
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_164() {
        let result = transpile(r#"
def dict_while_popitem() -> int:
    d: dict = {"a": 1, "b": 2, "c": 3}
    count: int = 0
    while len(d) > 0:
        d.popitem()
        count = count + 1
    return count
"#);
        assert!(!result.is_empty());
        assert!(result.contains("popitem"));
    }

    #[test]
    fn test_w22md_165() {
        let result = transpile(r#"
def dict_items_transform() -> dict:
    d: dict = {"a": 1, "b": 2}
    result: dict = {}
    for k, v in d.items():
        result[k.upper()] = v * 10
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_166() {
        let result = transpile(r#"
def dict_get_default_list() -> list:
    d: dict = {"a": [1, 2]}
    return d.get("b", [])
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_167() {
        let result = transpile(r#"
def dict_get_default_dict() -> dict:
    d: dict = {"outer": {"inner": 1}}
    return d.get("missing", {})
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_168() {
        let result = transpile(r#"
def dict_keys_in_check() -> bool:
    d: dict = {"a": 1, "b": 2}
    keys: list = list(d.keys())
    return "a" in keys
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_169() {
        let result = transpile(r#"
def dict_values_all_positive() -> bool:
    d: dict = {"a": 10, "b": 20, "c": 30}
    for value in d.values():
        if value <= 0:
            return False
    return True
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_170() {
        let result = transpile(r#"
def dict_update_conditional() -> None:
    d: dict = {"a": 1}
    other: dict = {"b": 2}
    if len(d) < 5:
        d.update(other)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_171() {
        let result = transpile(r#"
def dict_setdefault_append() -> None:
    d: dict = {}
    key: str = "items"
    d.setdefault(key, []).append(1)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_172() {
        let result = transpile(r#"
def dict_items_nested_loop() -> list:
    dicts: list = [{"a": 1}, {"b": 2}]
    result: list = []
    for d in dicts:
        for k, v in d.items():
            result.append((k, v))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_173() {
        let result = transpile(r#"
def dict_get_multiple() -> list:
    d: dict = {"a": 1, "b": 2, "c": 3}
    keys: list = ["a", "b", "d"]
    result: list = []
    for key in keys:
        result.append(d.get(key, -1))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_174() {
        let result = transpile(r#"
def dict_keys_list_ops() -> list:
    d: dict = {"x": 10, "y": 20, "z": 30}
    keys: list = list(d.keys())
    keys.sort()
    return keys
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_175() {
        let result = transpile(r#"
def dict_values_enumerate() -> list:
    d: dict = {"a": 100, "b": 200}
    result: list = []
    for i, value in enumerate(d.values()):
        result.append((i, value))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_176() {
        let result = transpile(r#"
def dict_update_multiple() -> None:
    d: dict = {"a": 1}
    updates: list = [{"b": 2}, {"c": 3}, {"d": 4}]
    for update in updates:
        d.update(update)
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_177() {
        let result = transpile(r#"
def dict_popitem_conditional() -> tuple:
    d: dict = {"a": 1, "b": 2}
    if len(d) > 1:
        return d.popitem()
    return ("", 0)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("popitem"));
    }

    #[test]
    fn test_w22md_178() {
        let result = transpile(r#"
def dict_clear_if_large() -> None:
    d: dict = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
    if len(d) > 4:
        d.clear()
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_179() {
        let result = transpile(r#"
def dict_items_sum_values() -> int:
    d: dict = {"a": 10, "b": 20, "c": 30}
    total: int = 0
    for key, value in d.items():
        total = total + value
    return total
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_180() {
        let result = transpile(r#"
def dict_setdefault_with_counter() -> None:
    d: dict = {}
    items: list = ["a", "b", "a", "c", "b", "a"]
    for item in items:
        d.setdefault(item, 0)
        d[item] = d[item] + 1
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    // Advanced dict patterns (tests 181-200)
    #[test]
    fn test_w22md_181() {
        let result = transpile(r#"
def dict_nested_setdefault() -> None:
    d: dict = {}
    d.setdefault("users", {}).setdefault("admin", [])
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_182() {
        let result = transpile(r#"
def dict_get_with_calculation() -> int:
    d: dict = {"a": 10, "b": 20}
    return d.get("c", d.get("a", 0) + d.get("b", 0))
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_183() {
        let result = transpile(r#"
def dict_keys_to_set() -> set:
    d: dict = {"a": 1, "b": 2, "c": 3}
    return set(d.keys())
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_184() {
        let result = transpile(r#"
def dict_values_to_list() -> list:
    d: dict = {"x": 100, "y": 200, "z": 300}
    values: list = list(d.values())
    return values
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_185() {
        let result = transpile(r#"
def dict_items_reverse() -> dict:
    d: dict = {"a": 1, "b": 2}
    result: dict = {}
    for k, v in d.items():
        result[v] = k
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_186() {
        let result = transpile(r#"
def dict_update_from_items() -> None:
    d: dict = {}
    items: list = [("a", 1), ("b", 2)]
    for k, v in items:
        d.update({k: v})
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_187() {
        let result = transpile(r#"
def dict_get_nested_default() -> str:
    d: dict = {}
    level1: dict = d.get("l1", {})
    level2: dict = level1.get("l2", {})
    return level2.get("l3", "default")
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_188() {
        let result = transpile(r#"
def dict_popitem_assign() -> None:
    d: dict = {"a": 1, "b": 2}
    key: str
    value: int
    key, value = d.popitem()
"#);
        assert!(!result.is_empty());
        assert!(result.contains("popitem"));
    }

    #[test]
    fn test_w22md_189() {
        let result = transpile(r#"
def dict_keys_filter() -> list:
    d: dict = {"a1": 1, "b2": 2, "a3": 3}
    result: list = []
    for key in d.keys():
        if key.startswith("a"):
            result.append(key)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_190() {
        let result = transpile(r#"
def dict_values_double() -> list:
    d: dict = {"a": 1, "b": 2, "c": 3}
    result: list = []
    for value in d.values():
        result.append(value * 2)
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_191() {
        let result = transpile(r#"
def dict_items_key_value_swap() -> list:
    d: dict = {"name": "john", "age": "30"}
    result: list = []
    for k, v in d.items():
        result.append((v, k))
    return result
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_192() {
        let result = transpile(r#"
def dict_setdefault_list_extend() -> None:
    d: dict = {}
    d.setdefault("nums", [])
    d["nums"].extend([1, 2, 3])
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }

    #[test]
    fn test_w22md_193() {
        let result = transpile(r#"
def dict_get_or_compute() -> int:
    cache: dict = {"a": 10}
    key: str = "b"
    value: int = cache.get(key, 20)
    return value
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_194() {
        let result = transpile(r#"
def dict_update_overwrite() -> None:
    d: dict = {"a": 1, "b": 2}
    d.update({"a": 100, "c": 3})
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_195() {
        let result = transpile(r#"
def dict_clear_and_rebuild() -> None:
    d: dict = {"old": 1}
    d.clear()
    d.update({"new": 2})
"#);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w22md_196() {
        let result = transpile(r#"
def dict_items_join_strings() -> str:
    d: dict = {"name": "john", "age": "30"}
    parts: list = []
    for k, v in d.items():
        parts.append(k + "=" + v)
    return ",".join(parts)
"#);
        assert!(!result.is_empty());
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w22md_197() {
        let result = transpile(r#"
def dict_get_chain_or() -> int:
    d1: dict = {}
    d2: dict = {"x": 10}
    return d1.get("x", d2.get("x", 0))
"#);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w22md_198() {
        let result = transpile(r#"
def dict_keys_intersection() -> set:
    d1: dict = {"a": 1, "b": 2}
    d2: dict = {"b": 3, "c": 4}
    return set(d1.keys()).intersection(set(d2.keys()))
"#);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w22md_199() {
        let result = transpile(r#"
def dict_values_min_max() -> tuple:
    d: dict = {"a": 10, "b": 30, "c": 20}
    values: list = list(d.values())
    return (min(values), max(values))
"#);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w22md_200() {
        let result = transpile(r#"
def dict_setdefault_nested_update() -> None:
    d: dict = {}
    d.setdefault("config", {})
    d["config"]["debug"] = True
"#);
        assert!(!result.is_empty());
        assert!(result.contains("setdefault"));
    }
}
