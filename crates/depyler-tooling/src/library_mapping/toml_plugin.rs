//! DEPYLER-0903: TOML Plugin Loader for Enterprise Library Mappings
//!
//! Loads library mappings from TOML configuration files.
//!
//! # File Format
//!
//! ```toml
//! [plugin]
//! id = "my-plugin"
//! version = "1.0.0"
//!
//! [[mappings]]
//! python_module = "my_lib"
//! rust_crate = "my_lib_rs"
//! python_version_req = ">=3.8"
//! rust_crate_version = "1.0"
//!
//! [mappings.items]
//! my_func = { rust_name = "my_func", pattern = "Direct" }
//! ```

use super::{
    ItemMapping, LibraryMapping, MappingConfidence, MappingPlugin, MappingRegistry, ParamType,
    TransformPattern, TypeTransform, ValidationError,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// TOML plugin configuration file structure
#[derive(Debug, Deserialize)]
pub struct TomlPluginConfig {
    /// Plugin metadata
    pub plugin: PluginMetadata,
    /// Library mappings
    #[serde(default)]
    pub mappings: Vec<TomlLibraryMapping>,
}

/// Plugin metadata from TOML
#[derive(Debug, Deserialize)]
pub struct PluginMetadata {
    /// Plugin identifier
    pub id: String,
    /// Plugin version
    pub version: String,
    /// Optional maintainer contact
    #[serde(default)]
    pub maintainer: Option<String>,
}

/// Library mapping from TOML
#[derive(Debug, Deserialize)]
pub struct TomlLibraryMapping {
    /// Python module path
    pub python_module: String,
    /// Rust crate path
    pub rust_crate: String,
    /// Python version requirement
    #[serde(default = "default_version_req")]
    pub python_version_req: String,
    /// Rust crate version
    #[serde(default = "default_version_req")]
    pub rust_crate_version: String,
    /// Item mappings
    #[serde(default)]
    pub items: HashMap<String, TomlItemMapping>,
    /// Cargo features
    #[serde(default)]
    pub features: Vec<String>,
    /// Confidence level
    #[serde(default)]
    pub confidence: TomlConfidence,
    /// Source documentation
    #[serde(default)]
    pub provenance: String,
}

fn default_version_req() -> String {
    "*".to_string()
}

/// Item mapping from TOML
#[derive(Debug, Deserialize)]
pub struct TomlItemMapping {
    /// Rust name
    pub rust_name: String,
    /// Transform pattern type
    #[serde(default)]
    pub pattern: TomlPattern,
    /// Extra args for MethodCall
    #[serde(default)]
    pub extra_args: Vec<String>,
    /// Method name for Constructor
    #[serde(default)]
    pub method: Option<String>,
    /// Indices for ReorderArgs
    #[serde(default)]
    pub indices: Vec<usize>,
    /// Pattern string for TypedTemplate
    #[serde(default)]
    pub pattern_str: Option<String>,
    /// Params for TypedTemplate
    #[serde(default)]
    pub params: Vec<String>,
    /// Param types for TypedTemplate
    #[serde(default)]
    pub param_types: Vec<String>,
    /// Type transform
    #[serde(default)]
    pub type_transform: Option<TomlTypeTransform>,
}

/// Transform pattern name from TOML
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TomlPattern {
    #[default]
    Direct,
    MethodCall,
    PropertyToMethod,
    Constructor,
    ReorderArgs,
    TypedTemplate,
    Template,
}

/// Type transform from TOML
#[derive(Debug, Deserialize)]
pub struct TomlTypeTransform {
    pub python_type: String,
    pub rust_type: String,
}

/// Confidence level from TOML
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TomlConfidence {
    Verified,
    Community,
    #[default]
    Experimental,
}

/// TOML-based plugin implementation
pub struct TomlPlugin {
    config: TomlPluginConfig,
}

impl TomlPlugin {
    /// Load plugin from TOML string
    pub fn parse(toml_content: &str) -> Result<Self, TomlParseError> {
        let config: TomlPluginConfig =
            toml::from_str(toml_content).map_err(|e| TomlParseError {
                message: e.to_string(),
                // toml 0.8+ includes line info in Display output
                line: None,
            })?;
        Ok(Self { config })
    }

    /// Load plugin from file path
    pub fn from_file(path: &Path) -> Result<Self, TomlParseError> {
        let content = std::fs::read_to_string(path).map_err(|e| TomlParseError {
            message: format!("Failed to read file: {}", e),
            line: None,
        })?;
        Self::parse(&content)
    }

    /// Convert TOML mapping to LibraryMapping
    fn convert_mapping(toml_mapping: &TomlLibraryMapping) -> LibraryMapping {
        let items = toml_mapping
            .items
            .iter()
            .map(|(name, item)| (name.clone(), Self::convert_item(item)))
            .collect();

        LibraryMapping {
            python_module: toml_mapping.python_module.clone(),
            rust_crate: toml_mapping.rust_crate.clone(),
            python_version_req: toml_mapping.python_version_req.clone(),
            rust_crate_version: toml_mapping.rust_crate_version.clone(),
            items,
            features: toml_mapping.features.clone(),
            confidence: Self::convert_confidence(&toml_mapping.confidence),
            provenance: toml_mapping.provenance.clone(),
        }
    }

    /// Convert TOML item mapping
    fn convert_item(toml_item: &TomlItemMapping) -> ItemMapping {
        let pattern = match &toml_item.pattern {
            TomlPattern::Direct => TransformPattern::Direct,
            TomlPattern::MethodCall => TransformPattern::MethodCall {
                extra_args: toml_item.extra_args.clone(),
            },
            TomlPattern::PropertyToMethod => TransformPattern::PropertyToMethod,
            TomlPattern::Constructor => TransformPattern::Constructor {
                method: toml_item
                    .method
                    .clone()
                    .unwrap_or_else(|| "new".to_string()),
            },
            TomlPattern::ReorderArgs => TransformPattern::ReorderArgs {
                indices: toml_item.indices.clone(),
            },
            TomlPattern::TypedTemplate => TransformPattern::TypedTemplate {
                pattern: toml_item.pattern_str.clone().unwrap_or_default(),
                params: toml_item.params.clone(),
                param_types: toml_item
                    .param_types
                    .iter()
                    .map(|s| Self::parse_param_type(s))
                    .collect(),
            },
            #[allow(deprecated)]
            TomlPattern::Template => TransformPattern::Template {
                template: toml_item.pattern_str.clone().unwrap_or_default(),
            },
        };

        let type_transform = toml_item.type_transform.as_ref().map(|tt| TypeTransform {
            python_type: tt.python_type.clone(),
            rust_type: tt.rust_type.clone(),
        });

        ItemMapping {
            rust_name: toml_item.rust_name.clone(),
            pattern,
            type_transform,
        }
    }

    /// Parse ParamType from string
    fn parse_param_type(s: &str) -> ParamType {
        match s.to_lowercase().as_str() {
            "expr" => ParamType::Expr,
            "string" => ParamType::String,
            "number" => ParamType::Number,
            "bytes" => ParamType::Bytes,
            "bool" => ParamType::Bool,
            "path" => ParamType::Path,
            "list" => ParamType::List,
            "dict" => ParamType::Dict,
            _ => ParamType::Expr, // Default to Expr
        }
    }

    /// Convert confidence level
    fn convert_confidence(conf: &TomlConfidence) -> MappingConfidence {
        match conf {
            TomlConfidence::Verified => MappingConfidence::Verified,
            TomlConfidence::Community => MappingConfidence::Community,
            TomlConfidence::Experimental => MappingConfidence::Experimental,
        }
    }
}

impl MappingPlugin for TomlPlugin {
    fn id(&self) -> &str {
        &self.config.plugin.id
    }

    fn version(&self) -> &str {
        &self.config.plugin.version
    }

    fn register(&self, registry: &mut MappingRegistry) {
        for toml_mapping in &self.config.mappings {
            let mapping = Self::convert_mapping(toml_mapping);
            registry.register_extension(mapping);
        }
    }

    fn validate(&self) -> Result<(), ValidationError> {
        for mapping in &self.config.mappings {
            for (name, item) in &mapping.items {
                // Validate ReorderArgs
                if let TomlPattern::ReorderArgs = item.pattern {
                    TransformPattern::validate_reorder_args(&item.indices).map_err(|mut e| {
                        e.mapping = Some(format!("{}::{}", mapping.python_module, name));
                        e
                    })?;
                }

                // Validate TypedTemplate
                if let TomlPattern::TypedTemplate = item.pattern {
                    let pattern_str = item.pattern_str.as_deref().unwrap_or("");
                    let param_types: Vec<_> = item
                        .param_types
                        .iter()
                        .map(|s| Self::parse_param_type(s))
                        .collect();
                    TransformPattern::validate_typed_template(
                        pattern_str,
                        &item.params,
                        &param_types,
                    )
                    .map_err(|mut e| {
                        e.mapping = Some(format!("{}::{}", mapping.python_module, name));
                        e
                    })?;
                }
            }
        }
        Ok(())
    }
}

/// Error parsing TOML plugin
#[derive(Debug)]
pub struct TomlParseError {
    pub message: String,
    pub line: Option<usize>,
}

impl std::fmt::Display for TomlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            write!(f, "TOML parse error at line {}: {}", line, self.message)
        } else {
            write!(f, "TOML parse error: {}", self.message)
        }
    }
}

