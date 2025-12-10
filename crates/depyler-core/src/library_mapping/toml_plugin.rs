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
                method: toml_item.method.clone().unwrap_or_else(|| "new".to_string()),
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
}
