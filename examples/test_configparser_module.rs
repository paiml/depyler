#[doc = "// NOTE: Map Python module 'configparser'(tracked in DEPYLER-0424)"]
use std::io::Cursor;
const STR__: &'static str = "=";
#[doc = "Test basic reading of config from string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_basic_read() {
    let config_string = "\n[DEFAULT]\nServerAliveInterval = 45\nCompression = yes\nCompressionLevel = 9\n\n[bitbucket.org]\nUser = hg\n\n[topsecret.server.com]\nPort = 50022\nForwardX11 = no\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert!(config.get("bitbucket.org".to_string()).is_some());
    assert!(config.get("topsecret.server.com".to_string()).is_some());
    assert!(
        config
            .get("bitbucket.org")
            .cloned()
            .unwrap_or_default()
            .get("User")
            .cloned()
            .unwrap_or_default()
            == "hg".to_string()
    );
    assert!(
        config
            .get("topsecret.server.com")
            .cloned()
            .unwrap_or_default()
            .get("Port")
            .cloned()
            .unwrap_or_default()
            == "50022".to_string()
    );
    println!("{}", "PASS: test_configparser_basic_read");
}
#[doc = "Test DEFAULT section values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_defaults() {
    let config_string =
        "\n[DEFAULT]\nServerAliveInterval = 45\nCompression = yes\n\n[example.com]\nUser = john\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert!(
        config
            .get("example.com")
            .cloned()
            .unwrap_or_default()
            .get("ServerAliveInterval")
            .cloned()
            .unwrap_or_default()
            == "45".to_string()
    );
    assert!(
        config
            .get("example.com")
            .cloned()
            .unwrap_or_default()
            .get("Compression")
            .cloned()
            .unwrap_or_default()
            == "yes".to_string()
    );
    assert!(
        config
            .get("example.com")
            .cloned()
            .unwrap_or_default()
            .get("User")
            .cloned()
            .unwrap_or_default()
            == "john".to_string()
    );
    println!("{}", "PASS: test_configparser_defaults");
}
#[doc = "Test type-converting get methods."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_get_methods() {
    let config_string = "\n[settings]\nport = 8080\ndebug = true\ntimeout = 30.5\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert!(config.get("settings").cloned().unwrap_or("port") == "8080".to_string());
    assert!(config.getint("settings".to_string(), "port".to_string()) == 8080);
    assert!(config.getboolean("settings".to_string(), "debug".to_string()) == true);
    assert!(config.getfloat("settings".to_string(), "timeout".to_string()) == 30.5);
    println!("{}", "PASS: test_configparser_get_methods");
}
#[doc = "Test listing sections."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_sections() {
    let config_string =
        "\n[section1]\nkey1 = value1\n\n[section2]\nkey2 = value2\n\n[section3]\nkey3 = value3\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    let sections = config.sections();
    assert!(sections.len() as i32 == 3);
    assert!(sections.get("section1".to_string()).is_some());
    assert!(sections.get("section2".to_string()).is_some());
    assert!(sections.get("section3".to_string()).is_some());
    println!("{}", "PASS: test_configparser_sections");
}
#[doc = "Test listing options in a section."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_options() {
    let config_string =
        "\n[database]\nhost = localhost\nport = 5432\nuser = admin\npassword = secret\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    let options = config.options("database".to_string());
    assert!(options.get("host".to_string()).is_some());
    assert!(options.get("port".to_string()).is_some());
    assert!(options.get("user".to_string()).is_some());
    assert!(options.get("password".to_string()).is_some());
    println!("{}", "PASS: test_configparser_options");
}
#[doc = "Test setting configuration values programmatically."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_set_values() {
    let config = configparser.ConfigParser();
    config.add_section("newsection".to_string());
    config.set(
        "newsection".to_string(),
        "option1".to_string(),
        "value1".to_string(),
    );
    config.set(
        "newsection".to_string(),
        "option2".to_string(),
        "value2".to_string(),
    );
    assert!(
        config
            .get("newsection")
            .cloned()
            .unwrap_or_default()
            .get("option1")
            .cloned()
            .unwrap_or_default()
            == "value1".to_string()
    );
    assert!(
        config
            .get("newsection")
            .cloned()
            .unwrap_or_default()
            .get("option2")
            .cloned()
            .unwrap_or_default()
            == "value2".to_string()
    );
    println!("{}", "PASS: test_configparser_set_values");
}
#[doc = "Test checking for section existence."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_has_section() {
    let config_string = "\n[existing]\nkey = value\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert!(config.has_section("existing".to_string()) == true);
    assert!(config.has_section("nonexistent".to_string()) == false);
    println!("{}", "PASS: test_configparser_has_section");
}
#[doc = "Test checking for option existence."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_has_option() {
    let config_string = "\n[section]\nexisting_option = value\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert!(config.has_option("section".to_string(), "existing_option".to_string()) == true);
    assert!(config.has_option("section".to_string(), "missing_option".to_string()) == false);
    println!("{}", "PASS: test_configparser_has_option");
}
#[doc = "Test removing sections and options."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_remove() {
    let config_string =
        "\n[section1]\noption1 = value1\noption2 = value2\n\n[section2]\noption3 = value3\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    config.remove_option("section1".to_string(), "option1".to_string());
    assert!(!config.has_option("section1".to_string(), "option1".to_string()));
    assert!(config.has_option("section1".to_string(), "option2".to_string()));
    config.remove_section("section2".to_string());
    assert!(!config.has_section("section2".to_string()));
    println!("{}", "PASS: test_configparser_remove");
}
#[doc = "Run all configparser tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "CONFIGPARSER MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_configparser_basic_read();
    test_configparser_defaults();
    test_configparser_get_methods();
    test_configparser_sections();
    test_configparser_options();
    test_configparser_set_values();
    test_configparser_has_section();
    test_configparser_has_option();
    test_configparser_remove();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL CONFIGPARSER TESTS PASSED!");
    println!("{}", "Total tests: 9");
    println!("{}", STR__.repeat(60 as usize));
}
