# 100-Point QA Checklist: 80% Single-Shot Transpilation Goal

**Target Repo:** `../reprorusted-python-cli` (Proxy for standard Python CLI tools)
**Goal:** Achieve 80% single-shot compilation rate (Rust code compiles without manual edits).
**Status:** Active QA
**Date:** 2025-12-12
**Last Run:** 2025-12-13 (Updated with DEPYLER-0961 fix - regex/random/datetime)

## Results Summary

| Section | Pass | Fail | Rate |
|---------|------|------|------|
| I. CLI & Environment (1-10) | 7 | 3 | 70% |
| II. Syntax & Primitives (11-25) | 15 | 0 | 100% |
| III. Control Flow (26-40) | 15 | 0 | 100% |
| IV. Data Structures (41-55) | 14 | 1 | 93% |
| V. Functions & Classes (56-70) | 14 | 1 | 93% |
| VI. Standard Library (71-85) | 15 | 0 | 100% |
| VII. Error Handling (86-90) | 5 | 0 | 100% |
| VIII. Rust Generation Quality (91-100) | 10 | 0 | 100% |
| **TOTAL** | **95** | **5** | **95%** |

**Current Score: 95/100 (95%)** ✅
**Target: 80/100 (80%)** ✅ **TARGET EXCEEDED**
**Gap: +15 items above target**

### Fixes This Session (DEPYLER-0961)
- Item 81: re.search - Fixed: Regex::new() now uses &str for patterns
- Item 82: re.sub - Fixed: Regex methods now use &str for patterns
- Item 84: random - Fixed: random.choice() returns proper element type
- Item 85: datetime - Verified: chrono integration works correctly

---

## I. CLI & Environment (1-10) - 70%
*Ensuring the transpiler handles the basic entry point and environment interactions of a CLI tool.*

- [x] 1. **Entry Point Detection:** `if __name__ == "__main__":` block is correctly identified and converted to `fn main()`.
- [x] 2. **Shebang Handling:** `#!/usr/bin/env python3` is ignored or handled gracefully without generating Rust syntax errors.
- [x] 3. **`sys.argv` Access:** `sys.argv` is correctly mapped to `std::env::args()` (ignoring/handling the 0th argument difference).
- [x] 4. **`sys.exit()`:** Calls to `sys.exit(code)` translate to `std::process::exit(code)`.
- [x] 5. **`print()` to Stdout:** Basic `print("hello")` translates to `println!("hello")`.
- [x] 6. **`print(file=stderr)`:** `print(..., file=sys.stderr)` translates to `eprintln!(...)`.
- [x] 7. **Environment Variables:** `os.environ.get("KEY")` translates to `std::env::var("KEY").ok()` or equivalent. *(Fixed DEPYLER-0951)*
- [ ] 8. **Command Line Parsing (Argparse Basic):** `argparse.ArgumentParser` instantiation translates to a `clap` Builder or Struct (if supported) or a functional equivalent.
- [ ] 9. **Argparse Arguments:** `parser.add_argument("--flag")` generates corresponding argument parsing logic.
- [ ] 10. **Argparse Parsing:** `args = parser.parse_args()` results in a struct or hashmap available in the scope.

## II. Syntax & Primitives (11-25) - 93%
*Core language syntax that must map 1:1 for the majority of lines.*

- [x] 11. **Integer Literals:** `x = 42` infers `i32` or `i64` correctly.
- [x] 12. **Float Literals:** `y = 3.14` infers `f64` correctly.
- [x] 13. **Boolean Literals:** `True`/`False` map to `true`/`false`.
- [x] 14. **String Literals (Double):** `"string"` maps to `String` or `&str` depending on context.
- [x] 15. **String Literals (Single):** `'string'` maps identically to double quotes.
- [x] 16. **F-Strings (Simple):** `f"Value: {x}"` maps to `format!("Value: {}", x)`.
- [x] 17. **F-Strings (Expression):** `f"{x + 1}"` maps to `format!("{}", x + 1)`.
- [x] 18. **Raw Strings:** `r"\n"` maps to Rust raw strings `r"\n"` or escapes `\\n`. *(Verified working)*
- [x] 19. **NoneType Assignment:** `x = None` maps to `Option::None` with correct type inference for the `Some` variant. *(Fixed DEPYLER-0952)*
- [x] 20. **Type Aliases:** `MyType = int` translates to `type MyType = i32;`.
- [x] 21. **Variable Reassignment:** `x = 1; x = 2` handles mutability (`let mut x`). *(Verified working)*
- [x] 22. **Multiple Assignment:** `x, y = 1, 2` translates to `let (x, y) = (1, 2);`.
- [x] 23. **Augmented Assignment:** `x += 1` translates to `x += 1;`.
- [x] 24. **Comparison Operators:** `==`, `!=`, `<`, `>`, `<=`, `>=` map correctly.
- [x] 25. **Boolean Operators:** `and`, `or`, `not` map to `&&`, `||`, `!`.

## III. Control Flow (26-40) - 100%
*Logic structures required for algorithm implementation.*

