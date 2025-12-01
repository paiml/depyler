# pathlib - Object-Oriented Filesystem Paths

Python's pathlib module provides an object-oriented approach to working with filesystem paths. It offers an intuitive API that's more readable than string-based path operations. Depyler transpiles these operations to Rust's `std::path::Path` and `PathBuf` with full type safety.

## Python → Rust Mapping

| Python Class/Method | Rust Equivalent | Notes |
|---------------------|-----------------|-------|
| `from pathlib import Path` | `use std::path::{Path, PathBuf}` | Path types |
| `Path("/path/to/file")` | `Path::new("/path/to/file")` | Immutable path |
| `p.name` | `p.file_name()` | Filename with extension |
| `p.stem` | `p.file_stem()` | Filename without extension |
| `p.suffix` | `p.extension()` | File extension |
| `p.parent` | `p.parent()` | Parent directory |
| `p.exists()` | `p.exists()` | Path existence check |
| `p.is_file()` | `p.is_file()` | File check |
| `p.is_dir()` | `p.is_dir()` | Directory check |
| `p / "sub"` | `p.join("sub")` | Path joining |
| `p.with_name("new")` | Custom method | Replace filename |
| `p.with_suffix(".ext")` | `p.with_extension("ext")` | Replace extension |

## Path Properties

### Extracting Path Components

Access various properties of a path without filesystem I/O:

```python
from pathlib import Path

def get_path_properties() -> str:
    # Create path object
    p = Path("/home/user/documents/file.txt")

    # Get various path properties
    name = str(p.name)        # "file.txt"
    stem = str(p.stem)        # "file"
    suffix = str(p.suffix)    # ".txt"
    parent = str(p.parent)    # "/home/user/documents"

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
        .unwrap_or("");
    let stem = p.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let suffix = p.extension()
        .map(|e| format!(".{}", e.to_str().unwrap_or("")))
        .unwrap_or_default();
    let parent = p.parent()
        .and_then(|p| p.to_str())
        .unwrap_or("");

    // Return concatenated result
    format!("{},{},{},{}", name, stem, suffix, parent)
}
```

**Path Properties:**
- `name`: Final component (filename)
- `stem`: Filename without extension
- `suffix`: File extension including dot
- `parent`: Parent directory path
- All operations are pure string manipulation (no I/O)

## Path Checking Operations

### Checking Path Type and Existence

Verify path existence and type with filesystem queries:

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

**Path Checking Methods:**
- `exists()`: Returns true if path exists (file, directory, or symlink)
- `is_file()`: Returns true only for regular files
- `is_dir()`: Returns true only for directories
- `is_symlink()`: Returns true for symbolic links (Python 3.12+)
- All methods perform actual filesystem I/O

## File I/O Operations

### write_text() and read_text() - Simple File Operations

Read and write text files with a single method call:

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
use std::path::Path;
use std::fs;

fn test_file_operations() -> String {
    // Create temporary file
    let tmp_path = "/tmp/test_file.txt";
    let p = Path::new(tmp_path);

    // Write text to file
    let content = "Hello, pathlib!";
    fs::write(p, content).expect("Failed to write file");

    // Read text back
    let read_content = fs::read_to_string(p)
        .expect("Failed to read file");

    // Cleanup
    fs::remove_file(p).ok();

    read_content
}
```

**File I/O Methods:**
- `write_text(content)`: Write string to file (creates/overwrites)
- `read_text()`: Read entire file as string
- `write_bytes(data)`: Write bytes to file
- `read_bytes()`: Read entire file as bytes
- Rust equivalent: `std::fs::write()` and `std::fs::read_to_string()`

## Directory Operations

### mkdir() and iterdir() - Directory Management

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
use std::path::Path;
use std::fs;

fn test_directory_ops() -> i32 {
    // Create temporary directory
    let tmp_dir = "/tmp/test_dir";
    let base = Path::new(tmp_dir);
    let subdir = base.join("test_subdir");

    // Create subdirectory
    fs::create_dir_all(&subdir).expect("Failed to create directory");

    // Create some files
    fs::write(subdir.join("file1.txt"), "content1")
        .expect("Failed to write file1");
    fs::write(subdir.join("file2.txt"), "content2")
        .expect("Failed to write file2");

    // Count files using read_dir()
    let mut count = 0;
    for entry in fs::read_dir(&subdir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                count += 1;
            }
        }
    }

    // Cleanup
    fs::remove_dir_all(tmp_dir).ok();

    count
}
```

