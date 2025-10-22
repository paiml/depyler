# os - File and Directory Operations

Python's os module provides functions for interacting with the operating system, including file operations, directory management, path manipulation, and environment variables. Depyler transpiles these to Rust's standard library (`std::fs`, `std::env`, `std::path`) with full type safety and error handling.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `import os` | `use std::fs::*; use std::env::*; use std::path::*` | Multiple Rust modules |
| `os.getcwd()` | `std::env::current_dir()` | Current working directory |
| `os.listdir(path)` | `std::fs::read_dir()` | Directory listing |
| `os.path.exists(path)` | `Path::exists()` | Path existence check |
| `os.path.isfile(path)` | `Path::is_file()` | File type check |
| `os.path.isdir(path)` | `Path::is_dir()` | Directory type check |
| `os.path.basename(path)` | `Path::file_name()` | Extract filename |
| `os.path.dirname(path)` | `Path::parent()` | Extract directory |
| `os.getenv(key)` | `std::env::var()` | Environment variables |

## Current Working Directory

### getcwd() - Get Current Directory

Get the current working directory as a string:

```python
import os

def get_current_directory() -> str:
    # Get current working directory
    cwd = os.getcwd()

    return cwd
```

**Generated Rust:**

```rust
use std::env;

fn get_current_directory() -> String {
    // Get current working directory
    let cwd = env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();

    cwd
}
```

**Key Differences:**
- Python returns `str` directly
- Rust returns `Result<PathBuf, io::Error>` (requires error handling)
- Use `.to_string_lossy()` for cross-platform string conversion
- `.expect()` provides panic on error (suitable for simple cases)

## Directory Listing

### listdir() - List Directory Contents

List all files and directories in a given path:

```python
import os

def list_current_directory() -> int:
    # List files in current directory
    cwd = os.getcwd()
    files = os.listdir(cwd)

    # Return count of files
    return len(files)
```

**Generated Rust:**

```rust
use std::env;
use std::fs;

fn list_current_directory() -> i32 {
    // List files in current directory
    let cwd = env::current_dir()
        .expect("Failed to get current directory");
    let entries = fs::read_dir(cwd)
        .expect("Failed to read directory");

    // Return count of files
    let count = entries.count() as i32;
    count
}
```

**Iteration Pattern:**

```python
import os

def list_directory_names() -> list:
    files = os.listdir('.')
    return files
```

**Generated Rust:**

```rust
use std::fs;

fn list_directory_names() -> Vec<String> {
    let entries = fs::read_dir(".")
        .expect("Failed to read directory");

    entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().map(String::from)
            })
        })
        .collect()
}
```

**Key Points:**
- `os.listdir()` returns `list[str]` (entry names only)
- `fs::read_dir()` returns iterator of `DirEntry` (full metadata)
- Rust requires explicit error handling for each entry
- Use `.filter_map()` to handle errors gracefully

## Path Operations

### Path Checking - exists(), isfile(), isdir()

Check if paths exist and determine their types:

```python
import os

def check_path_operations() -> bool:
    # Check if current directory exists
    cwd = os.getcwd()
    exists = os.path.exists(cwd)

    # Check if it's a directory
    is_dir = os.path.isdir(cwd)

    # Check if it's a file (should be False for directory)
    is_file = os.path.isfile(cwd)

    return exists and is_dir and not is_file
```

**Generated Rust:**

```rust
use std::env;
use std::path::Path;

fn check_path_operations() -> bool {
    // Check if current directory exists
    let cwd = env::current_dir()
        .expect("Failed to get current directory");
    let cwd_path = Path::new(&cwd);
    let exists = cwd_path.exists();

    // Check if it's a directory
    let is_dir = cwd_path.is_dir();

    // Check if it's a file (should be False for directory)
    let is_file = cwd_path.is_file();

    exists && is_dir && !is_file
}
```

**Comparison Table:**