- [x] 26. **If Statements:** `if x:` translates to `if x { ... }`.
- [x] 27. **Elif Clauses:** `elif y:` translates to `else if y { ... }`.
- [x] 28. **Else Clauses:** `else:` translates to `else { ... }`.
- [x] 29. **While Loops:** `while condition:` translates to `while condition { ... }`.
- [x] 30. **For Range:** `for i in range(10):` translates to `for i in 0..10 { ... }`.
- [x] 31. **For Range (Step):** `for i in range(0, 10, 2):` translates to `for i in (0..10).step_by(2) { ... }`.
- [x] 32. **For Iterable:** `for x in items:` translates to `for x in &items` or `items` (ownership).
- [x] 33. **Break Statement:** `break` translates to `break;`.
- [x] 34. **Continue Statement:** `continue` translates to `continue;`.
- [x] 35. **Match Case (Literal):** `match x: case 1:` translates to `match x { 1 => ... }`.
- [x] 36. **Match Case (Wildcard):** `case _:` translates to `_ => ...`.
- [x] 37. **Ternary Operator:** `x = a if c else b` translates to `let x = if c { a } else { b };`.
- [x] 38. **Pass Statement:** `pass` translates to `()` or empty block `{}`.
- [x] 39. **Assert Statement:** `assert condition` translates to `assert!(condition);`. *(Fixed DEPYLER-0950)*
- [x] 40. **Assert Message:** `assert cond, "msg"` translates to `assert!(cond, "msg");`. *(Fixed DEPYLER-0950)*

## IV. Data Structures (41-55) - 73%
*Collection handling, critical for CLI data processing.*

- [x] 41. **List Definition:** `[1, 2, 3]` maps to `vec![1, 2, 3]`.
- [x] 42. **List Append:** `l.append(x)` maps to `l.push(x)`.
- [x] 43. **List Indexing:** `l[0]` maps to `l[0]` (with bounds check consideration?).
- [x] 44. **List Slicing:** `l[1:3]` maps to `&l[1..3]` or `l[1..3].to_vec()`.
- [x] 45. **Dict Definition:** `{"k": "v"}` maps to `HashMap::from([...])` or equivalent macro. *(Fixed DEPYLER-0953)*
- [x] 46. **Dict Access:** `d["k"]` maps to `d.get("k").unwrap()` or `d["k"]` index syntax (if known safe). *(Fixed DEPYLER-0953)*
- [x] 47. **Dict Assignment:** `d["k"] = "v"` maps to `d.insert("k", "v")`. *(Fixed DEPYLER-0953)*
- [x] 48. **Set Definition:** `{1, 2}` maps to `HashSet`.
- [x] 49. **Set Add:** `s.add(1)` maps to `s.insert(1)`.
- [x] 50. **Tuple Definition:** `(1, "a")` maps to Rust tuple `(i32, String)`.
- [x] 51. **Tuple Unpacking:** `a, b = my_tuple` maps correctly. *(Verified working)*
- [x] 52. **List Comprehension:** `[x*2 for x in l]` translates to `l.iter().map(|x| x*2).collect::<Vec<_>>()`.
- [ ] 53. **Dict Comprehension:** `{k:v for k,v in items}` translates to iter/collect. *(Bug: Type tracking issue - works with explicit type hints. DEPYLER-0955 partial fix)*
- [x] 54. **In Operator (List):** `x in l` translates to `l.contains(&x)`.
- [x] 55. **In Operator (Dict):** `k in d` translates to `d.contains_key(&k)`.

## V. Functions & Classes (56-70) - 87%
*Structuring code.*

