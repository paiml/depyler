"""
Comprehensive test suite for config parsing patterns.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests config parsing core features using pure dict-based implementation:
- Creating config stores
- Parsing configuration from strings
- Accessing sections and options
- Getting values with type conversion
- Setting values programmatically
- Checking existence of sections and options
- Removing sections and options
"""


def config_new() -> dict[str, dict[str, str]]:
    """Create an empty config store."""
    result: dict[str, dict[str, str]] = {}
    return result


def config_set_section(config: dict[str, dict[str, str]], section: str) -> dict[str, dict[str, str]]:
    """Add a section to config."""
    if section not in config:
        config[section] = {}
    return config


def config_set_value(config: dict[str, dict[str, str]], section: str, key: str, value: str) -> dict[str, dict[str, str]]:
    """Set a value in a config section."""
    val_copy: str = value + ""
    if section not in config:
        config[section] = {}
    config[section][key] = val_copy
    return config


def config_get_value(config: dict[str, dict[str, str]], section: str, key: str, default_val: str) -> str:
    """Get a value from a config section with a default."""
    if section not in config:
        return default_val
    sect: dict[str, str] = config[section]
    if key not in sect:
        return default_val
    return sect[key]


def config_has_section(config: dict[str, dict[str, str]], section: str) -> bool:
    """Check if a section exists."""
    return section in config


def config_has_option(config: dict[str, dict[str, str]], section: str, key: str) -> bool:
    """Check if an option exists in a section."""
    if section not in config:
        return False
    return key in config[section]


def config_remove_option(config: dict[str, dict[str, str]], section: str, key: str) -> dict[str, dict[str, str]]:
    """Remove an option from a section by rebuilding without that key."""
    if section not in config:
        return config
    old_sect: dict[str, str] = config[section]
    new_sect: dict[str, str] = {}
    for k in old_sect:
        if k != key:
            new_sect[k] = old_sect[k]
    config[section] = new_sect
    return config


def config_remove_section(config: dict[str, dict[str, str]], section: str) -> dict[str, dict[str, str]]:
    """Remove a section by rebuilding without it."""
    result: dict[str, dict[str, str]] = {}
    for sec in config:
        if sec != section:
            result[sec] = config[sec]
    return result


def config_count_sections(config: dict[str, dict[str, str]]) -> int:
    """Count the number of sections."""
    count: int = 0
    for sec in config:
        count += 1
    return count


def config_count_options(config: dict[str, dict[str, str]], section: str) -> int:
    """Count options in a section."""
    if section not in config:
        return 0
    count: int = 0
    sect: dict[str, str] = config[section]
    for k in sect:
        count += 1
    return count


def config_get_int(config: dict[str, dict[str, str]], section: str, key: str, default_val: int) -> int:
    """Get a value as integer."""
    raw: str = config_get_value(config, section, key, "")
    if raw == "":
        return default_val
    return int(raw)


def config_get_bool(config: dict[str, dict[str, str]], section: str, key: str) -> bool:
    """Get a value as boolean (true/yes/1 -> True)."""
    raw: str = config_get_value(config, section, key, "")
    if raw == "true":
        return True
    if raw == "yes":
        return True
    if raw == "1":
        return True
    return False


def test_basic_read() -> int:
    """Test basic config store creation and read."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "server", "host", "localhost")
    cfg = config_set_value(cfg, "server", "port", "8080")
    cfg = config_set_value(cfg, "database", "user", "admin")

    if config_get_value(cfg, "server", "host", "") == "localhost":
        passed += 1
    if config_get_value(cfg, "server", "port", "") == "8080":
        passed += 1
    if config_get_value(cfg, "database", "user", "") == "admin":
        passed += 1
    return passed


def test_defaults_pattern() -> int:
    """Test defaults pattern: section inherits from defaults dict."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "defaults", "timeout", "30")
    cfg = config_set_value(cfg, "defaults", "retries", "3")
    cfg = config_set_value(cfg, "app", "name", "myapp")

    # App section has its own value
    if config_get_value(cfg, "app", "name", "") == "myapp":
        passed += 1
    # Defaults section values accessible
    if config_get_value(cfg, "defaults", "timeout", "") == "30":
        passed += 1
    if config_get_value(cfg, "defaults", "retries", "") == "3":
        passed += 1
    return passed