| Python Function | Rust Method | Returns | Notes |
|----------------|-------------|---------|-------|
| `os.path.exists(path)` | `Path::exists()` | `bool` | True if path exists |
| `os.path.isfile(path)` | `Path::is_file()` | `bool` | True if regular file |
| `os.path.isdir(path)` | `Path::is_dir()` | `bool` | True if directory |
| `os.path.islink(path)` | `Path::is_symlink()` | `bool` | True if symbolic link |

**Key Behavior:**
- All methods return `false` for non-existent paths
- Symlinks: `exists()` follows links, `is_symlink()` detects links
- No exceptions thrown for inaccessible paths (returns `false`)
- Thread-safe and TOCTOU-aware (check-then-use races possible)

## Path Components

### basename() and dirname() - Extract Path Components

Extract filename and directory from paths:

```python
import os

def extract_path_components() -> tuple:
    # Create a sample path
    path = "/home/user/documents/file.txt"

    # Extract filename (basename)
    filename = os.path.basename(path)

    # Extract directory (dirname)
    directory = os.path.dirname(path)

    return (filename, directory)
```

**Generated Rust:**

```rust
use std::path::Path;

fn extract_path_components() -> (String, String) {
    // Create a sample path
    let path = Path::new("/home/user/documents/file.txt");

    // Extract filename (basename)
    let filename = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();

    // Extract directory (dirname)
    let directory = path.parent()
        .and_then(|parent| parent.to_str())
        .unwrap_or("")
        .to_string();

    (filename, directory)
}
```

**Component Extraction:**

| Operation | Python | Rust | Example Input | Output |
|-----------|--------|------|---------------|--------|
| Filename | `basename()` | `file_name()` | `/home/file.txt` | `file.txt` |
| Directory | `dirname()` | `parent()` | `/home/file.txt` | `/home` |
| Extension | `splitext()` | `extension()` | `file.txt` | `.txt` |
| Stem | N/A | `file_stem()` | `file.txt` | `file` |

**Edge Cases:**
- Root path (`/`): `basename()` returns `""`, `dirname()` returns `/`
- Relative path (`file.txt`): `dirname()` returns `""`
- Trailing slash (`/home/`): `basename()` returns `""`
- Empty path: Both return `""`

## Environment Variables

### getenv() - Read Environment Variables

Read environment variables with optional defaults:

```python
import os

def read_environment_variables() -> str:
    # Read environment variable with default
    home = os.getenv('HOME', '/default/home')

    # Read PATH variable
    path = os.getenv('PATH', '')

    return home
```

**Generated Rust:**

```rust
use std::env;

fn read_environment_variables() -> String {
    // Read environment variable with default
    let home = env::var("HOME")
        .unwrap_or_else(|_| String::from("/default/home"));

    // Read PATH variable
    let path = env::var("PATH")
        .unwrap_or_else(|_| String::from(""));

    home
}
```

**Environment Variable Operations:**

| Python Function | Rust Function | Returns | Notes |
|----------------|---------------|---------|-------|
| `os.getenv(key)` | `env::var(key)` | `Option<String>` / `Result` | Read variable |
| `os.getenv(key, default)` | `env::var(key).unwrap_or(default)` | `String` | With default |
| `os.environ[key]` | `env::var(key).unwrap()` | `String` | Panics if missing |
| `os.environ.get(key)` | `env::var(key).ok()` | `Option<String>` | Returns None |

**Key Differences:**
- Python `getenv()` returns `None` for missing variables (or default if provided)
- Rust `env::var()` returns `Result<String, VarError>`
- Use `.unwrap_or()` or `.unwrap_or_else()` for defaults
- Use `.ok()` to convert `Result` to `Option`

**Common Pattern - Reading Multiple Variables:**

```python
import os

def get_config() -> dict:
    config = {
        'home': os.getenv('HOME', '/home'),
        'user': os.getenv('USER', 'unknown'),
        'shell': os.getenv('SHELL', '/bin/sh')
    }
    return config
```

**Generated Rust:**

```rust
use std::env;
use std::collections::HashMap;

fn get_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert(
        "home".to_string(),
        env::var("HOME").unwrap_or_else(|_| String::from("/home"))
    );
    config.insert(
        "user".to_string(),
        env::var("USER").unwrap_or_else(|_| String::from("unknown"))
    );
    config.insert(
        "shell".to_string(),
        env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"))
    );
    config
}
```

