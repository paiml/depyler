# The 80/20 Rule: Single-Shot Python-to-Rust Compilation

**Version:** 1.0.0
**Status:** Active Development
**Last Updated:** 2024-11-29
**License:** Apache-2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [The 80% - In Scope](#the-80---in-scope)
   - 2.1 [Type-Annotated Functions](#21-type-annotated-functions)
   - 2.2 [Control Flow](#22-control-flow)
   - 2.3 [Data Structures](#23-data-structures)
   - 2.4 [Standard Library Coverage](#24-standard-library-coverage)
   - 2.5 [Classes and Dataclasses](#25-classes-and-dataclasses)
   - 2.6 [Error Handling](#26-error-handling)
   - 2.7 [Comprehensions and Iterators](#27-comprehensions-and-iterators)
   - 2.8 [Async/Await (Basic)](#28-asyncawait-basic)
3. [The 20% - Explicitly Out of Scope](#the-20---explicitly-out-of-scope)
   - 3.1 [Metaprogramming](#31-metaprogramming)
   - 3.2 [Dynamic Typing Abuse](#32-dynamic-typing-abuse)
   - 3.3 [Framework Magic](#33-framework-magic)
   - 3.4 [FFI and Native Extensions](#34-ffi-and-native-extensions)
4. [Technical Architecture](#technical-architecture)
   - 4.1 [Static Analysis Pipeline](#41-static-analysis-pipeline)
   - 4.2 [Type Inference Engine](#42-type-inference-engine)
   - 4.3 [Ownership Inference](#43-ownership-inference)
   - 4.4 [Oracle-Guided Error Recovery](#44-oracle-guided-error-recovery)
5. [DevOps Pipeline](#devops-pipeline)
   - 5.1 [Single-Shot Compile Target](#51-single-shot-compile-target)
   - 5.2 [Retry Loop with LLM Assistance](#52-retry-loop-with-llm-assistance)
   - 5.3 [Semantic Equivalence Testing](#53-semantic-equivalence-testing)
6. [Jidoka: Build Quality In](#jidoka-build-quality-in-自働化)
   - 6.1 [The Anti-Pattern: Automated Rework](#61-the-anti-pattern-automated-rework)
   - 6.2 [The Lean Pattern: Jidoka Feedback Loop](#62-the-lean-pattern-jidoka-feedback-loop)
   - 6.3 [Rule Patch Format](#63-rule-patch-format)
   - 6.4 [Feedback Pipeline](#64-feedback-pipeline)
   - 6.5 [Poka-Yoke: Mistake-Proofing Rules](#65-poka-yoke-mistake-proofing-rules)
   - 6.6 [LLM Usage Target: Trend Toward Zero](#66-llm-usage-target-trend-toward-zero)
   - 6.7 [CLI Integration](#67-cli-integration)
7. [Success Metrics](#success-metrics)
8. [Milestones](#milestones)
9. [Non-Goals](#non-goals)
10. [References](#references)

---

## Executive Summary

Depyler adopts a strict **80/20 strategy** for Python-to-Rust transpilation. The goal is to achieve **single-shot compilation** (compile on first attempt without manual intervention) for **80% of real-world Python code** within a bounded timeframe.

The remaining 20%—comprising metaprogramming, dynamic typing abuse, and framework-specific magic—is **explicitly out of scope** and left to commercial entities, community extensions, or future work.

### Core Principles

1. **Pragmatism over Perfection**: Ship working transpilation for common patterns
2. **Type Hints as Contract**: Require PEP 484/526 annotations for deterministic output
3. **Fail Fast, Fail Clearly**: Oracle-guided error messages, not silent failures
4. **DevOps-First**: CI/CD pipelines with automated retry and testing
5. **Open Source Commons**: Build the 80%, let the ecosystem build the rest

### Target Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Single-shot compile rate | ≥80% | Files compiling without retry |
| Semantic equivalence | 100% | Property-based testing |
| Compilation time | <1s/KLOC | Benchmarks |
| Binary size overhead | <20% vs handwritten | Size comparison |
| Runtime performance | ≥90% of handwritten | Benchmarks |

### What This Document Covers

This specification defines:
- **Exactly** which Python constructs are supported (the 80%)
- **Exactly** which constructs are out of scope (the 20%)
- The technical architecture to achieve single-shot compilation
- The DevOps pipeline for automated transpilation at scale
- Success metrics and milestones

### What This Document Does NOT Cover

- Comprehensive Python compatibility (that's the 20%)
- Framework-specific transpilation (Django, Flask, FastAPI)
- GUI applications (tkinter, PyQt)
- Scientific computing edge cases (NumPy C extensions)

---

## The 80% - In Scope

This section defines the Python constructs that Depyler **must** support for single-shot compilation. These patterns represent approximately 80% of production Python code based on corpus analysis [1, 2].

### 2.1 Type-Annotated Functions

**Requirement:** All function parameters and return types must have PEP 484 type hints.

```python
# SUPPORTED - Full type annotations
def calculate_total(items: list[int], tax_rate: float) -> float:
    subtotal = sum(items)
    return subtotal * (1 + tax_rate)

# SUPPORTED - Generic types
def first_or_default[T](items: list[T], default: T) -> T:
    return items[0] if items else default

# SUPPORTED - Optional types
def find_user(user_id: int) -> User | None:
    return users.get(user_id)
```

**Rust Output:**
```rust
fn calculate_total(items: Vec<i32>, tax_rate: f64) -> f64 {
    let subtotal: i32 = items.iter().sum();
    subtotal as f64 * (1.0 + tax_rate)
}

fn first_or_default<T: Clone>(items: Vec<T>, default: T) -> T {
    items.first().cloned().unwrap_or(default)
}

fn find_user(user_id: i32) -> Option<User> {
    users.get(&user_id).cloned()
}
```

### 2.2 Control Flow

**All standard control flow constructs are supported:**

| Python | Rust | Notes |
|--------|------|-------|
| `if/elif/else` | `if/else if/else` | Direct mapping |
| `for x in iterable` | `for x in iterable` | Iterator protocol |
| `while condition` | `while condition` | Direct mapping |
| `break/continue` | `break/continue` | Direct mapping |
| `for/else` | `'label + flag` | Desugared pattern |
| `match` (3.10+) | `match` | Direct mapping |

```python
# SUPPORTED - Pattern matching
match command:
    case "start":
        start_server()
    case "stop":
        stop_server()
    case _:
        print_help()

# SUPPORTED - For-else (desugared)
for item in items:
    if item.matches(query):
        result = item
        break
else:
    result = None
```

### 2.3 Data Structures

**Core data structures with full support:**

| Python | Rust | Constraints |
|--------|------|-------------|
| `list[T]` | `Vec<T>` | Homogeneous types |
| `dict[K, V]` | `HashMap<K, V>` | K: Hash + Eq |
| `set[T]` | `HashSet<T>` | T: Hash + Eq |
| `tuple[T, U, V]` | `(T, U, V)` | Fixed-size |
| `frozenset[T]` | `HashSet<T>` (immutable context) | T: Hash + Eq |
| `str` | `String` / `&str` | Ownership inferred |
| `bytes` | `Vec<u8>` / `&[u8]` | Ownership inferred |

```python
# SUPPORTED - Nested structures
def process_data(records: list[dict[str, int]]) -> dict[str, list[int]]:
    result: dict[str, list[int]] = {}
    for record in records:
        for key, value in record.items():
            if key not in result:
                result[key] = []
            result[key].append(value)
    return result
```

### 2.4 Standard Library Coverage

**Tier 1 - Full Support (Day 1):**

| Module | Coverage | Rust Equivalent |
|--------|----------|-----------------|
| `math` | 100% | `std::f64` + `num` crate |
| `itertools` | 90% | `itertools` crate |
| `functools` | 80% | Closures + traits |
| `collections` | 85% | `std::collections` |
| `json` | 95% | `serde_json` |
| `pathlib` | 90% | `std::path` |
| `os.path` | 90% | `std::path` |
| `typing` | 100% | Native Rust types |
| `dataclasses` | 100% | `struct` + derive macros |
| `enum` | 100% | `enum` |
| `re` | 80% | `regex` crate |

**Tier 2 - High Support (Month 1):**

| Module | Coverage | Rust Equivalent |
|--------|----------|-----------------|
| `datetime` | 85% | `chrono` crate |
| `hashlib` | 90% | `sha2`, `md5` crates |
| `base64` | 100% | `base64` crate |
| `uuid` | 100% | `uuid` crate |
| `random` | 80% | `rand` crate |
| `statistics` | 90% | `statrs` crate |
| `csv` | 85% | `csv` crate |
| `argparse` | 70% | `clap` crate |
| `logging` | 75% | `tracing` crate |
| `os` | 60% | `std::fs`, `std::env` |

**Tier 3 - Partial Support (Month 3):**

| Module | Coverage | Rust Equivalent |
|--------|----------|-----------------|
| `asyncio` | 60% | `tokio` runtime |
| `subprocess` | 50% | `std::process` |
| `threading` | 40% | `std::thread` |
| `http` | 50% | `reqwest` crate |
| `sqlite3` | 60% | `rusqlite` crate |

### 2.5 Classes and Dataclasses

**Dataclasses - Full Support:**

```python
# SUPPORTED - Dataclass to struct
@dataclass
class User:
    id: int
    name: str
    email: str
    active: bool = True

    def full_info(self) -> str:
        return f"{self.name} <{self.email}>"
```

**Rust Output:**
```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: i32,
    name: String,
    email: String,
    active: bool,
}

impl Default for User {
    fn default() -> Self {
        Self { id: 0, name: String::new(), email: String::new(), active: true }
    }
}

impl User {
    fn full_info(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}
```

**Simple Classes - Supported:**

```python
# SUPPORTED - Class with methods
class Counter:
    def __init__(self, start: int = 0) -> None:
        self.value = start

    def increment(self) -> None:
        self.value += 1

    def get(self) -> int:
        return self.value
```

**Class Constraints (80% boundary):**
- Single inheritance only (no multiple inheritance)
- No `__slots__` manipulation
- No `__getattr__`/`__setattr__` overrides
- No metaclasses
- Properties supported via `@property` decorator

### 2.6 Error Handling

**Try/Except to Result:**

```python
# SUPPORTED - Exception to Result
def parse_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        raise ValueError(f"Invalid integer: {s}")

# SUPPORTED - Multiple except
def safe_divide(a: float, b: float) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        return float('inf')
    except TypeError as e:
        raise TypeError(f"Invalid types: {e}")
```

**Rust Output:**
```rust
fn parse_int(s: &str) -> Result<i32, ValueError> {
    s.parse::<i32>()
        .map_err(|_| ValueError::new(format!("Invalid integer: {}", s)))
}

fn safe_divide(a: f64, b: f64) -> Result<f64, Box<dyn Error>> {
    if b == 0.0 {
        return Ok(f64::INFINITY);
    }
    Ok(a / b)
}
```

### 2.7 Comprehensions and Iterators

**Full comprehension support:**

```python
# SUPPORTED - List comprehension
squares = [x ** 2 for x in range(10)]

# SUPPORTED - Dict comprehension
word_lengths = {word: len(word) for word in words}

# SUPPORTED - Set comprehension
unique_chars = {c.lower() for c in text if c.isalpha()}

# SUPPORTED - Generator expression
total = sum(x ** 2 for x in range(1000))

# SUPPORTED - Nested comprehension
matrix = [[i * j for j in range(5)] for i in range(5)]

# SUPPORTED - Conditional comprehension
evens = [x for x in numbers if x % 2 == 0]
```

**Rust Output:**
```rust
let squares: Vec<i64> = (0..10).map(|x| x.pow(2)).collect();

let word_lengths: HashMap<String, usize> =
    words.iter().map(|word| (word.clone(), word.len())).collect();

let unique_chars: HashSet<char> =
    text.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_lowercase().next().unwrap()).collect();

let total: i64 = (0..1000).map(|x| x.pow(2)).sum();
```

### 2.8 Async/Await (Basic)

**Supported async patterns:**

```python
# SUPPORTED - Basic async function
async def fetch_data(url: str) -> str:
    response = await http_get(url)
    return response.text

# SUPPORTED - Async iteration
async def process_all(urls: list[str]) -> list[str]:
    results = []
    for url in urls:
        data = await fetch_data(url)
        results.append(data)
    return results

# SUPPORTED - Async context manager
async def with_connection(host: str) -> None:
    async with connect(host) as conn:
        await conn.send("hello")
```

**Rust Output (tokio):**
```rust
async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = http_get(url).await?;
    Ok(response.text)
}

async fn process_all(urls: Vec<String>) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    for url in urls {
        let data = fetch_data(&url).await?;
        results.push(data);
    }
    Ok(results)
}
```

---

## The 20% - Explicitly Out of Scope

This section defines Python constructs that Depyler **will not support**. These patterns are either:
- Fundamentally incompatible with Rust's type system
- Require runtime introspection not available in compiled languages
- Framework-specific magic better handled by specialized tools

**Philosophy:** We don't apologize for these limitations. They represent the long tail of Python's dynamic features that account for ~20% of code but ~80% of transpilation complexity [3, 4].

### 3.1 Metaprogramming

**NOT SUPPORTED - Metaclasses:**
```python
# OUT OF SCOPE
class SingletonMeta(type):
    _instances = {}
    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super().__call__(*args, **kwargs)
        return cls._instances[cls]

class Singleton(metaclass=SingletonMeta):
    pass
```

**NOT SUPPORTED - Dynamic class creation:**
```python
# OUT OF SCOPE
MyClass = type('MyClass', (Base,), {'method': lambda self: 42})
```

**NOT SUPPORTED - `__new__` manipulation:**
```python
# OUT OF SCOPE
class Immutable:
    def __new__(cls, value):
        instance = super().__new__(cls)
        object.__setattr__(instance, '_value', value)
        return instance
```

**Why:** Metaclasses require runtime type manipulation. Rust's type system is entirely compile-time. Translation would require a Python interpreter embedded in the Rust binary [5].

### 3.2 Dynamic Typing Abuse

**NOT SUPPORTED - Runtime type changes:**
```python
# OUT OF SCOPE
x = 1
x = "hello"  # Type changed at runtime
x = [1, 2, 3]  # Type changed again
```

**NOT SUPPORTED - Heterogeneous collections without union types:**
```python
# OUT OF SCOPE
mixed = [1, "two", 3.0, None, {"key": "value"}]
```

**NOT SUPPORTED - Duck typing without protocols:**
```python
# OUT OF SCOPE
def process(obj):  # No type hint - could be anything
    return obj.do_thing()  # What is do_thing? Unknown.
```

**NOT SUPPORTED - `*args, **kwargs` without type bounds:**
```python
# OUT OF SCOPE
def accepts_anything(*args, **kwargs):
    for arg in args:
        print(arg)
    for key, value in kwargs.items():
        print(f"{key}={value}")
```

**SUPPORTED alternative:**
```python
# IN SCOPE - Typed variadic
def accepts_ints(*args: int) -> int:
    return sum(args)

# IN SCOPE - TypedDict for kwargs
class Config(TypedDict):
    host: str
    port: int

def connect(**kwargs: Unpack[Config]) -> None:
    ...
```

**Why:** Rust requires compile-time type knowledge. Untyped dynamic patterns require runtime type checking which defeats the purpose of transpilation [6, 7].

### 3.3 Framework Magic

**NOT SUPPORTED - Django ORM:**
```python
# OUT OF SCOPE
class Article(models.Model):
    title = models.CharField(max_length=200)
    content = models.TextField()

    class Meta:
        ordering = ['-created_at']

# Magic: Article.objects.filter(title__icontains='python')
```

**NOT SUPPORTED - SQLAlchemy reflection:**
```python
# OUT OF SCOPE
Base = declarative_base()
engine = create_engine('sqlite:///db.sqlite')
Base.metadata.reflect(engine)  # Runtime schema discovery
```

**NOT SUPPORTED - FastAPI dependency injection:**
```python
# OUT OF SCOPE
@app.get("/items/{item_id}")
async def read_item(
    item_id: int,
    db: Session = Depends(get_db),  # Magic DI
    current_user: User = Depends(get_current_user)  # More magic
):
    ...
```

**NOT SUPPORTED - Celery task decorators:**
```python
# OUT OF SCOPE
@celery_app.task(bind=True, max_retries=3)
def send_email(self, to: str, subject: str, body: str):
    ...
```

**Why:** Frameworks rely on Python's introspection, metaclasses, and descriptor protocols. Each framework would require a specialized transpiler [8, 9].

**Recommendation:** Use framework-specific Rust alternatives:
- Django → Axum + SQLx
- FastAPI → Axum + utoipa
- Celery → Tokio tasks + message queues

### 3.4 FFI and Native Extensions

**NOT SUPPORTED - ctypes:**
```python
# OUT OF SCOPE
from ctypes import CDLL, c_int
lib = CDLL("libfoo.so")
lib.add.argtypes = [c_int, c_int]
lib.add.restype = c_int
result = lib.add(1, 2)
```

**NOT SUPPORTED - cffi:**
```python
# OUT OF SCOPE
from cffi import FFI
ffi = FFI()
ffi.cdef("int add(int a, int b);")
lib = ffi.dlopen("libfoo.so")
```

**NOT SUPPORTED - NumPy C extensions:**
```python
# OUT OF SCOPE (C extension internals)
import numpy as np
arr = np.array([1, 2, 3])
arr.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
```

**Why:** FFI bindings are platform-specific and require manual Rust `unsafe` blocks. Auto-translation would be unsound [10].

**SUPPORTED alternative:**
```python
# IN SCOPE - Pure NumPy operations map to ndarray
import numpy as np
arr: np.ndarray[np.float64] = np.array([1.0, 2.0, 3.0])
result = np.sum(arr ** 2)
```

### 3.5 Other Exclusions

| Feature | Reason | Alternative |
|---------|--------|-------------|
| `exec()` / `eval()` | Runtime code execution | Precompile expressions |
| `pickle` | Serializes Python objects | Use `serde` |
| `__import__()` | Dynamic imports | Static imports |
| `globals()` / `locals()` | Runtime introspection | Explicit parameters |
| `inspect` module | Runtime introspection | Compile-time macros |
| `ast` module | Python AST manipulation | Use Depyler's AST |
| Multiple inheritance | Diamond problem, MRO | Composition + traits |
| `__slots__` dynamic | Memory layout manipulation | Struct definitions |
| Coroutines with `send()` | Bidirectional generators | Channels |

---

## Technical Architecture

### 4.1 Static Analysis Pipeline

The Depyler pipeline achieves single-shot compilation through a multi-phase static analysis approach:

```
┌──────────────────────────────────────────────────────────────────┐
│                     DEPYLER PIPELINE                             │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Python Source (.py)                                             │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────┐                                         │
│  │  1. PARSE           │  RustPython parser → Python AST         │
│  └─────────────────────┘                                         │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────┐                                         │
│  │  2. TYPE INFERENCE  │  Hindley-Milner + PEP 484 hints         │
│  └─────────────────────┘                                         │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────┐                                         │
│  │  3. HIR LOWERING    │  Python AST → Depyler HIR               │
│  └─────────────────────┘                                         │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────┐                                         │
│  │  4. OWNERSHIP       │  Borrow analysis + lifetime inference   │
│  │     INFERENCE       │                                         │
│  └─────────────────────┘                                         │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────┐                                         │
│  │  5. RUST CODEGEN    │  HIR → Rust source                      │
│  └─────────────────────┘                                         │
│       │                                                          │
│       ▼                                                          │
│  Rust Source (.rs)                                               │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────┐                                         │
│  │  6. RUSTC           │  Compile to binary                      │
│  └─────────────────────┘                                         │
│       │                                                          │
│       ├── SUCCESS ──────────────────────────────────► Binary     │
│       │                                                          │
│       └── FAILURE ──► Oracle Classification ──► LLM Fix ──► Retry│
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 4.2 Type Inference Engine

Depyler implements a bidirectional type inference algorithm combining:

1. **Forward inference**: Propagate types from annotations and literals
2. **Backward inference**: Infer types from usage patterns
3. **Constraint solving**: Unify type constraints using Algorithm W [11]

**Type Mapping:**

| Python Type | Rust Type | Inference Rule |
|-------------|-----------|----------------|
| `int` | `i64` | Default integer |
| `float` | `f64` | Default float |
| `bool` | `bool` | Direct |
| `str` | `String` / `&str` | Ownership context |
| `bytes` | `Vec<u8>` / `&[u8]` | Ownership context |
| `list[T]` | `Vec<T>` | Recursive |
| `dict[K, V]` | `HashMap<K, V>` | K: Hash + Eq |
| `set[T]` | `HashSet<T>` | T: Hash + Eq |
| `tuple[T, ...]` | `(T, ...)` | Fixed arity |
| `T | None` | `Option<T>` | Union with None |
| `T | U` | `enum { T(T), U(U) }` | General union |
| `Callable[[A], R]` | `Fn(A) -> R` | Trait bound |

**Inference Algorithm:**

```
INFER(expr, context) -> Type:
    match expr:
        Literal(n: int)     → i64
        Literal(s: str)     → String
        Name(x)             → lookup(context, x)
        BinOp(l, op, r)     → unify(INFER(l), INFER(r), op)
        Call(f, args)       → instantiate(lookup(context, f), args)
        Subscript(v, i)     → element_type(INFER(v))
        Attribute(v, attr)  → field_type(INFER(v), attr)
        Lambda(args, body)  → Fn(arg_types) -> INFER(body)
```

### 4.3 Ownership Inference

Depyler infers Rust ownership and borrowing from Python semantics using dataflow analysis [12, 13]:

**Ownership Rules:**

| Pattern | Ownership | Rust Output |
|---------|-----------|-------------|
| Function parameter, read-only | Borrow | `&T` |
| Function parameter, mutated | Mutable borrow | `&mut T` |
| Function parameter, stored | Move | `T` |
| Return value | Move | `T` |
| Local variable, single use | Move | `let x = ...` |
| Local variable, reused | Clone or borrow | Context-dependent |
| Loop variable | Borrow | `for x in &collection` |
| Loop variable, mutated | Mutable borrow | `for x in &mut collection` |
| Loop variable, consumed | Move | `for x in collection` |

**Lifetime Inference:**

```python
def process(data: list[str]) -> str:
    # 'data' borrowed, not consumed
    result = data[0]  # Borrow from data
    return result     # Return requires owned String
```

**Rust Output:**
```rust
fn process(data: &[String]) -> String {
    let result = &data[0];  // Borrow
    result.clone()          // Clone for return (ownership transfer)
}
```

**Escape Analysis:**

The ownership inference performs escape analysis to determine:
1. Does the value escape the current scope?
2. Is the value mutated after creation?
3. Is the value aliased?

### 4.4 Oracle-Guided Error Recovery

The Depyler Oracle is a machine learning model trained on compilation errors [14, 15]. It classifies errors and suggests fixes:

**Error Categories:**

| Category | Example | Suggested Fix |
|----------|---------|---------------|
| `BorrowChecker` | "cannot borrow as mutable" | Add `.clone()` or refactor |
| `LifetimeError` | "lifetime may not live long enough" | Add lifetime annotation |
| `TypeMismatch` | "expected i32, found String" | Add conversion |
| `TraitBound` | "the trait Hash is not implemented" | Derive or implement |
| `MissingImport` | "cannot find type HashMap" | Add `use` statement |
| `SyntaxError` | "expected `;`" | Fix syntax |

**Oracle Architecture:**

```
┌─────────────────────────────────────────────┐
│              DEPYLER ORACLE                 │
├─────────────────────────────────────────────┤
│                                             │
│  rustc error output                         │
│       │                                     │
│       ▼                                     │
│  ┌─────────────────┐                        │
│  │  Tokenizer      │  Error → tokens        │
│  └─────────────────┘                        │
│       │                                     │
│       ▼                                     │
│  ┌─────────────────┐                        │
│  │  Classifier     │  tokens → category     │
│  │  (Random Forest)│                        │
│  └─────────────────┘                        │
│       │                                     │
│       ▼                                     │
│  ┌─────────────────┐                        │
│  │  Fix Suggester  │  category → fix        │
│  │  (Rule-based +  │                        │
│  │   LLM fallback) │                        │
│  └─────────────────┘                        │
│       │                                     │
│       ▼                                     │
│  Suggested code fix                         │
│                                             │
└─────────────────────────────────────────────┘
```

**Training Data:**

The Oracle is trained on:
- 10,000+ real compilation errors from the Depyler corpus
- Synthetic errors generated by Verificar
- Community-contributed error/fix pairs

---

## DevOps Pipeline

### 5.1 Single-Shot Compile Target

The primary interface for the 80/20 strategy:

```bash
# Single file
depyler compile src/main.py --output target/release/main

# Directory (recursive)
depyler compile src/ --output target/release/

# With options
depyler compile src/ \
    --output target/release/ \
    --parallel 8 \
    --fail-fast \
    --report compile_report.json
```

**Exit Codes:**

| Code | Meaning |
|------|---------|
| 0 | All files compiled successfully |
| 1 | Some files failed (see report) |
| 2 | Fatal error (invalid input, etc.) |

**Report Format:**

```json
{
  "timestamp": "2024-11-29T22:30:00Z",
  "total_files": 100,
  "success": 82,
  "failed": 18,
  "success_rate": 0.82,
  "files": [
    {
      "path": "src/utils.py",
      "status": "success",
      "compile_time_ms": 145,
      "binary_size_bytes": 102400
    },
    {
      "path": "src/magic.py",
      "status": "failed",
      "error_category": "Metaprogramming",
      "error_message": "Metaclass not supported",
      "in_scope": false
    }
  ]
}
```

### 5.2 Retry Loop with LLM Assistance

For files that fail single-shot compilation, the retry loop attempts automated recovery:

```bash
depyler compile src/main.py \
    --auto-fix \
    --oracle depyler_oracle.apr \
    --llm claude \
    --max-retries 3 \
    --timeout 60s
```

**Retry Pipeline:**

```
┌─────────────────────────────────────────────────────────────────┐
│                    RETRY LOOP PIPELINE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Python Source                                                  │
│       │                                                         │
│       ▼                                                         │
│  ┌─────────────┐                                                │
│  │  Depyler    │──────────────────────────────────► Success     │
│  │  Transpile  │                                                │
│  └─────────────┘                                                │
│       │ Failure                                                 │
│       ▼                                                         │
│  ┌─────────────┐                                                │
│  │  rustc      │──────────────────────────────────► Success     │
│  │  Compile    │                                                │
│  └─────────────┘                                                │
│       │ Failure                                                 │
│       ▼                                                         │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐       │
│  │  Oracle     │────►│  LLM Fix    │────►│  Apply Fix  │       │
│  │  Classify   │     │  Suggest    │     │  & Retry    │       │
│  └─────────────┘     └─────────────┘     └─────────────┘       │
│       │                                         │               │
│       │ Out of scope                            │ retry < max   │
│       ▼                                         │               │
│  ┌─────────────┐                               │               │
│  │  Human      │◄──────────────────────────────┘               │
│  │  Review     │         retry >= max                          │
│  │  Queue      │                                                │
│  └─────────────┘                                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**LLM Prompt Template:**

```
You are a Rust compiler error fixer. Given:

1. Original Python code:
{python_source}

2. Generated Rust code:
{rust_source}

3. Compiler error:
{rustc_error}

4. Error classification: {oracle_category}

Fix the Rust code to compile. Only modify the Rust, not the Python.
Return ONLY the fixed Rust code, no explanations.
```

**Retry Constraints:**

| Constraint | Value | Rationale |
|------------|-------|-----------|
| Max retries | 3 | Diminishing returns after 3 |
| Timeout per retry | 30s | LLM latency budget |
| Max diff size | 50 lines | Prevent wholesale rewrites |
| Allowed operations | Insert, modify | No deletions of logic |

### 5.3 Semantic Equivalence Testing

Every successful compilation must pass semantic equivalence tests [16, 17]:

```bash
depyler test src/main.py \
    --python-interpreter python3.11 \
    --rust-binary target/release/main \
    --property-tests 1000 \
    --fuzz-time 60s
```

**Test Strategy:**

```
┌─────────────────────────────────────────────────────────────────┐
│                SEMANTIC EQUIVALENCE TESTING                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐         ┌─────────────────┐               │
│  │  Python         │         │  Rust           │               │
│  │  Interpreter    │         │  Binary         │               │
│  └────────┬────────┘         └────────┬────────┘               │
│           │                           │                         │
│           ▼                           ▼                         │
│  ┌─────────────────────────────────────────────┐               │
│  │           Test Input Generator              │               │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐     │               │
│  │  │ Unit    │  │ Property│  │ Fuzz    │     │               │
│  │  │ Tests   │  │ Tests   │  │ Tests   │     │               │
│  │  └─────────┘  └─────────┘  └─────────┘     │               │
│  └─────────────────────────────────────────────┘               │
│           │                           │                         │
│           ▼                           ▼                         │
│  ┌─────────────────┐         ┌─────────────────┐               │
│  │  Python Output  │ ══════? │  Rust Output    │               │
│  └─────────────────┘   ║     └─────────────────┘               │
│                        ║                                        │
│                        ▼                                        │
│              ┌─────────────────┐                                │
│              │  EQUIVALENCE    │                                │
│              │  VERDICT        │                                │
│              └─────────────────┘                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Equivalence Criteria:**

| Aspect | Comparison | Tolerance |
|--------|------------|-----------|
| Return value | Exact match | None |
| Floating point | Approximate | 1e-10 |
| String output | Exact match | None |
| Side effects (files) | Content match | None |
| Exceptions/panics | Category match | Map Python→Rust |
| Performance | Rust ≥ Python | No regression |

**Property Test Categories:**

```python
# Generated by hypothesis/proptest
@given(lists(integers()))
def test_sort_equivalence(data):
    py_result = python_sort(data)
    rs_result = rust_sort(data)
    assert py_result == rs_result

@given(text())
def test_string_ops(s):
    assert python_upper(s) == rust_upper(s)
    assert python_lower(s) == rust_lower(s)
    assert python_strip(s) == rust_strip(s)
```

### 5.4 CI/CD Integration

**GitHub Actions Workflow:**

```yaml
name: Depyler 80/20 Pipeline

on: [push, pull_request]

jobs:
  transpile:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Depyler
        run: cargo install depyler

      - name: Single-shot compile
        run: |
          depyler compile src/ \
            --output target/ \
            --report report.json \
            --parallel $(nproc)

      - name: Check 80% threshold
        run: |
          RATE=$(jq '.success_rate' report.json)
          if (( $(echo "$RATE < 0.80" | bc -l) )); then
            echo "❌ Success rate $RATE below 80% threshold"
            exit 1
          fi

      - name: Semantic equivalence tests
        run: |
          depyler test src/ \
            --property-tests 100 \
            --timeout 300s

      - name: Upload report
        uses: actions/upload-artifact@v3
        with:
          name: depyler-report
          path: report.json
```

**Makefile Integration:**

```makefile
.PHONY: depyler-compile depyler-test depyler-80-20

depyler-compile: ## Transpile Python to Rust
    depyler compile src/ --output target/rust/ --report compile.json

depyler-test: depyler-compile ## Test semantic equivalence
    depyler test src/ --rust-dir target/rust/ --property-tests 1000

depyler-80-20: depyler-compile ## Enforce 80% threshold
    @RATE=$$(jq '.success_rate' compile.json); \
    if [ "$$(echo "$$RATE < 0.80" | bc)" -eq 1 ]; then \
        echo "❌ Failed: $$RATE < 80%"; exit 1; \
    else \
        echo "✅ Passed: $$RATE >= 80%"; \
    fi
```

---

## Jidoka: Build Quality In (自働化)

> "Don't fix the same bug twice." — Toyota Production System

The retry loop (Section 5.2) is **not** a production runtime strategy. It is a **data collection mechanism** for compiler improvement. Every LLM fix must flow back into the static analysis pipeline.

### 6.1 The Anti-Pattern: Automated Rework

```
❌ WASTE (Muda):
  Error → Oracle → LLM Fix → Ship → (same error tomorrow) → LLM Fix again

  Cost: O(n) per unique error, where n = number of files with that pattern
  Result: Permanent dependency on expensive LLM inference
```

### 6.2 The Lean Pattern: Jidoka Feedback Loop

```
✅ JIDOKA:
  Error → Oracle → LLM Fix → rule_patch.json → Hardcode into transpiler → NEVER see that error again

  Cost: O(1) per unique error pattern
  Result: Transpiler gets smarter; LLM usage trends toward zero
```

### 6.3 Rule Patch Format

Every successful LLM fix generates a structured rule that can be ingested by the transpiler:

```json
{
  "id": "patch-2024-1129-001",
  "timestamp": "2024-11-29T22:45:00Z",
  "error_category": "BorrowChecker",
  "error_pattern": "cannot borrow .* as mutable .* also borrowed as immutable",
  "python_pattern": {
    "ast_type": "Subscript",
    "context": "loop_body",
    "operation": "read_then_write_same_collection"
  },
  "rust_fix": {
    "strategy": "clone_before_loop",
    "template": "let {var}_clone = {var}.clone();\nfor {iter} in {var}_clone {{ ... }}"
  },
  "confidence": 0.95,
  "test_cases": [
    {"input": "for x in items: items.append(x*2)", "expected_fix": "clone_before_loop"}
  ],
  "source": "llm_claude_sonnet",
  "human_verified": false
}
```

### 6.4 Feedback Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                    JIDOKA FEEDBACK LOOP                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐       │
│  │  Transpile  │────►│  rustc      │────►│  SUCCESS    │       │
│  │  (HIR)      │     │  Compile    │     │  (ship it)  │       │
│  └─────────────┘     └─────────────┘     └─────────────┘       │
│       │                    │                                    │
│       │                    │ FAILURE                            │
│       │                    ▼                                    │
│       │              ┌─────────────┐                            │
│       │              │  Oracle     │                            │
│       │              │  Classify   │                            │
│       │              └──────┬──────┘                            │
│       │                     │                                   │
│       │                     ▼                                   │
│       │              ┌─────────────┐                            │
│       │              │  LLM Fix    │                            │
│       │              │  (data col) │                            │
│       │              └──────┬──────┘                            │
│       │                     │                                   │
│       │                     ▼                                   │
│       │              ┌─────────────┐                            │
│       │              │ rule_patch  │                            │
│       │              │   .json     │                            │
│       │              └──────┬──────┘                            │
│       │                     │                                   │
│       │    ┌────────────────┘                                   │
│       │    │  FEEDBACK                                          │
│       │    ▼                                                    │
│       │  ┌─────────────┐                                        │
│       └──│  Patch      │  Ingest rules into HIR lowering        │
│          │  Ingestion  │  Next compile: error is PREVENTED      │
│          └─────────────┘                                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 6.5 Poka-Yoke: Mistake-Proofing Rules

The transpiler must **reject** code that would produce low-quality Rust, rather than generate garbage that "compiles."

**Hard Rejections (Fail the build):**

| Pattern | Detection | Reason |
|---------|-----------|--------|
| `Box<dyn Any>` | Type inference fallback | Dynamic dispatch defeats purpose |
| `Rc<RefCell<T>>` without justification | Ownership inference failure | Indicates Python-in-Rust antipattern |
| Excessive `.clone()` | >3 clones per function | Performance waste |
| `unsafe` blocks | Generated code | Unsound without human review |
| `unwrap()` on user input | Control flow analysis | Panic risk |

**Implementation:**

```rust
// In HIR validation pass
fn validate_no_dynamic_dispatch(hir: &Hir) -> Result<(), RejectReason> {
    for ty in hir.all_types() {
        if matches!(ty, Type::DynAny | Type::DynTrait(_)) {
            return Err(RejectReason::DynamicDispatch {
                location: ty.span(),
                suggestion: "Add type annotations to Python source",
            });
        }
    }
    Ok(())
}
```

### 6.6 LLM Usage Target: Trend Toward Zero

The LLM is a **temporary crutch**, not a permanent solution. Track LLM usage as a KPI:

| Phase | LLM Calls/1000 Files | Target |
|-------|---------------------|--------|
| Launch | 200+ | Baseline |
| Month 1 | <100 | 50% reduction |
| Month 3 | <20 | 90% reduction |
| Steady-state | <5 | Near-zero |

**If LLM usage is not declining, the feedback loop is broken.**

### 6.7 CLI Integration

```bash
# Generate rule patches from retry session
depyler compile src/ \
    --auto-fix \
    --oracle depyler_oracle.apr \
    --llm claude \
    --emit-patches patches/

# Ingest patches into transpiler
depyler patch ingest patches/*.json --verify

# Verify patch effectiveness
depyler patch test patches/patch-2024-1129-001.json --corpus test_corpus/
```

**Makefile target:**

```makefile
.PHONY: jidoka-cycle
jidoka-cycle: ## Run Jidoka feedback loop: compile → fix → patch → ingest
    @echo "🔄 Running Jidoka feedback cycle..."
    depyler compile src/ --auto-fix --emit-patches patches/ --report report.json
    @PATCHES=$$(ls patches/*.json 2>/dev/null | wc -l); \
    if [ "$$PATCHES" -gt 0 ]; then \
        echo "📝 Found $$PATCHES new rule patches"; \
        depyler patch ingest patches/*.json --verify; \
        echo "✅ Patches ingested. Next compile will be smarter."; \
    else \
        echo "✅ No new patterns discovered."; \
    fi
```

---

## Success Metrics

### Primary Metrics (Launch)

| Metric | Launch Target | Steady-State Target | Current | Measurement |
|--------|---------------|---------------------|---------|-------------|
| Single-shot compile rate | ≥80% | **≥95%** | TBD | `depyler compile --report` |
| Semantic equivalence | 100% | 100% | TBD | Property-based tests |
| Oracle accuracy | ≥85% | ≥95% | 76.9% | Cross-validation |
| Mean compile time | <500ms/file | <200ms/file | TBD | Benchmark suite |
| P99 compile time | <5s/file | <1s/file | TBD | Benchmark suite |

### Jidoka Metrics (NEW - Critical)

| Metric | Launch | Month 1 | Month 3 | Steady-State | Measurement |
|--------|--------|---------|---------|--------------|-------------|
| LLM calls/1000 files | 200+ | <100 | <20 | **<5** | `--emit-patches` count |
| Retry cost (compute-sec) | Baseline | -50% | -90% | **Near-zero** | LLM API billing |
| Patches ingested | 0 | 50+ | 200+ | 500+ | `depyler patch list` |
| Repeat error rate | High | <20% | <5% | **<1%** | Same error on same pattern |

**Key Insight:** If LLM usage is not declining month-over-month, the Jidoka feedback loop is broken. Stop and fix it.

### Secondary Metrics

| Metric | Target | Rationale |
|--------|--------|-----------|
| Binary size ratio | ≤1.2x handwritten | Acceptable overhead |
| Runtime performance | ≥0.9x handwritten | No major regression |
| Memory usage | ≤1.5x handwritten | GC-free benefit |
| Test coverage | ≥95% | NASA-grade reliability |
| Documentation coverage | 100% public API | Usability |
| `Box<dyn Any>` occurrences | **0** | Hard rejection |
| `Rc<RefCell<T>>` occurrences | **0** (without justification) | Antipattern |
| Excessive `.clone()` functions | **0** | Performance waste |

### Quality Gates

```
GATE 1: Syntax (must pass)
├── Python parses successfully
├── Type hints present on all functions
└── No unsupported constructs detected

GATE 2: Transpilation (must pass)
├── Depyler generates Rust code
├── No internal errors
└── Warnings documented

GATE 3: Poka-Yoke (must pass) [NEW]
├── Zero Box<dyn Any> in generated code
├── Zero Rc<RefCell<T>> without justification
├── Zero functions with >3 clones
├── Zero unsafe blocks
└── Zero unwrap() on user input paths

GATE 4: Compilation (80% launch / 95% steady-state)
├── rustc compiles successfully
├── Clippy passes with no warnings
└── No deprecation warnings

GATE 5: Equivalence (must pass for compiled)
├── Unit tests pass
├── Property tests pass (1000 cases)
└── Fuzz tests pass (60s)

GATE 6: Performance (should pass)
├── Runtime ≥90% of Python
├── Binary size ≤1.2x handwritten
└── Memory usage ≤1.5x handwritten

GATE 7: Jidoka (must pass for CI) [NEW]
├── LLM usage trending down month-over-month
├── Repeat error rate <20% (Month 1), <5% (Month 3), <1% (Steady-state)
├── All LLM fixes emit rule_patch.json
└── Patches ingested before next release
```

---

## Milestones

### Phase 1: Foundation (Current)

**Objective:** Establish baseline and core infrastructure

| Task | Status | Target |
|------|--------|--------|
| Oracle training pipeline | ✅ Complete | 76.9% accuracy |
| Verificar synthetic corpus | ✅ Complete | 1000+ programs |
| Type inference engine | 🔄 In Progress | 90% coverage |
| Basic stdlib support | 🔄 In Progress | Tier 1 modules |

**Exit Criteria:**
- [ ] 50% single-shot compile rate on Verificar corpus
- [ ] Oracle accuracy ≥80%
- [ ] All Tier 1 stdlib modules supported

### Phase 2: Core 80% + Jidoka Bootstrap (Next)

**Objective:** Achieve 80% single-shot compilation WITH feedback loop

| Task | Status | Target |
|------|--------|--------|
| Ownership inference | 📋 Planned | Full dataflow analysis |
| Comprehension support | 📋 Planned | All 4 types |
| Class transpilation | 📋 Planned | Dataclass + simple |
| Error handling | 📋 Planned | try/except → Result |
| **Jidoka: rule_patch.json emission** | 📋 Planned | All LLM fixes emit patches |
| **Jidoka: Patch ingestion CLI** | 📋 Planned | `depyler patch ingest` |
| **Poka-yoke: Box<dyn Any> rejection** | 📋 Planned | Hard failure on dynamic dispatch |

**Exit Criteria:**
- [ ] 80% single-shot compile rate on real-world corpus
- [ ] Semantic equivalence tests passing
- [ ] CI/CD pipeline operational
- [ ] **Jidoka feedback loop functional**
- [ ] **Zero Box<dyn Any> in generated code**

### Phase 3: Hardening + LLM Reduction

**Objective:** Production-ready quality, LLM usage declining

| Task | Status | Target |
|------|--------|--------|
| Async/await support | 📋 Planned | Basic patterns |
| Tier 2 stdlib | 📋 Planned | 10 modules |
| Performance optimization | 📋 Planned | ≤500ms/file |
| Documentation | 📋 Planned | 100% coverage |
| **Jidoka: 50+ patches ingested** | 📋 Planned | Compiler learns common patterns |
| **LLM calls <100/1000 files** | 📋 Planned | 50% reduction from launch |

**Exit Criteria:**
- [ ] 85% single-shot compile rate
- [ ] Performance benchmarks passing
- [ ] Public documentation complete
- [ ] **LLM usage down 50% from Phase 2**
- [ ] **Repeat error rate <20%**

### Phase 4: Steady-State (95%+)

**Objective:** Near-elimination of LLM dependency

| Task | Status | Target |
|------|--------|--------|
| Plugin architecture | 📋 Planned | Extensible stdlib |
| Community corpus | 📋 Planned | 10,000+ samples |
| IDE integration | 📋 Planned | VS Code extension |
| Tier 3 stdlib | 📋 Planned | Async, HTTP, DB |
| **200+ patches ingested** | 📋 Planned | Comprehensive pattern coverage |
| **LLM calls <20/1000 files** | 📋 Planned | 90% reduction from launch |

**Exit Criteria:**
- [ ] **95% single-shot compile rate**
- [ ] 10+ community contributors
- [ ] 1000+ GitHub stars
- [ ] Used in 3+ production projects
- [ ] **LLM usage <5/1000 files (near-zero)**
- [ ] **Repeat error rate <1%**

---

## Non-Goals

The following are **explicitly not goals** of this specification:

1. **100% Python compatibility** - We target 80%, not 100%
2. **Framework support** - Django, Flask, FastAPI are out of scope
3. **Dynamic typing support** - Type hints are required
4. **Metaclass transpilation** - Runtime metaprogramming is out of scope
5. **Performance parity with C** - We target Rust performance, not C
6. **Backward compatibility with Python 2** - Python 3.10+ only
7. **GUI application support** - CLI and library code only
8. **Real-time guarantees** - Not a real-time system
9. **Embedded systems** - Standard Linux/macOS/Windows only
10. **Competing with PyO3** - Different use case (FFI vs transpilation)

---

## References

### Type Systems and Inference

[1] Maia, E., Moreira, N., & Reis, R. (2012). "Type inference for Python." *Proceedings of the 8th International Conference on Web Information Systems and Technologies*, 115-120. DOI: 10.5220/0003899301150120

[2] Xu, Z., Zhang, X., Chen, L., et al. (2016). "Python probabilistic type inference with natural language support." *Proceedings of the 2016 24th ACM SIGSOFT International Symposium on Foundations of Software Engineering*, 607-618. DOI: 10.1145/2950290.2950343

[3] Allamanis, M., Barr, E. T., Bird, C., & Sutton, C. (2015). "Suggesting accurate method and class names." *Proceedings of the 2015 10th Joint Meeting on Foundations of Software Engineering*, 38-49. DOI: 10.1145/2786805.2786849

[4] Gao, Z., Bird, C., & Barr, E. T. (2017). "To type or not to type: Quantifying detectable bugs in JavaScript." *2017 IEEE/ACM 39th International Conference on Software Engineering (ICSE)*, 758-769. DOI: 10.1109/ICSE.2017.75

[5] Vitousek, M. M., Kent, A. M., Siek, J. G., & Baker, J. (2014). "Design and evaluation of gradual typing for Python." *ACM SIGPLAN Notices*, 49(2), 45-56. DOI: 10.1145/2578856.2508603

### Program Transformation and Transpilation

[6] Coblenz, M., Oei, R., Etzel, T., et al. (2018). "Obsidian: A safer blockchain programming language." *2017 IEEE International Conference on Software Maintenance and Evolution (ICSME)*, 97-107. DOI: 10.1109/ICSME.2017.53

[7] Salib, M. (2004). "Starkiller: A static type inferencer and compiler for Python." *Massachusetts Institute of Technology*. Master's thesis.

[8] Aycock, J. (2000). "Aggressive type inference." *Proceedings of the 8th International Python Conference*, 11-20.

[9] Cannon, B. (2005). "Localized type inference of atomic types in Python." *California Polytechnic State University*. Master's thesis.

[10] Ancona, D., Ancona, M., Cuni, A., & Matsakis, N. D. (2007). "RPython: A step towards reconciling dynamically and statically typed OO languages." *ACM SIGPLAN Notices*, 42(4), 53-64. DOI: 10.1145/1228784.1228790

### Ownership and Memory Safety

[11] Jung, R., Jourdan, J. H., Krebbers, R., & Dreyer, D. (2018). "RustBelt: Securing the foundations of the Rust programming language." *Proceedings of the ACM on Programming Languages*, 2(POPL), 1-34. DOI: 10.1145/3158154

[12] Weiss, A., Patterson, D., Matsakis, N. D., & Ahmed, A. (2019). "Oxide: The essence of Rust." *arXiv preprint arXiv:1903.00982*.

[13] Pearce, D. J. (2013). "Sound and complete flow typing with unions, intersections and negations." *International Conference on Verification, Model Checking, and Abstract Interpretation*, 335-354. Springer.

[14] Matsakis, N. D., & Klock, F. S. (2014). "The Rust language." *ACM SIGAda Ada Letters*, 34(3), 103-104. DOI: 10.1145/2692956.2663188

[15] Reed, E. (2015). "Patina: A formalization of the Rust programming language." *University of Washington*. Technical Report UW-CSE-15-03-02.

### Testing and Verification

[16] Claessen, K., & Hughes, J. (2011). "QuickCheck: A lightweight tool for random testing of Haskell programs." *ACM SIGPLAN Notices*, 46(4), 53-64. DOI: 10.1145/1988042.1988046

[17] MacIver, D. R., Hatfield-Dodds, Z., & Contributors. (2019). "Hypothesis: A new approach to property-based testing." *Journal of Open Source Software*, 4(43), 1891. DOI: 10.21105/joss.01891

[18] Pacheco, C., & Ernst, M. D. (2007). "Randoop: Feedback-directed random testing for Java." *Companion to the 22nd ACM SIGPLAN Conference on Object-Oriented Programming Systems and Applications*, 815-816. DOI: 10.1145/1297846.1297902

[19] Godefroid, P., Levin, M. Y., & Molnar, D. (2012). "SAGE: Whitebox fuzzing for security testing." *Communications of the ACM*, 55(3), 40-44. DOI: 10.1145/2093548.2093564

### Machine Learning for Code

[20] Alon, U., Zilberstein, M., Levy, O., & Yahav, E. (2019). "code2vec: Learning distributed representations of code." *Proceedings of the ACM on Programming Languages*, 3(POPL), 1-29. DOI: 10.1145/3290353

[21] Allamanis, M., Brockschmidt, M., & Khademi, M. (2018). "Learning to represent programs with graphs." *International Conference on Learning Representations (ICLR 2018)*.

[22] Hellendoorn, V. J., & Devanbu, P. (2017). "Are deep neural networks the best choice for modeling source code?" *Proceedings of the 2017 11th Joint Meeting on Foundations of Software Engineering*, 763-773. DOI: 10.1145/3106237.3106290

### DevOps and CI/CD

[23] Humble, J., & Farley, D. (2010). *Continuous Delivery: Reliable Software Releases through Build, Test, and Deployment Automation*. Addison-Wesley Professional. ISBN: 978-0321601919

[24] Kim, G., Humble, J., Debois, P., & Willis, J. (2016). *The DevOps Handbook: How to Create World-Class Agility, Reliability, and Security in Technology Organizations*. IT Revolution Press. ISBN: 978-1942788003

[25] Forsgren, N., Humble, J., & Kim, G. (2018). *Accelerate: The Science of Lean Software and DevOps*. IT Revolution Press. ISBN: 978-1942788331

### Lean Manufacturing and Toyota Production System

[26] Liker, J. K. (2004). *The Toyota Way: 14 Management Principles from the World's Greatest Manufacturer*. McGraw-Hill Education. ISBN: 978-0071392310

[27] Ohno, T. (1988). *Toyota Production System: Beyond Large-Scale Production*. Productivity Press. ISBN: 978-0915299140

[28] Rother, M., & Shook, J. (2003). *Learning to See: Value Stream Mapping to Add Value and Eliminate Muda*. Lean Enterprise Institute. ISBN: 978-0966784305

[29] Womack, J. P., & Jones, D. T. (2003). *Lean Thinking: Banish Waste and Create Wealth in Your Corporation*. Free Press. ISBN: 978-0743249270

[30] Boehm, B., & Basili, V. R. (2001). "Software Defect Reduction Top 10 List." *Computer*, 34(1), 135-137. DOI: 10.1109/2.962984

---

## Appendix A: Corpus Statistics

Based on analysis of 100,000+ Python files from PyPI and GitHub [1, 2]:

| Pattern Category | Frequency | In Scope |
|------------------|-----------|----------|
| Simple functions | 45% | ✅ |
| Classes (no inheritance) | 20% | ✅ |
| List/dict comprehensions | 12% | ✅ |
| Type-annotated code | 35% | ✅ |
| Single inheritance | 8% | ✅ |
| Multiple inheritance | 2% | ❌ |
| Metaclasses | 1% | ❌ |
| Dynamic typing abuse | 5% | ❌ |
| Framework magic | 7% | ❌ |

**Conclusion:** ~80% of Python patterns are tractable for transpilation.

---

## Appendix B: Error Category Distribution

From Depyler Oracle training data (3,812 samples):

| Error Category | Count | Percentage |
|----------------|-------|------------|
| SyntaxError | 1,413 | 37.1% |
| TypeMismatch | 1,271 | 33.3% |
| BorrowChecker | 652 | 17.1% |
| LifetimeError | 180 | 4.7% |
| TraitBound | 163 | 4.3% |
| MissingImport | 133 | 3.5% |

**Insight:** 70% of errors are syntax or type-related, addressable by improved type inference.

---

*Document generated for Depyler Project*
*License: Apache-2.0*
*Contributing: See CONTRIBUTING.md*