impl std::error::Error for TomlParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ parse_param_type tests ============

    #[test]
    fn test_parse_param_type_expr() {
        assert!(matches!(
            TomlPlugin::parse_param_type("expr"),
            ParamType::Expr
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("Expr"),
            ParamType::Expr
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("EXPR"),
            ParamType::Expr
        ));
    }

    #[test]
    fn test_parse_param_type_string() {
        assert!(matches!(
            TomlPlugin::parse_param_type("string"),
            ParamType::String
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("String"),
            ParamType::String
        ));
    }

    #[test]
    fn test_parse_param_type_number() {
        assert!(matches!(
            TomlPlugin::parse_param_type("number"),
            ParamType::Number
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("Number"),
            ParamType::Number
        ));
    }

    #[test]
    fn test_parse_param_type_bytes() {
        assert!(matches!(
            TomlPlugin::parse_param_type("bytes"),
            ParamType::Bytes
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("Bytes"),
            ParamType::Bytes
        ));
    }

    #[test]
    fn test_parse_param_type_bool() {
        assert!(matches!(
            TomlPlugin::parse_param_type("bool"),
            ParamType::Bool
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("Bool"),
            ParamType::Bool
        ));
    }

    #[test]
    fn test_parse_param_type_path() {
        assert!(matches!(
            TomlPlugin::parse_param_type("path"),
            ParamType::Path
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("Path"),
            ParamType::Path
        ));
    }

    #[test]
    fn test_parse_param_type_list() {
        assert!(matches!(
            TomlPlugin::parse_param_type("list"),
            ParamType::List
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("List"),
            ParamType::List
        ));
    }

    #[test]
    fn test_parse_param_type_dict() {
        assert!(matches!(
            TomlPlugin::parse_param_type("dict"),
            ParamType::Dict
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("Dict"),
            ParamType::Dict
        ));
    }

    #[test]
    fn test_parse_param_type_unknown_defaults_expr() {
        assert!(matches!(
            TomlPlugin::parse_param_type("unknown"),
            ParamType::Expr
        ));
        assert!(matches!(
            TomlPlugin::parse_param_type("custom"),
            ParamType::Expr
        ));
        assert!(matches!(TomlPlugin::parse_param_type(""), ParamType::Expr));
    }

    // ============ convert_confidence tests ============

    #[test]
    fn test_convert_confidence_verified() {
        let conf = TomlConfidence::Verified;
        assert!(matches!(
            TomlPlugin::convert_confidence(&conf),
            MappingConfidence::Verified
        ));
    }

    #[test]
    fn test_convert_confidence_community() {
        let conf = TomlConfidence::Community;
        assert!(matches!(
            TomlPlugin::convert_confidence(&conf),
            MappingConfidence::Community
        ));
    }

    #[test]
    fn test_convert_confidence_experimental() {
        let conf = TomlConfidence::Experimental;
        assert!(matches!(
            TomlPlugin::convert_confidence(&conf),
            MappingConfidence::Experimental
        ));
    }

    // ============ TomlParseError tests ============

    #[test]
    fn test_toml_parse_error_display_with_line() {
        let err = TomlParseError {
            message: "unexpected token".to_string(),
            line: Some(42),
        };
        let display = format!("{}", err);
        assert!(display.contains("line 42"));
        assert!(display.contains("unexpected token"));
    }

    #[test]
    fn test_toml_parse_error_display_without_line() {
        let err = TomlParseError {
            message: "invalid syntax".to_string(),
            line: None,
        };
        let display = format!("{}", err);
        assert!(!display.contains("line"));
        assert!(display.contains("invalid syntax"));
    }

    // ============ default_version_req tests ============

    #[test]
    fn test_default_version_req() {
        assert_eq!(default_version_req(), "*");
    }

    // ============ TomlPlugin parsing tests ============

    #[test]
    fn test_toml_plugin_basic_parsing() {
        let toml = r#"
[plugin]
id = "test-plugin"
version = "1.0.0"

[[mappings]]
python_module = "test_module"
rust_crate = "test_crate"
python_version_req = ">=3.8"
rust_crate_version = "1.0"

[mappings.items]
test_func = { rust_name = "test_func", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert_eq!(plugin.id(), "test-plugin");
        assert_eq!(plugin.version(), "1.0.0");
        assert_eq!(plugin.config.mappings.len(), 1);
        assert_eq!(plugin.config.mappings[0].python_module, "test_module");
    }

    #[test]
    fn test_toml_plugin_method_call() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "pandas"
rust_crate = "polars"

[mappings.items]
head = { rust_name = "head", pattern = "MethodCall", extra_args = ["None"] }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        let item = &plugin.config.mappings[0].items["head"];
        assert_eq!(item.extra_args, vec!["None"]);
    }

    #[test]
    fn test_toml_plugin_reorder_args() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "subprocess"
