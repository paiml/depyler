# pathlib - Object-Oriented Filesystem Paths

Python's pathlib module provides object-oriented filesystem paths, making path manipulation more intuitive and cross-platform. Depyler transpiles pathlib operations to Rust's `std::path::Path` and `std::path::PathBuf` with full type safety and platform compatibility.

## Python → Rust Mapping

| Python Class/Method | Rust Equivalent | Notes |
|---------------------|-----------------|-------|
| `from pathlib import Path` | `use std::path::{Path, PathBuf}` | Path types |
| `Path(str)` | `PathBuf::from(str)` or `Path::new(str)` | Path creation |
| `path.name` | `path.file_name()` | Filename |
| `path.stem` | `path.file_stem()` | Filename without extension |
| `path.suffix` | `path.extension()` | File extension |
| `path.parent` | `path.parent()` | Parent directory |
| `path.parts` | `path.components()` | Path components |
| `path.exists()` | `path.exists()` | Existence check |
| `path.is_file()` | `path.is_file()` | File check |
| `path.is_dir()` | `path.is_dir()` | Directory check |
| `path / "subdir"` | `path.join("subdir")` | Path joining |
| `path.read_text()` | `fs::read_to_string(path)` | Read file |
| `path.write_text(content)` | `fs::write(path, content)` | Write file |
| `path.mkdir()` | `fs::create_dir(path)` | Create directory |
| `path.iterdir()` | `fs::read_dir(path)` | List directory |

## Path Creation and Properties

### Creating Path Objects

Create Path objects from strings and extract properties:

```python
from pathlib import Path

def get_path_properties() -> str:
    # Create path object
    p = Path("/home/user/documents/file.txt")

    # Get various path properties
    name = str(p.name)
    stem = str(p.stem)
    suffix = str(p.suffix)
    parent = str(p.parent)

    # Return concatenated result for verification
    return name + "," + stem + "," + suffix + "," + parent
```

**Generated Rust:**

```rust
use std::path::Path;

fn get_path_properties() -> String {
    // Create path object
    let p = Path::new("/home/user/documents/file.txt");

    // Get various path properties
    let name = p.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let stem = p.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let suffix = p.extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e))
        .unwrap_or_else(|| String::new());

    let parent = p.parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
        .to_string();

    // Return concatenated result for verification
    format!("{},{},{},{}", name, stem, suffix, parent)
}
```

**Path Properties:**

| Property | Description | Example Input | Output |
|----------|-------------|---------------|--------|
| `name` | Full filename | `/home/file.txt` | `file.txt` |
| `stem` | Filename without extension | `/home/file.txt` | `file` |
| `suffix` | File extension (with dot) | `/home/file.txt` | `.txt` |
| `parent` | Parent directory | `/home/file.txt` | `/home` |
| `parts` | All path components | `/home/user/file.txt` | `('/', 'home', 'user', 'file.txt')` |

## Path Checking Operations

### exists(), is_file(), is_dir()

Check path existence and types:

```python
from pathlib import Path

def check_path_types() -> bool:
    # Check current directory
    current = Path(".")
    exists = current.exists()

    # Should be a directory
    is_directory = current.is_dir()

    # Should not be a file
    is_file = current.is_file()

    return exists and is_directory and not is_file
```

**Generated Rust:**

```rust
use std::path::Path;

fn check_path_types() -> bool {
    // Check current directory
    let current = Path::new(".");
    let exists = current.exists();

    // Should be a directory
    let is_directory = current.is_dir();

    // Should not be a file
    let is_file = current.is_file();

    exists && is_directory && !is_file
}
```

**Checking Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `exists()` | `bool` | True if path exists |
| `is_file()` | `bool` | True if regular file |
| `is_dir()` | `bool` | True if directory |
| `is_symlink()` | `bool` | True if symbolic link |
| `is_absolute()` | `bool` | True if absolute path |

**Key Behavior:**
- All methods return `false` for non-existent paths
- Symlinks: `exists()` follows links, `is_symlink()` detects links
- No exceptions for inaccessible paths (returns `false`)

## Path Construction with / Operator

### Joining Paths with /

Build paths using the intuitive / operator:

```python
from pathlib import Path

def construct_paths() -> str:
    # Build path using / operator
    home = Path("/home")
    user_dir = home / "user"
    docs = user_dir / "documents"
    file_path = docs / "file.txt"

    # Convert to string
    result = str(file_path)

    return result
```

**Generated Rust:**

```rust
use std::path::PathBuf;

fn construct_paths() -> String {
    // Build path using join() method
    let mut home = PathBuf::from("/home");
    let mut user_dir = home.clone();
    user_dir.push("user");

    let mut docs = user_dir.clone();
    docs.push("documents");

    let mut file_path = docs.clone();
    file_path.push("file.txt");

    // Convert to string
    file_path.to_string_lossy().to_string()
}
```