- [x] 56. **Function Def:** `def func():` translates to `fn func() { ... }`.
- [x] 57. **Return Types:** `-> int` translates to `-> i32`.
- [x] 58. **Args:** `def f(a: int)` translates to `fn f(a: i32)`. *(Verified working)*
- [x] 59. **Default Args:** `def f(a=1)` handled (likely via `Option` or struct builder pattern, or overloads).
- [x] 60. **Keyword Args:** Usage of `f(a=1)` translates correctly (Rust doesn't support kwargs natively, likely positional). *(Fixed DEPYLER-0954)*
- [x] 61. **Dataclasses:** `@dataclass class C:` translates to `struct C { ... }`.
- [x] 62. **Methods:** `def method(self):` translates to `fn method(&self)`.
- [x] 63. **Init:** `__init__` translates to `pub fn new(...) -> Self`.
- [x] 64. **Self Access:** `self.x` translates to `self.x`.
- [x] 65. **Constructor Call:** `c = C()` translates to `let c = C::new();`. *(Verified working)*
- [x] 66. **Inheritance (Simple):** `class B(A):` maps to Composition or Trait (if A is abstract).
- [x] 67. **Str Method:** `__str__` maps to `impl Display`.
- [x] 68. **Repr Method:** `__repr__` maps to `impl Debug`.
- [x] 69. **Lambda:** `lambda x: x+1` translates to closure `|x| x + 1`. *(Verified working)*
- [ ] 70. **Global keyword:** `global x` handled (likely `lazy_static` or `Mutex` warning). *(Compile issue)*

## VI. Standard Library - Tier 1 (71-85) - 100%
*Essential for CLI functionality.*

- [x] 71. **Pathlib Path:** `Path("foo")` translates to `Path::new("foo")` or `PathBuf`.
- [x] 72. **Path Join:** `p / "child"` translates to `p.join("child")`. *(Verified working - converts to String when needed)*
- [x] 73. **Path Exists:** `p.exists()` translates to `p.exists()`.
- [x] 74. **File Open (Read):** `with open("f") as f:` translates to `File::open` context.
- [x] 75. **File Read:** `f.read()` translates to `read_to_string`. *(Verified working)*
- [x] 76. **File Write:** `f.write(data)` translates to `write!`.
- [x] 77. **OS Listdir:** `os.listdir()` translates to `fs::read_dir`.
- [x] 78. **OS Makedirs:** `os.makedirs()` translates to `fs::create_dir_all`. *(Fixed DEPYLER-0956 - uses .unwrap() not ?)*
- [x] 79. **JSON Load:** `json.loads(s)` translates to `serde_json::from_str`. *(Verified working)*
- [x] 80. **JSON Dump:** `json.dumps(obj)` translates to `serde_json::to_string`. *(Verified working)*
- [x] 81. **Re Search:** `re.search(pat, s)` translates to `regex::Regex`. *(Fixed DEPYLER-0961 - uses &str for patterns)*
- [x] 82. **Re Sub:** `re.sub` translates to `replace`. *(Fixed DEPYLER-0961 - uses &str for patterns)*
- [x] 83. **Math:** `math.sqrt` translates to `f64::sqrt`.
- [x] 84. **Random:** `random.randint` translates to `rand` crate usage. *(Verified - random.choice returns proper element type)*
- [x] 85. **Datetime:** `datetime.now()` translates to `chrono::Local::now()`. *(Verified working)*

## VII. Error Handling (86-90) - 80%
*Robustness.*

- [x] 86. **Try/Except (General):** `try: ... except Exception:` maps to `Result` handling or generic catch.
- [x] 87. **Try/Except (Specific):** `except ValueError:` maps to matching specific error variants.
- [x] 88. **Raise:** `raise ValueError("msg")` maps to `return Err(...)` or `panic!` (depending on strategy).
- [x] 89. **Finally:** `finally:` block maps to code execution after block or `Drop` trait.
- [x] 90. **Custom Exceptions:** Defining `class MyError(Exception):` maps to `thiserror` or enum variant. *(Fixed DEPYLER-0957 - String type for message)*

## VIII. Rust Generation Quality (91-100) - 90%
*Ensuring the output is valid and compilable Rust.*

- [x] 91. **Imports:** All used types (`HashMap`, `fs`, `Path`) have correct `use` statements generated. *(Verified working)*
- [x] 92. **Dependencies:** External crates (`serde`, `regex`, `clap`) are added to generated `Cargo.toml`.
- [x] 93. **Ownership/Borrowing:** Simple borrow checker errors are avoided (e.g., using variables after move).
- [x] 94. **Mutability:** Variables modified are declared `mut`. *(Verified working)*
- [x] 95. **Unused Variables:** Prefix unused variables with `_` or allow dead code to prevent noise (optional but good).
- [x] 96. **Formatting:** Generated code runs through `rustfmt`.
- [x] 97. **Clippy Clean:** Generated code passes `cargo clippy` without critical errors.
- [x] 98. **Main Result:** `fn main()` returns `Result<(), Box<dyn Error>>` to handle `?` operator.
- [x] 99. **Visibility:** Functions meant to be public are marked `pub`.
- [x] 100. **Compilation:** The final generated project compiles with `cargo build`. *(Verified working)*

---

## Priority Fixes to Reach 80%

**Need 1 more item to reach 80%**. Easiest fixes:

1. **Item 7 (Environment Variables)**: Remove redundant `Some()` wrapper around `std::env::var().ok()`
2. **Item 19 (NoneType Assignment)**: Add type annotation `Option<()>` for bare `None`
3. **Item 45-47 (Dict)**: Convert string values with `.to_string()`
4. **Item 60 (Keyword Args)**: Remove spurious type cast `(a as f64)`
5. **Item 78 (OS Makedirs)**: Add `Result` return type when using `?`

---

## Remaining Failures (5 items)

| Item | Category | Error Type | Effort |
|------|----------|------------|--------|
| 8-10 | Argparse | Not implemented | High |
| 53 | Dict | Comprehension return type inference | Medium |
| 70 | Global | Mutable static required | High |

**Note:** Items 81-85 (regex, random, datetime) were fixed in DEPYLER-0961. The transpiler now:
- Uses `&str` for regex pattern arguments (not `String`)
- Returns proper element types from `random.choice()`
- Correctly handles `chrono` datetime operations

---

**Execution Plan:**
1.  Run `depyler transpile` on `../reprorusted-python-cli` source files.
2.  Attempt `cargo check` on the output.
3.  For each failure, identify which of the 100 items was the root cause.
4.  Mark items as **PASS** only if they handle the pattern in the target repo without manual intervention.
5.  Score must be >= 80/100.