rust_crate = "std::process"

[mappings.items]
run = { rust_name = "run", pattern = "ReorderArgs", indices = [0, 2, 1] }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert!(plugin.validate().is_ok());
    }

    #[test]
    fn test_toml_plugin_invalid_reorder_args() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "bad"
rust_crate = "bad_crate"

[mappings.items]
bad_func = { rust_name = "bad_func", pattern = "ReorderArgs", indices = [0, 5, 1] }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        let result = plugin.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_toml_plugin_typed_template() {
        let toml = r#"
[plugin]
id = "aws"
version = "1.0.0"

[[mappings]]
python_module = "boto3.s3"
rust_crate = "aws_sdk_s3"

[mappings.items]
upload = { rust_name = "put_object", pattern = "TypedTemplate", pattern_str = "{client}.put_object({bucket}, {key})", params = ["client", "bucket", "key"], param_types = ["Expr", "String", "String"] }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert!(plugin.validate().is_ok());
    }

    #[test]
    fn test_toml_plugin_register() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "custom"
rust_crate = "custom_rs"

[mappings.items]
func = { rust_name = "func", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        let mut registry = MappingRegistry::new();
        plugin.register(&mut registry);

        let item = registry.lookup("custom", "func");
        assert!(item.is_some());
        assert_eq!(item.unwrap().rust_name, "func");
    }

    #[test]
    fn test_toml_plugin_constructor_pattern() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "pathlib"