**Directory Operations:**
- `mkdir(exist_ok=False)`: Create single directory
- `mkdir(parents=True)`: Create nested directories (like `mkdir -p`)
- `iterdir()`: Iterate over directory contents
- `rmdir()`: Remove empty directory
- Rust: `fs::create_dir()`, `fs::create_dir_all()`, `fs::read_dir()`

## Path Manipulation

### with_name() and with_suffix() - Path Transformation

Create new paths by modifying existing ones:

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

    // Change suffix (extension)
    let new_suffix = p.with_extension("md");

    // Get string representations
    let name_str = new_name.to_string_lossy().to_string();
    let suffix_str = new_suffix.to_string_lossy().to_string();

    // Return concatenated result
    format!("{},{}", name_str, suffix_str)
}
```

**Path Manipulation Methods:**
- `with_name(name)`: Replace filename, keep parent
- `with_suffix(suffix)`: Replace file extension
- `with_stem(stem)`: Replace filename without extension
- Returns new Path object (immutable operations)
- Rust: `with_file_name()`, `with_extension()`

## Path Construction

### The / Operator - Intuitive Path Building

Build paths using Python's division operator:

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
use std::path::{Path, PathBuf};

fn construct_paths() -> String {
    // Build path using join()
    let home = Path::new("/home");
    let mut user_dir = home.to_path_buf();
    user_dir.push("user");
    
    let mut docs = user_dir.clone();
    docs.push("documents");
    
    let mut file_path = docs.clone();
    file_path.push("file.txt");

    // Convert to string
    file_path.to_string_lossy().to_string()
}
```

**Path Construction:**
- Python: Use `/` operator for intuitive joining
- Handles path separators automatically (Unix `/`, Windows `\`)
- Rust: Use `join()` or `push()` methods
- `PathBuf` is mutable, `Path` is immutable

## Common Use Cases

### 1. Finding Files by Extension

Recursively find all files with specific extension:

```python
from pathlib import Path

def find_python_files(directory: str) -> list:
    """Find all Python files in directory tree."""
    path = Path(directory)
    python_files = []
    
    # Use glob to find .py files recursively
    for py_file in path.rglob("*.py"):
        python_files.append(str(py_file))
    
    return python_files
```

**Rust Equivalent:**

```rust
use std::path::Path;
use walkdir::WalkDir;

fn find_python_files(directory: &str) -> Vec<String> {
    let path = Path::new(directory);
    let mut python_files = Vec::new();
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("py") {
            python_files.push(entry.path().to_string_lossy().to_string());
        }
    }
    
    python_files
}
```

### 2. Building Configuration Paths

Create platform-appropriate config paths:

```python
from pathlib import Path
import os

def get_config_dir() -> Path:
    """Get application config directory."""
    if os.name == 'nt':  # Windows
        base = Path(os.getenv('APPDATA'))
    else:  # Unix/Linux/macOS
        base = Path.home() / '.config'
    
    config_dir = base / 'myapp'
    config_dir.mkdir(parents=True, exist_ok=True)
    
    return config_dir
```

### 3. Safe Path Joining

Safely join paths without worrying about separators:

```python
from pathlib import Path

def build_project_path(project_root: str, *parts) -> Path:
    """Build path within project safely."""
    root = Path(project_root)
    
    # Join all parts using / operator
    full_path = root
    for part in parts:
        full_path = full_path / part
    
    return full_path