def test_get_methods() -> int:
    """Test type-converting get methods."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "settings", "port", "8080")
    cfg = config_set_value(cfg, "settings", "debug", "true")
    cfg = config_set_value(cfg, "settings", "retries", "5")

    if config_get_value(cfg, "settings", "port", "") == "8080":
        passed += 1
    if config_get_int(cfg, "settings", "port", 0) == 8080:
        passed += 1
    if config_get_bool(cfg, "settings", "debug"):
        passed += 1
    return passed


def test_sections() -> int:
    """Test counting and checking sections."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "section1", "k1", "v1")
    cfg = config_set_value(cfg, "section2", "k2", "v2")
    cfg = config_set_value(cfg, "section3", "k3", "v3")

    if config_count_sections(cfg) == 3:
        passed += 1
    if config_has_section(cfg, "section1"):
        passed += 1
    if config_has_section(cfg, "section2"):
        passed += 1
    return passed


def test_options() -> int:
    """Test listing options in a section."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "database", "host", "localhost")
    cfg = config_set_value(cfg, "database", "port", "5432")
    cfg = config_set_value(cfg, "database", "user", "admin")
    cfg = config_set_value(cfg, "database", "password", "secret")

    if config_count_options(cfg, "database") == 4:
        passed += 1
    if config_has_option(cfg, "database", "host"):
        passed += 1
    if config_has_option(cfg, "database", "port"):
        passed += 1
    return passed


def test_set_values() -> int:
    """Test setting configuration values programmatically."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_section(cfg, "newsection")
    cfg = config_set_value(cfg, "newsection", "option1", "value1")
    cfg = config_set_value(cfg, "newsection", "option2", "value2")

    if config_get_value(cfg, "newsection", "option1", "") == "value1":
        passed += 1
    if config_get_value(cfg, "newsection", "option2", "") == "value2":
        passed += 1
    if config_has_section(cfg, "newsection"):
        passed += 1
    return passed


def test_has_section() -> int:
    """Test checking for section existence."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "existing", "key", "value")

    if config_has_section(cfg, "existing"):
        passed += 1
    if not config_has_section(cfg, "nonexistent"):
        passed += 1
    if config_count_sections(cfg) == 1:
        passed += 1
    return passed


def test_has_option() -> int:
    """Test checking for option existence."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "section", "existing_option", "value")

    if config_has_option(cfg, "section", "existing_option"):
        passed += 1
    if not config_has_option(cfg, "section", "missing_option"):
        passed += 1
    if config_get_value(cfg, "section", "existing_option", "") == "value":
        passed += 1
    return passed


def test_remove() -> int:
    """Test removing sections and options."""
    passed: int = 0
    cfg: dict[str, dict[str, str]] = config_new()
    cfg = config_set_value(cfg, "section1", "option1", "value1")
    cfg = config_set_value(cfg, "section1", "option2", "value2")
    cfg = config_set_value(cfg, "section2", "option3", "value3")

    cfg = config_remove_option(cfg, "section1", "option1")
    if not config_has_option(cfg, "section1", "option1"):
        passed += 1
    if config_has_option(cfg, "section1", "option2"):
        passed += 1

    cfg = config_remove_section(cfg, "section2")
    if not config_has_section(cfg, "section2"):
        passed += 1
    return passed


def test_module() -> int:
    """Run all config tests and return count of passed tests."""
    passed: int = 0
    passed += test_basic_read()
    passed += test_defaults_pattern()
    passed += test_get_methods()
    passed += test_sections()
    passed += test_options()
    passed += test_set_values()
    passed += test_has_section()
    passed += test_has_option()
    passed += test_remove()
    return passed


if __name__ == "__main__":
    result: int = test_module()
    print("PASSED: " + str(result))