rust_crate = "std::path"

[mappings.items]
Path = { rust_name = "PathBuf", pattern = "Constructor", method = "from" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        let item = &plugin.config.mappings[0].items["Path"];
        assert_eq!(item.method, Some("from".to_string()));
    }

    #[test]
    fn test_toml_plugin_property_to_method_pattern() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "os.path"
rust_crate = "std::path"

[mappings.items]
exists = { rust_name = "exists", pattern = "PropertyToMethod" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert!(plugin.validate().is_ok());
    }

    #[test]
    fn test_toml_plugin_with_features() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "crypto"
rust_crate = "ring"
features = ["std", "alloc"]

[mappings.items]
hash = { rust_name = "digest", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert_eq!(plugin.config.mappings[0].features, vec!["std", "alloc"]);
    }

    #[test]
    fn test_toml_plugin_with_confidence() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "verified_lib"
rust_crate = "verified_rs"
confidence = "Verified"
provenance = "Official API mapping"

[mappings.items]
func = { rust_name = "func", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert!(matches!(
            plugin.config.mappings[0].confidence,
            TomlConfidence::Verified
        ));
        assert_eq!(plugin.config.mappings[0].provenance, "Official API mapping");
    }

    #[test]
    fn test_toml_plugin_with_type_transform() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "numpy"