## Complete Function Coverage

All common os functions are supported:

| Python Function | Rust Equivalent | Category |
|----------------|-----------------|----------|
| `os.getcwd()` | `env::current_dir()` | Directory Operations |
| `os.listdir(path)` | `fs::read_dir()` | Directory Operations |
| `os.path.exists(path)` | `Path::exists()` | Path Checks |
| `os.path.isfile(path)` | `Path::is_file()` | Path Checks |
| `os.path.isdir(path)` | `Path::is_dir()` | Path Checks |
| `os.path.basename(path)` | `Path::file_name()` | Path Components |
| `os.path.dirname(path)` | `Path::parent()` | Path Components |
| `os.getenv(key)` | `env::var()` | Environment |

## Common Use Cases

### 1. Check File Existence Before Reading

```python
import os

def safe_read_file(path: str) -> str:
    if os.path.exists(path) and os.path.isfile(path):
        with open(path, 'r') as f:
            return f.read()
    return ""
```

**Generated Rust:**

```rust
use std::fs;
use std::path::Path;

fn safe_read_file(path: &str) -> String {
    let path_obj = Path::new(path);
    if path_obj.exists() && path_obj.is_file() {
        fs::read_to_string(path).unwrap_or_default()
    } else {
        String::new()
    }
}
```

### 2. List All Files in Directory (Recursive)

```python
import os

def list_all_files(directory: str) -> list:
    files = []
    for entry in os.listdir(directory):
        full_path = os.path.join(directory, entry)
        if os.path.isfile(full_path):
            files.append(entry)
    return files
```

**Generated Rust:**

```rust
use std::fs;
use std::path::Path;

fn list_all_files(directory: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        files.push(name_str.to_string());
                    }
                }
            }
        }
    }

    files
}
```

### 3. Get Configuration from Environment

```python
import os

def get_database_config() -> dict:
    return {
        'host': os.getenv('DB_HOST', 'localhost'),
        'port': int(os.getenv('DB_PORT', '5432')),
        'user': os.getenv('DB_USER', 'postgres'),
        'password': os.getenv('DB_PASSWORD', '')
    }
```

**Generated Rust:**

```rust
use std::env;

struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
}

fn get_database_config() -> DatabaseConfig {
    DatabaseConfig {
        host: env::var("DB_HOST")
            .unwrap_or_else(|_| String::from("localhost")),
        port: env::var("DB_PORT")
            .unwrap_or_else(|_| String::from("5432"))
            .parse()
            .unwrap_or(5432),
        user: env::var("DB_USER")
            .unwrap_or_else(|_| String::from("postgres")),
        password: env::var("DB_PASSWORD")
            .unwrap_or_default(),
    }
}
```

### 4. Build Absolute Paths from Components

```python
import os

def get_config_file_path() -> str:
    home = os.getenv('HOME', '/home/user')
    config_dir = '.config'
    app_name = 'myapp'
    config_file = 'settings.toml'

    # Build path: ~/.config/myapp/settings.toml
    path = os.path.join(home, config_dir, app_name, config_file)
    return path
```

**Generated Rust:**