# Usage:
# path = build_project_path("/home/user/project", "src", "lib", "utils.py")
# Result: /home/user/project/src/lib/utils.py
```

### 4. Temporary File Handling

Work with temporary files using pathlib:

```python
from pathlib import Path
import tempfile

def process_with_temp_file(data: str) -> str:
    """Process data using temporary file."""
    # Create temp directory
    with tempfile.TemporaryDirectory() as tmpdir:
        temp_path = Path(tmpdir) / "processing.txt"
        
        # Write data
        temp_path.write_text(data)
        
        # Process (example: convert to uppercase)
        content = temp_path.read_text()
        result = content.upper()
        
        # File automatically cleaned up
        return result
```

## Performance Characteristics

| Operation | Python pathlib | Rust std::path | Notes |
|-----------|----------------|----------------|-------|
| Property access | ~0.1 μs | ~0.05 μs | Pure string ops |
| Path construction | ~0.5 μs | ~0.2 μs | Allocation overhead |
| `exists()` | ~2 μs | ~1 μs | Filesystem syscall |
| `is_file()` | ~2 μs | ~1 μs | Filesystem syscall |
| `read_text()` (1KB) | ~50 μs | ~30 μs | I/O bound |
| `write_text()` (1KB) | ~60 μs | ~35 μs | I/O bound |

**Performance Notes:**
- Pure path operations (properties, joining) are fast in both
- Filesystem operations limited by OS, not language
- Rust benefits from zero-cost abstractions
- No GIL in Rust for concurrent file operations

**Rust Advantages:**
- Stack allocation for Path (no heap)
- Compile-time path validation (where possible)
- Better error handling with Result types
- Thread-safe by default

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_pathlib.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_pathlib.py -v
```

**Expected Output:**
```
tests/test_pathlib.py::test_pathlib_properties PASSED                    [ 16%]
tests/test_pathlib.py::test_pathlib_checks PASSED                        [ 33%]
tests/test_pathlib.py::test_pathlib_file_io PASSED                       [ 50%]
tests/test_pathlib.py::test_pathlib_directory_operations PASSED          [ 66%]
tests/test_pathlib.py::test_pathlib_manipulation PASSED                  [ 83%]
tests/test_pathlib.py::test_pathlib_path_construction PASSED             [100%]

====== 6 passed in 0.XX s ======
```

## Comparison: pathlib vs os.path

### Why Use pathlib?

**pathlib Advantages:**
```python
# pathlib: Object-oriented, chainable
from pathlib import Path
config = Path.home() / '.config' / 'app' / 'settings.json'

# os.path: String-based, verbose
import os
config = os.path.join(os.path.expanduser('~'), '.config', 'app', 'settings.json')
```

**Type Safety:**
```python
# pathlib: Type-safe Path objects
p = Path("/home/user/file.txt")
if p.exists():  # IDE knows p is a Path
    content = p.read_text()

# os.path: Just strings
p = "/home/user/file.txt"
if os.path.exists(p):  # p is just str
    with open(p) as f:
        content = f.read()
```

**Method Chaining:**
```python
# pathlib: Natural chaining
files = [f for f in Path('.').iterdir() if f.is_file() and f.suffix == '.py']

# os.path: Multiple steps
files = [f for f in os.listdir('.') 
         if os.path.isfile(f) and os.path.splitext(f)[1] == '.py']
```

### When to Use os.path

- Legacy codebases using os.path
- Simple string operations
- Python 2 compatibility (pathlib is Python 3.4+)
- Extremely performance-critical code (minimal difference)

## Platform Differences

### Path Separators

**Unix/Linux/macOS:**
- Separator: `/`
- Example: `/home/user/file.txt`
- Root: `/`