rust_crate = "ndarray"

[mappings.items]
array = { rust_name = "Array", pattern = "Constructor", type_transform = { python_type = "ndarray", rust_type = "Array<f64, Ix1>" } }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        let type_transform = &plugin.config.mappings[0].items["array"].type_transform;
        assert!(type_transform.is_some());
        let tt = type_transform.as_ref().unwrap();
        assert_eq!(tt.python_type, "ndarray");
        assert_eq!(tt.rust_type, "Array<f64, Ix1>");
    }

    #[test]
    fn test_toml_plugin_parse_error_invalid_toml() {
        let invalid_toml = r#"
[plugin
id = "broken"
"#;
        let result = TomlPlugin::parse(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_toml_plugin_parse_error_missing_required() {
        let missing_id = r#"
[plugin]
version = "1.0.0"
"#;
        let result = TomlPlugin::parse(missing_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_toml_plugin_default_version_req_used() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "test"
rust_crate = "test_rs"

[mappings.items]
func = { rust_name = "func", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        // Should use default "*" for version requirements
        assert_eq!(plugin.config.mappings[0].python_version_req, "*");
        assert_eq!(plugin.config.mappings[0].rust_crate_version, "*");
    }

    #[test]
    fn test_toml_plugin_multiple_mappings() {
        let toml = r#"
[plugin]
id = "test"
version = "1.0.0"

[[mappings]]
python_module = "module1"
rust_crate = "crate1"

[mappings.items]
func1 = { rust_name = "func1", pattern = "Direct" }

[[mappings]]
python_module = "module2"
rust_crate = "crate2"

[mappings.items]
func2 = { rust_name = "func2", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert_eq!(plugin.config.mappings.len(), 2);
        assert_eq!(plugin.config.mappings[0].python_module, "module1");
        assert_eq!(plugin.config.mappings[1].python_module, "module2");
    }

    #[test]
    fn test_toml_plugin_with_maintainer() {
        let toml = r#"
[plugin]
id = "enterprise-plugin"
version = "2.0.0"
maintainer = "team@example.com"

[[mappings]]
python_module = "enterprise"
rust_crate = "enterprise_rs"

[mappings.items]
func = { rust_name = "func", pattern = "Direct" }
"#;

        let plugin = TomlPlugin::parse(toml).unwrap();
        assert_eq!(
            plugin.config.plugin.maintainer,
            Some("team@example.com".to_string())
        );
    }
}