```rust
use std::env;
use std::path::PathBuf;

fn get_config_file_path() -> PathBuf {
    let home = env::var("HOME")
        .unwrap_or_else(|_| String::from("/home/user"));
    let config_dir = ".config";
    let app_name = "myapp";
    let config_file = "settings.toml";

    // Build path: ~/.config/myapp/settings.toml
    let mut path = PathBuf::from(home);
    path.push(config_dir);
    path.push(app_name);
    path.push(config_file);

    path
}
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `getcwd()` | O(1) | O(1) | System call |
| `listdir()` | O(n) | O(n) | n = number of entries |
| `path.exists()` | O(1) | O(1) | File system stat |
| `path.isfile()` | O(1) | O(1) | File system stat |
| `path.isdir()` | O(1) | O(1) | File system stat |
| `path.basename()` | O(1) | O(1) | String operation |
| `path.dirname()` | O(1) | O(1) | String operation |
| `getenv()` | O(1) | O(1) | Hash table lookup |

**Performance Notes:**
- File system operations are I/O bound
- Rust path operations are zero-cost abstractions
- `listdir()` results can be streamed (Rust) vs buffered (Python)
- Environment variable access is thread-safe in Rust
- Path string operations avoid allocations when possible

## Safety and Guarantees

**Type Safety:**
- All path operations preserve UTF-8 validity (`.to_string_lossy()` fallback)
- Environment variables return `Result` types (explicit error handling)
- Directory iterators handle entry read errors gracefully
- Path components return `Option` for missing components

**Error Handling:**
- Python: Exceptions for I/O errors (`OSError`, `FileNotFoundError`)
- Rust: `Result` types with explicit error handling
- Missing files: Python raises exceptions, Rust returns `Err` or `false`
- Permission denied: Both return errors (Python exception, Rust `io::Error`)

**Important Notes:**
- TOCTOU races: Check-then-use is inherently racy (use `fs::OpenOptions`)
- Symbolic links: `exists()` follows links, may return `false` for broken links
- Path encoding: Non-UTF-8 paths require special handling in Rust
- Cross-platform: Use `Path` methods, avoid string manipulation

**Best Practices:**
```rust
// ❌ BAD: TOCTOU race
if path.exists() {
    let file = fs::File::open(path)?; // File might be deleted here
}

// ✅ GOOD: Atomic operation
match fs::File::open(path) {
    Ok(file) => { /* use file */ },
    Err(e) => { /* handle error */ },
}
```

## Platform Differences

### Path Separators

| Platform | Separator | Example |
|----------|-----------|---------|
| Unix/Linux | `/` | `/home/user/file.txt` |
| Windows | `\` | `C:\Users\user\file.txt` |
| Both | `std::path::MAIN_SEPARATOR` | Cross-platform |

**Rust automatically handles platform separators via `Path` and `PathBuf`.**

### Environment Variables

| Variable | Unix/Linux | Windows | macOS |
|----------|-----------|---------|--------|
| Home directory | `HOME` | `USERPROFILE` | `HOME` |
| Path separator | `:` | `;` | `:` |
| Executable paths | `PATH` | `PATH` | `PATH` |
| Temp directory | `TMPDIR` | `TEMP` | `TMPDIR` |

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_os.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_os.py -v
```

**Expected Output:**
```
tests/test_os.py::test_os_getcwd PASSED                    [ 20%]
tests/test_os.py::test_os_listdir PASSED                   [ 40%]
tests/test_os.py::test_os_path_operations PASSED           [ 60%]
tests/test_os.py::test_os_path_components PASSED           [ 80%]
tests/test_os.py::test_os_getenv PASSED                    [100%]

====== 5 passed in 0.XX s ======
```

## Performance Tips

**Optimization strategies:**
- Cache `getcwd()` result if working directory doesn't change
- Use `read_dir()` iterator directly instead of collecting to `Vec`
- Pre-allocate `Vec` when collecting directory entries
- Use `OsStr` instead of `String` when UTF-8 conversion unnecessary
- Batch environment variable reads at startup

**Example: Efficient Directory Scanning**
```rust
use std::fs;
use std::path::Path;

fn count_files_efficiently(dir: &str) -> usize {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| e.path().is_file())
                .count()
        })
        .unwrap_or(0)
}
```

## Security Considerations

**Path Traversal:**
```python
import os

def safe_join(base: str, user_path: str) -> str:
    # Prevent ../../../etc/passwd attacks
    full_path = os.path.join(base, user_path)
    real_path = os.path.realpath(full_path)
    real_base = os.path.realpath(base)

    if not real_path.startswith(real_base):
        raise ValueError("Path traversal detected")

    return real_path
```

**Environment Variable Injection:**
- Never pass user input directly to `getenv()` keys
- Validate environment variable values before use
- Be careful with `PATH` manipulation
- Don't trust `LD_PRELOAD`, `LD_LIBRARY_PATH` in setuid programs