**Alternative Rust (more idiomatic):**

```rust
use std::path::PathBuf;

fn construct_paths() -> String {
    let mut path = PathBuf::from("/home");
    path.push("user");
    path.push("documents");
    path.push("file.txt");

    path.to_string_lossy().to_string()
}
```

## File I/O Operations

### read_text() and write_text()

Read and write text files using Path methods:

```python
from pathlib import Path
import tempfile
import os

def test_file_operations() -> str:
    # Create temporary file
    fd, tmp_path = tempfile.mkstemp(suffix=".txt")
    os.close(fd)

    p = Path(tmp_path)

    # Write text to file
    content = "Hello, pathlib!"
    p.write_text(content)

    # Read text back
    read_content = p.read_text()

    # Cleanup
    os.unlink(tmp_path)

    return read_content
```

**Generated Rust:**

```rust
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

fn test_file_operations() -> String {
    // Create temporary file
    let tmp_file = NamedTempFile::new().expect("Failed to create temp file");
    let tmp_path = tmp_file.path();

    // Write text to file
    let content = "Hello, pathlib!";
    fs::write(tmp_path, content).expect("Failed to write file");

    // Read text back
    let read_content = fs::read_to_string(tmp_path)
        .expect("Failed to read file");

    // Cleanup happens automatically with NamedTempFile

    read_content
}
```

**File I/O Methods:**

| Python Method | Rust Equivalent | Description |
|---------------|-----------------|-------------|
| `path.read_text()` | `fs::read_to_string(path)` | Read file as UTF-8 string |
| `path.read_text(encoding)` | Custom decoder needed | Read with specific encoding |
| `path.write_text(content)` | `fs::write(path, content)` | Write string to file |
| `path.read_bytes()` | `fs::read(path)` | Read file as bytes |
| `path.write_bytes(data)` | `fs::write(path, data)` | Write bytes to file |

**Key Differences:**
- Python: Encoding defaults to UTF-8
- Rust: Always UTF-8 for `read_to_string()`, binary for `read()`
- Both overwrite existing files when writing
- Both methods handle file creation/opening automatically

## Directory Operations

### mkdir() and iterdir()

Create directories and iterate over contents:

```python
from pathlib import Path
import tempfile
import shutil

def test_directory_ops() -> int:
    # Create temporary directory
    tmp_dir = tempfile.mkdtemp()

    # Create subdirectory using pathlib
    base = Path(tmp_dir)
    subdir = base / "test_subdir"
    subdir.mkdir(exist_ok=True)

    # Create some files
    (subdir / "file1.txt").write_text("content1")
    (subdir / "file2.txt").write_text("content2")

    # Count files using iterdir()
    count = 0
    for item in subdir.iterdir():
        if item.is_file():
            count += 1

    # Cleanup
    shutil.rmtree(tmp_dir)

    return count
```

**Generated Rust:**

```rust
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn test_directory_ops() -> i32 {
    // Create temporary directory
    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = tmp_dir.path();

    // Create subdirectory using fs
    let mut subdir = PathBuf::from(base);
    subdir.push("test_subdir");
    fs::create_dir_all(&subdir).expect("Failed to create dir");

    // Create some files
    let mut file1 = subdir.clone();
    file1.push("file1.txt");
    fs::write(&file1, "content1").expect("Failed to write file1");

    let mut file2 = subdir.clone();
    file2.push("file2.txt");
    fs::write(&file2, "content2").expect("Failed to write file2");

    // Count files using read_dir()
    let mut count = 0;
    for entry in fs::read_dir(&subdir).expect("Failed to read dir") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                count += 1;
            }
        }
    }

    // Cleanup happens automatically with TempDir

    count
}
```

**Directory Methods:**

| Python Method | Rust Equivalent | Description |
|---------------|-----------------|-------------|
| `path.mkdir()` | `fs::create_dir(path)` | Create single directory |
| `path.mkdir(parents=True)` | `fs::create_dir_all(path)` | Create with parents |
| `path.mkdir(exist_ok=True)` | Check `exists()` first | Don't error if exists |
| `path.rmdir()` | `fs::remove_dir(path)` | Remove empty directory |
| `path.iterdir()` | `fs::read_dir(path)` | Iterate directory contents |

## Path Manipulation

### with_name() and with_suffix()

Modify paths by changing filename or extension:

```python
from pathlib import Path

def manipulate_paths() -> str:
    # Original path
    p = Path("/home/user/document.txt")

    # Change filename
    new_name = p.with_name("report.txt")

    # Change suffix
    new_suffix = p.with_suffix(".md")

    # Get string representations
    name_str = str(new_name)
    suffix_str = str(new_suffix)

    # Return concatenated result
    return name_str + "," + suffix_str
```

**Generated Rust:**

```rust
use std::path::{Path, PathBuf};

fn manipulate_paths() -> String {
    // Original path
    let p = Path::new("/home/user/document.txt");

    // Change filename
    let new_name = p.with_file_name("report.txt");

    // Change suffix
    let mut new_suffix = PathBuf::from(p);
    new_suffix.set_extension("md");

    // Get string representations
    let name_str = new_name.to_string_lossy().to_string();
    let suffix_str = new_suffix.to_string_lossy().to_string();

    // Return concatenated result
    format!("{},{}", name_str, suffix_str)
}
```

**Manipulation Methods:**

| Python Method | Rust Equivalent | Description |
|---------------|-----------------|-------------|
| `path.with_name(name)` | `path.with_file_name(name)` | Replace filename |
| `path.with_suffix(suffix)` | `path.set_extension(suffix)` | Replace extension |
| `path.with_stem(stem)` | Custom logic | Replace stem (Py 3.9+) |
| `path.resolve()` | `path.canonicalize()` | Resolve to absolute |
| `path.absolute()` | `path.absolutize()` | Make absolute |

## Common Use Cases

### 1. Find All Files with Extension

```python
from pathlib import Path

def find_python_files(directory: str) -> list:
    base = Path(directory)
    python_files = []

    for item in base.iterdir():
        if item.is_file() and item.suffix == '.py':
            python_files.append(str(item))

    return python_files
```

### 2. Create Nested Directory Structure

```python
from pathlib import Path

def create_project_structure(base_dir: str) -> None:
    base = Path(base_dir)

    # Create multiple directories
    (base / "src").mkdir(parents=True, exist_ok=True)
    (base / "tests").mkdir(exist_ok=True)
    (base / "docs").mkdir(exist_ok=True)

    # Create initial files
    (base / "README.md").write_text("# Project")
    (base / "src" / "__init__.py").write_text("")
```

### 3. Safe Path Joining

```python
from pathlib import Path

def build_config_path() -> Path:
    home = Path.home()  # Get user's home directory
    config = home / ".config" / "myapp" / "settings.toml"

    # Create parent directories if needed
    config.parent.mkdir(parents=True, exist_ok=True)

    return config
```

### 4. File Extension Operations

```python
from pathlib import Path

def process_file(file_path: str) -> dict:
    p = Path(file_path)

    return {
        'name': p.name,
        'stem': p.stem,
        'extension': p.suffix,
        'parent': str(p.parent),
        'is_python': p.suffix == '.py',
        'is_hidden': p.name.startswith('.')
    }
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| Path creation | O(1) | O(1) | No filesystem access |
| Property access | O(1) | O(1) | String operations only |
| `exists()` | O(1) | O(1) | Single stat call |
| `is_file()/is_dir()` | O(1) | O(1) | Single stat call |
| `iterdir()` | O(n) | O(n) | n = number of entries |
| `read_text()` | O(n) | O(n) | n = file size |
| `write_text()` | O(n) | O(n) | n = content size |
| `mkdir()` | O(1) | O(1) | Single syscall |
| `mkdir(parents=True)` | O(d) | O(d) | d = depth of tree |

**Performance Notes:**
- Path operations are zero-cost abstractions in Rust
- File I/O is buffered in both languages
- Directory iteration is lazy in Rust (iterator), eager in Python (list)
- Path manipulation creates new objects (immutable)

## Safety and Guarantees

**Type Safety:**
- `Path` (borrowed) vs `PathBuf` (owned) distinction in Rust
- All path operations preserve platform conventions
- UTF-8 validation with `.to_string_lossy()` fallback
- `Option` types for missing path components

**Error Handling:**
- Python: Raises `OSError` subclasses (`FileNotFoundError`, `PermissionError`)
- Rust: Returns `Result<T, io::Error>` with explicit error handling
- Both support `exist_ok` patterns for idempotent operations

**Important Notes:**
- TOCTOU races: check-then-use is racy (same as `os` module)
- Symbolic links: `exists()` follows links, may return `false` for broken links
- Path encoding: Non-UTF-8 paths require `OsStr` in Rust
- Cross-platform: `Path` handles platform differences automatically

**Best Practices:**

```rust
// ❌ BAD: TOCTOU race
if path.exists() {
    fs::write(path, content)?; // File might be deleted/created here
}

// ✅ GOOD: Atomic operation with proper error handling
match fs::write(path, content) {
    Ok(_) => println!("Written successfully"),
    Err(e) => eprintln!("Failed to write: {}", e),
}

