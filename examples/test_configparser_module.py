"""
Comprehensive test suite for configparser module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests configparser core features:
- Creating ConfigParser instances
- Reading configuration from strings
- Accessing sections and options
- Getting values with type conversion
- Setting values programmatically
"""

import configparser
from io import StringIO


def test_configparser_basic_read():
    """Test basic reading of config from string."""
    config_string = """
[DEFAULT]
ServerAliveInterval = 45
Compression = yes
CompressionLevel = 9

[bitbucket.org]
User = hg

[topsecret.server.com]
Port = 50022
ForwardX11 = no
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    # Check section exists
    assert 'bitbucket.org' in config
    assert 'topsecret.server.com' in config

    # Check values
    assert config['bitbucket.org']['User'] == 'hg'
    assert config['topsecret.server.com']['Port'] == '50022'

    print("PASS: test_configparser_basic_read")


def test_configparser_defaults():
    """Test DEFAULT section values."""
    config_string = """
[DEFAULT]
ServerAliveInterval = 45
Compression = yes

[example.com]
User = john
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    # DEFAULT values should be available in all sections
    assert config['example.com']['ServerAliveInterval'] == '45'
    assert config['example.com']['Compression'] == 'yes'
    assert config['example.com']['User'] == 'john'

    print("PASS: test_configparser_defaults")


def test_configparser_get_methods():
    """Test type-converting get methods."""
    config_string = """
[settings]
port = 8080
debug = true
timeout = 30.5
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    # Get as string
    assert config.get('settings', 'port') == '8080'

    # Get as int
    assert config.getint('settings', 'port') == 8080

    # Get as boolean
    assert config.getboolean('settings', 'debug') == True

    # Get as float
    assert config.getfloat('settings', 'timeout') == 30.5

    print("PASS: test_configparser_get_methods")


def test_configparser_sections():
    """Test listing sections."""
    config_string = """
[section1]
key1 = value1

[section2]
key2 = value2

[section3]
key3 = value3
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    sections = config.sections()
    assert len(sections) == 3
    assert 'section1' in sections
    assert 'section2' in sections
    assert 'section3' in sections

    print("PASS: test_configparser_sections")


def test_configparser_options():
    """Test listing options in a section."""
    config_string = """
[database]
host = localhost
port = 5432
user = admin
password = secret
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    options = config.options('database')
    assert 'host' in options
    assert 'port' in options
    assert 'user' in options
    assert 'password' in options

    print("PASS: test_configparser_options")


def test_configparser_set_values():
    """Test setting configuration values programmatically."""
    config = configparser.ConfigParser()

    # Add new section
    config.add_section('newsection')

    # Set values
    config.set('newsection', 'option1', 'value1')
    config.set('newsection', 'option2', 'value2')

    # Verify
    assert config['newsection']['option1'] == 'value1'
    assert config['newsection']['option2'] == 'value2'

    print("PASS: test_configparser_set_values")


def test_configparser_has_section():
    """Test checking for section existence."""
    config_string = """
[existing]
key = value
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    assert config.has_section('existing') == True
    assert config.has_section('nonexistent') == False

    print("PASS: test_configparser_has_section")


def test_configparser_has_option():
    """Test checking for option existence."""
    config_string = """
[section]
existing_option = value
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    assert config.has_option('section', 'existing_option') == True
    assert config.has_option('section', 'missing_option') == False

    print("PASS: test_configparser_has_option")


def test_configparser_remove():
    """Test removing sections and options."""
    config_string = """
[section1]
option1 = value1
option2 = value2

[section2]
option3 = value3
"""
    config = configparser.ConfigParser()
    config.read_string(config_string)

    # Remove an option
    config.remove_option('section1', 'option1')
    assert not config.has_option('section1', 'option1')
    assert config.has_option('section1', 'option2')

    # Remove a section
    config.remove_section('section2')
    assert not config.has_section('section2')

    print("PASS: test_configparser_remove")


def main():
    """Run all configparser tests."""
    print("=" * 60)
    print("CONFIGPARSER MODULE TESTS")
    print("=" * 60)

    test_configparser_basic_read()
    test_configparser_defaults()
    test_configparser_get_methods()
    test_configparser_sections()
    test_configparser_options()
    test_configparser_set_values()
    test_configparser_has_section()
    test_configparser_has_option()
    test_configparser_remove()

    print("=" * 60)
    print("ALL CONFIGPARSER TESTS PASSED!")
    print("Total tests: 9")
    print("=" * 60)


if __name__ == "__main__":
    main()