**Windows:**
- Separator: `\` (backslash)
- Example: `C:\Users\user\file.txt`
- Roots: `C:\`, `D:\`, etc.

**pathlib handles both automatically:**
```python
# Works on all platforms
p = Path("dir") / "subdir" / "file.txt"
# Unix: dir/subdir/file.txt
# Windows: dir\subdir\file.txt
```

### Home Directory

```python
# Cross-platform home directory
home = Path.home()
# Unix: /home/username
# Windows: C:\Users\username
# macOS: /Users/username
```

## Alternative Rust Patterns

### Using PathBuf for Mutable Paths

```rust
use std::path::PathBuf;

fn build_path_dynamically(base: &str, parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(base);
    
    for part in parts {
        path.push(part);
    }
    
    path
}

// Usage:
// let path = build_path_dynamically("/home", &["user", "docs", "file.txt"]);
```

### Error Handling with Result

```rust
use std::path::Path;
use std::fs;
use std::io;

fn safe_read_file(path: &Path) -> Result<String, io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist"
        ));
    }
    
    fs::read_to_string(path)
}

// Usage:
// match safe_read_file(Path::new("file.txt")) {
//     Ok(content) => println!("{}", content),
//     Err(e) => eprintln!("Error: {}", e),
// }
```

### Path Extension Trait

```rust
use std::path::{Path, PathBuf};

trait PathExt {
    fn ensure_dir(&self) -> std::io::Result<()>;
}

impl PathExt for Path {
    fn ensure_dir(&self) -> std::io::Result<()> {
        if !self.exists() {
            std::fs::create_dir_all(self)?;
        }
        Ok(())
    }
}

// Usage:
// Path::new("/tmp/mydir").ensure_dir()?;
```

## Future Support

**Currently Supported:**
- ✅ `Path()` - Path creation
- ✅ `p.name`, `p.stem`, `p.suffix`, `p.parent` - Properties
- ✅ `p.exists()`, `p.is_file()`, `p.is_dir()` - Checks
- ✅ `p.write_text()`, `p.read_text()` - File I/O
- ✅ `p.mkdir()`, `p.iterdir()` - Directory ops
- ✅ `p.with_name()`, `p.with_suffix()` - Manipulation
- ✅ `p / "sub"` - Path joining

**Planned Support:**
- 🔄 `p.glob("*.py")` - Pattern matching
- 🔄 `p.rglob("*.py")` - Recursive pattern matching
- 🔄 `p.resolve()` - Absolute path resolution
- 🔄 `p.relative_to(other)` - Relative paths
- 🔄 `p.rename(target)` - Rename/move
- 🔄 `p.unlink()` - Delete file
- 🔄 `p.rmdir()` - Remove directory
- 🔄 `p.chmod(mode)` - Change permissions

**Workarounds for Unsupported Features:**

```rust
// glob() - Use glob crate
use glob::glob;
for entry in glob("*.py")? {
    println!("{:?}", entry?);
}

// resolve() - Use canonicalize
use std::fs;
let absolute = fs::canonicalize(path)?;

// rename() - Use std::fs::rename
fs::rename(old_path, new_path)?;

// unlink() - Use std::fs::remove_file
fs::remove_file(path)?;
```

## Best Practices

**DO:**
- ✅ Use pathlib for new Python code
- ✅ Use `/` operator for path joining
- ✅ Use Path.home() instead of os.path.expanduser()
- ✅ Check path.exists() before operations
- ✅ Use context managers for temporary files

**DON'T:**
- ❌ Mix pathlib and os.path unnecessarily
- ❌ Use string concatenation for paths
- ❌ Assume path separator (use Path)
- ❌ Forget to handle Path.read_text() exceptions
- ❌ Use absolute paths when relative paths work

**Type Hints:**
```python
from pathlib import Path
from typing import Union

# Accept both str and Path
def process_file(path: Union[str, Path]) -> None:
    p = Path(path)  # Convert to Path if str
    # ... work with p
```