// ✅ GOOD: Use OpenOptions for fine-grained control
use std::fs::OpenOptions;

OpenOptions::new()
    .write(true)
    .create_new(true)  // Fail if exists (prevents overwrite)
    .open(path)?;
```

## Path vs PathBuf

Understanding the difference between `Path` (borrowed) and `PathBuf` (owned):

| Feature | `Path` | `PathBuf` |
|---------|--------|-----------|
| Ownership | Borrowed reference | Owned value |
| Mutability | Immutable | Can be modified |
| Storage | Reference to existing path | Owns path data |
| Use case | Function parameters, temporary | Building paths, return values |
| Similar to | `&str` | `String` |

**Python:**
```python
from pathlib import Path

# Always creates new objects (immutable)
p1 = Path("/home")
p2 = p1 / "user"  # New Path object
```

**Rust:**
```rust
use std::path::{Path, PathBuf};

// Path (borrowed)
let p: &Path = Path::new("/home");

// PathBuf (owned)
let mut pb = PathBuf::from("/home");
pb.push("user");  // Modifies in place
```

## Platform Differences

### Path Separators

| Platform | Separator | Example |
|----------|-----------|---------|
| Unix/Linux | `/` | `/home/user/file.txt` |
| Windows | `\` | `C:\Users\user\file.txt` |
| Both | Handled automatically | Use `Path` methods |

### Special Paths

```python
from pathlib import Path

# Cross-platform home directory
home = Path.home()

# Current working directory
cwd = Path.cwd()

# Temporary directory (use tempfile module)
import tempfile
tmp = Path(tempfile.gettempdir())
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_pathlib.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_pathlib.py -v
```

**Expected Output:**
```
tests/test_pathlib.py::test_pathlib_properties PASSED        [ 16%]
tests/test_pathlib.py::test_pathlib_checks PASSED            [ 33%]
tests/test_pathlib.py::test_pathlib_file_io PASSED           [ 50%]
tests/test_pathlib.py::test_pathlib_directory_operations PASSED [ 66%]
tests/test_pathlib.py::test_pathlib_manipulation PASSED      [ 83%]
tests/test_pathlib.py::test_pathlib_path_construction PASSED [100%]

====== 6 passed in 0.XX s ======
```

## Performance Tips

**Optimization strategies:**
- Use `Path` for function parameters (borrowed, no allocation)
- Use `PathBuf` for building or returning paths (owned)
- Avoid repeated `to_string()` conversions
- Cache path checks when used multiple times
- Use `components()` iterator instead of `parts` list

**Example: Efficient Path Building**
```rust
use std::path::PathBuf;

fn build_path_efficiently(base: &str, parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(base);

    // extend() is more efficient than multiple push() calls
    path.extend(parts);

    path
}
```

## Comparison with os.path

| Feature | `os.path` | `pathlib.Path` |
|---------|-----------|----------------|
| Style | Functional | Object-oriented |
| Path joining | `os.path.join(a, b)` | `Path(a) / b` |
| Check existence | `os.path.exists(p)` | `Path(p).exists()` |
| Read file | `open(p).read()` | `Path(p).read_text()` |
| Basename | `os.path.basename(p)` | `Path(p).name` |
| Extension | `os.path.splitext(p)[1]` | `Path(p).suffix` |
| Parent | `os.path.dirname(p)` | `Path(p).parent` |

**Recommendation:** Use `pathlib.Path` for new code - it's more intuitive and cross-platform.

## Advanced Features

### Glob Patterns

```python
from pathlib import Path

def find_files(pattern: str) -> list:
    cwd = Path.cwd()

    # Find all Python files recursively
    py_files = list(cwd.glob('**/*.py'))

    # Find all files matching pattern
    matches = list(cwd.glob(pattern))

    return [str(f) for f in matches]
```

### Resolving Paths

```python
from pathlib import Path

def resolve_path(rel_path: str) -> str:
    p = Path(rel_path)

    # Resolve to absolute path, following symlinks
    absolute = p.resolve()

    # Check if path is absolute
    is_abs = p.is_absolute()

    return str(absolute)
```

## Security Considerations

**Path Traversal:**
```python
from pathlib import Path

def safe_join(base: Path, user_path: str) -> Path:
    # Prevent ../../etc/passwd attacks
    full_path = (base / user_path).resolve()

    # Ensure result is within base directory
    try:
        full_path.relative_to(base.resolve())
    except ValueError:
        raise ValueError("Path traversal detected")

    return full_path
```

**Symlink Attacks:**
- Always use `resolve()` before trusting paths
- Check `is_symlink()` when security matters
- Be careful with TOCTOU races
- Use atomic operations when possible

