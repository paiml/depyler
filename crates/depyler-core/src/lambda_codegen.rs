use anyhow::Result;
use depyler_annotations::{LambdaAnnotations, LambdaEventType, Architecture};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Lambda-specific code generation for Rust output
#[derive(Debug, Clone)]
pub struct LambdaCodeGenerator {
    templates: HashMap<LambdaTemplate, String>,
    optimization_profile: OptimizationProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LambdaTemplate {
    BasicHandler,
    StreamingHandler,
    BatchProcessor,
    EventBridgeHandler,
    CargoToml,
    BuildScript,
    SamTemplate,
    CdkConstruct,
}

#[derive(Debug, Clone)]
pub struct OptimizationProfile {
    pub lto: bool,
    pub panic_abort: bool,
    pub codegen_units: u8,
    pub opt_level: String,
    pub strip: bool,
    pub mimalloc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaGenerationContext {
    pub event_type: Option<LambdaEventType>,
    pub response_type: String,
    pub handler_body: String,
    pub imports: Vec<String>,
    pub dependencies: Vec<String>,
    pub annotations: LambdaAnnotations,
    pub function_name: String,
    pub module_name: String,
}

impl Default for OptimizationProfile {
    fn default() -> Self {
        Self {
            lto: true,
            panic_abort: true,
            codegen_units: 1,
            opt_level: "z".to_string(),
            strip: true,
            mimalloc: true,
        }
    }
}

impl Default for LambdaCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaCodeGenerator {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Basic handler template
        templates.insert(LambdaTemplate::BasicHandler, BASIC_HANDLER_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::StreamingHandler, STREAMING_HANDLER_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::BatchProcessor, BATCH_PROCESSOR_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::EventBridgeHandler, EVENTBRIDGE_HANDLER_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::CargoToml, CARGO_TOML_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::BuildScript, BUILD_SCRIPT_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::SamTemplate, SAM_TEMPLATE.to_string());
        templates.insert(LambdaTemplate::CdkConstruct, CDK_CONSTRUCT_TEMPLATE.to_string());

        Self {
            templates,
            optimization_profile: OptimizationProfile::default(),
        }
    }

    pub fn with_optimization_profile(mut self, profile: OptimizationProfile) -> Self {
        self.optimization_profile = profile;
        self
    }

    /// Generate complete Lambda Rust project from Python handler
    pub fn generate_lambda_project(&self, context: &LambdaGenerationContext) -> Result<LambdaProject> {
        let handler_code = self.generate_handler(context)?;
        let cargo_toml = self.generate_cargo_toml(context)?;
        let build_script = self.generate_build_script(context)?;
        
        let mut project = LambdaProject {
            handler_code,
            cargo_toml,
            build_script,
            sam_template: None,
            cdk_construct: None,
            readme: self.generate_readme(context)?,
        };

        // Generate deployment templates if needed
        if !context.annotations.pre_warm_paths.is_empty() {
            project.sam_template = Some(self.generate_sam_template(context)?);
            project.cdk_construct = Some(self.generate_cdk_construct(context)?);
        }

        Ok(project)
    }

    /// Generate the main handler Rust code
    pub fn generate_handler(&self, context: &LambdaGenerationContext) -> Result<String> {
        let template = match &context.event_type {
            Some(LambdaEventType::SqsEvent) if context.annotations.batch_failure_reporting => {
                self.templates.get(&LambdaTemplate::BatchProcessor)
            }
            Some(LambdaEventType::EventBridgeEvent(_)) if context.annotations.custom_serialization => {
                self.templates.get(&LambdaTemplate::EventBridgeHandler)
            }
            _ => self.templates.get(&LambdaTemplate::BasicHandler)
        }.ok_or_else(|| anyhow::anyhow!("Template not found"))?;

        let mut code = template.clone();
        
        // Replace template variables
        code = code.replace("{{function_name}}", &context.function_name);
        code = code.replace("{{handler_body}}", &context.handler_body);
        code = code.replace("{{response_type}}", &context.response_type);
        
        // Handle event type specific replacements
        if let Some(event_type) = &context.event_type {
            let (event_module, event_rust_type) = self.get_event_type_mapping(event_type);
            code = code.replace("{{event_module}}", &event_module);
            code = code.replace("{{event_type}}", &event_rust_type);
        } else {
            code = code.replace("{{event_type}}", "serde_json::Value");
            code = code.replace("{{event_module}}", "");
        }

        // Handle tracing
        if context.annotations.tracing_enabled {
            code = code.replace("{{tracing_enabled}}", "true");
        } else {
            code = code.replace("{{tracing_enabled}}", "false");
        }

        // Add imports
        let imports_section = context.imports.join("\n");
        code = code.replace("{{imports}}", &imports_section);

        Ok(code)
    }

    /// Generate Cargo.toml for Lambda project
    pub fn generate_cargo_toml(&self, context: &LambdaGenerationContext) -> Result<String> {
        let template = self.templates.get(&LambdaTemplate::CargoToml)
            .ok_or_else(|| anyhow::anyhow!("Cargo.toml template not found"))?;

        let mut cargo_toml = template.clone();
        cargo_toml = cargo_toml.replace("{{package_name}}", &context.module_name);
        
        // Add event-specific dependencies
        let mut dependencies = context.dependencies.clone();
        
        // Core Lambda dependencies
        dependencies.push("lambda_runtime = \"0.8\"".to_string());
        dependencies.push("tokio = { version = \"1\", features = [\"macros\"] }".to_string());
        dependencies.push("serde = { version = \"1.0\", features = [\"derive\"] }".to_string());
        dependencies.push("serde_json = \"1.0\"".to_string());
        dependencies.push("anyhow = \"1.0\"".to_string());

        // Event-specific dependencies
        if context.event_type.is_some() {
            dependencies.push("aws-lambda-events = \"0.10\"".to_string());
        }

        if context.annotations.tracing_enabled {
            dependencies.push("tracing = \"0.1\"".to_string());
            dependencies.push("tracing-subscriber = \"0.3\"".to_string());
        }

        if self.optimization_profile.mimalloc {
            dependencies.push("mimalloc = { version = \"0.1\", default-features = false }".to_string());
        }

        let deps_section = dependencies.join("\n");
        cargo_toml = cargo_toml.replace("{{dependencies}}", &deps_section);

        // Add optimization profile
        let profile_section = self.generate_optimization_profile();
        cargo_toml = cargo_toml.replace("{{profile}}", &profile_section);

        // Add Lambda metadata
        let metadata_section = self.generate_lambda_metadata(context);
        cargo_toml = cargo_toml.replace("{{lambda_metadata}}", &metadata_section);

        Ok(cargo_toml)
    }

    /// Generate build script for cargo-lambda
    pub fn generate_build_script(&self, context: &LambdaGenerationContext) -> Result<String> {
        let template = self.templates.get(&LambdaTemplate::BuildScript)
            .ok_or_else(|| anyhow::anyhow!("Build script template not found"))?;

        let mut script = template.clone();
        
        let arch_flag = match context.annotations.architecture {
            Architecture::Arm64 => "--arm64",
            Architecture::X86_64 => "--x86-64",
        };
        script = script.replace("{{architecture}}", arch_flag);

        Ok(script)
    }

    /// Generate SAM template
    pub fn generate_sam_template(&self, context: &LambdaGenerationContext) -> Result<String> {
        let template = self.templates.get(&LambdaTemplate::SamTemplate)
            .ok_or_else(|| anyhow::anyhow!("SAM template not found"))?;

        let mut sam = template.clone();
        sam = sam.replace("{{function_name}}", &context.function_name);
        sam = sam.replace("{{memory_size}}", &context.annotations.memory_size.to_string());
        
        let timeout = context.annotations.timeout.unwrap_or(15);
        sam = sam.replace("{{timeout}}", &timeout.to_string());

        let arch = match context.annotations.architecture {
            Architecture::Arm64 => "arm64",
            Architecture::X86_64 => "x86_64",
        };
        sam = sam.replace("{{architecture}}", arch);

        Ok(sam)
    }

    /// Generate CDK construct
    pub fn generate_cdk_construct(&self, context: &LambdaGenerationContext) -> Result<String> {
        let template = self.templates.get(&LambdaTemplate::CdkConstruct)
            .ok_or_else(|| anyhow::anyhow!("CDK template not found"))?;

        let mut cdk = template.clone();
        cdk = cdk.replace("{{function_name}}", &context.function_name);
        cdk = cdk.replace("{{memory_size}}", &context.annotations.memory_size.to_string());
        
        let timeout = context.annotations.timeout.unwrap_or(15);
        cdk = cdk.replace("{{timeout}}", &timeout.to_string());

        Ok(cdk)
    }

    fn generate_readme(&self, context: &LambdaGenerationContext) -> Result<String> {
        Ok(format!(
            r#"# {} Lambda Function

Generated Rust Lambda function from Python source.

## Build

```bash
cargo lambda build --release
```

## Test

```bash
cargo lambda test
```

## Deploy

```bash
cargo lambda deploy
```

## Configuration

- Memory: {}MB
- Timeout: {}s
- Architecture: {:?}
- Event Type: {:?}
"#,
            context.function_name,
            context.annotations.memory_size,
            context.annotations.timeout.unwrap_or(15),
            context.annotations.architecture,
            context.event_type
        ))
    }

    fn get_event_type_mapping(&self, event_type: &LambdaEventType) -> (String, String) {
        match event_type {
            LambdaEventType::S3Event => ("s3".to_string(), "S3Event".to_string()),
            LambdaEventType::ApiGatewayProxyRequest => ("apigw".to_string(), "ApiGatewayProxyRequest".to_string()),
            LambdaEventType::ApiGatewayV2HttpRequest => ("apigw".to_string(), "ApiGatewayV2httpRequest".to_string()),
            LambdaEventType::SqsEvent => ("sqs".to_string(), "SqsEvent".to_string()),
            LambdaEventType::SnsEvent => ("sns".to_string(), "SnsEvent".to_string()),
            LambdaEventType::DynamodbEvent => ("dynamodb".to_string(), "DynamodbEvent".to_string()),
            LambdaEventType::EventBridgeEvent(custom_type) => {
                if let Some(custom) = custom_type {
                    ("eventbridge".to_string(), format!("EventBridgeEvent<{custom}>"))
                } else {
                    ("eventbridge".to_string(), "EventBridgeEvent<serde_json::Value>".to_string())
                }
            }
            LambdaEventType::CloudwatchEvent => ("cloudwatch_events".to_string(), "CloudWatchEvent".to_string()),
            LambdaEventType::KinesisEvent => ("kinesis".to_string(), "KinesisEvent".to_string()),
            LambdaEventType::Custom(name) => ("".to_string(), name.clone()),
            LambdaEventType::Auto => ("".to_string(), "serde_json::Value".to_string()),
        }
    }

    fn generate_optimization_profile(&self) -> String {
        format!(
            r#"
[profile.lambda]
inherits = "release"
opt-level = "{}"
lto = {}
codegen-units = {}
panic = "{}"
strip = {}
overflow-checks = false
incremental = false
"#,
            self.optimization_profile.opt_level,
            self.optimization_profile.lto,
            self.optimization_profile.codegen_units,
            if self.optimization_profile.panic_abort { "abort" } else { "unwind" },
            self.optimization_profile.strip
        )
    }

    fn generate_lambda_metadata(&self, context: &LambdaGenerationContext) -> String {
        let arch = match context.annotations.architecture {
            Architecture::Arm64 => "aarch64-unknown-linux-musl",
            Architecture::X86_64 => "x86_64-unknown-linux-musl",
        };

        format!(
            r#"
[package.metadata.lambda]
compile = "{}"
memory = {}
timeout = {}
architecture = "{}"
"#,
            arch,
            context.annotations.memory_size,
            context.annotations.timeout.unwrap_or(15),
            match context.annotations.architecture {
                Architecture::Arm64 => "arm64",
                Architecture::X86_64 => "x86_64",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct LambdaProject {
    pub handler_code: String,
    pub cargo_toml: String,
    pub build_script: String,
    pub sam_template: Option<String>,
    pub cdk_construct: Option<String>,
    pub readme: String,
}

// Template constants
const BASIC_HANDLER_TEMPLATE: &str = r#"{{imports}}
use lambda_runtime::{service_fn, LambdaEvent, Error};
{% if event_type %}
use aws_lambda_events::{{event_module}}::{{event_type}};
{% endif %}

{% if mimalloc %}
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;
{% endif %}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    {% if tracing_enabled %}
    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .init();
    {% endif %}
    
    {% if cold_start_optimize %}
    // Pre-warm critical paths
    let _ = serde_json::Value::Null;
    {% endif %}
    
    lambda_runtime::run(service_fn({{function_name}})).await
}

async fn {{function_name}}(
    event: LambdaEvent<{% if event_type %}{{event_type}}{% else %}serde_json::Value{% endif %}>
) -> Result<{{response_type}}, Error> {
    {{handler_body}}
}
"#;

const STREAMING_HANDLER_TEMPLATE: &str = r#"{{imports}}
use lambda_runtime::{service_fn, LambdaEvent, Error, StreamResponse};
use bytes::Bytes;
use futures::stream::Stream;

async fn {{function_name}}(
    event: LambdaEvent<{{event_type}}>
) -> Result<StreamResponse<impl Stream<Item = Result<Bytes, Error>>>, Error> {
    let stream = futures::stream::iter(vec![
        Ok(Bytes::from("data: ")),
        Ok(Bytes::from({{handler_body}})),
    ]);
    
    Ok(StreamResponse::new(stream))
}
"#;

const BATCH_PROCESSOR_TEMPLATE: &str = r#"{{imports}}
use aws_lambda_events::sqs::{SqsBatchResponse, SqsBatchItemFailure, SqsEvent};
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn({{function_name}})).await
}

async fn {{function_name}}(event: LambdaEvent<SqsEvent>) -> Result<SqsBatchResponse, Error> {
    let mut batch_item_failures = Vec::new();
    
    for record in event.payload.records {
        let message_id = record.message_id.clone().unwrap_or_default();
        
        match process_record(&record).await {
            Ok(_) => {},
            Err(_) => {
                batch_item_failures.push(SqsBatchItemFailure {
                    item_identifier: message_id,
                });
            }
        }
    }
    
    Ok(SqsBatchResponse {
        batch_item_failures,
    })
}

async fn process_record(record: &aws_lambda_events::sqs::SqsMessage) -> Result<(), Error> {
    {{handler_body}}
}
"#;

const EVENTBRIDGE_HANDLER_TEMPLATE: &str = r#"{{imports}}
use aws_lambda_events::eventbridge::EventBridgeEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn({{function_name}})).await
}

async fn {{function_name}}(
    event: LambdaEvent<EventBridgeEvent<serde_json::Value>>,
) -> Result<(), Error> {
    {{handler_body}}
}
"#;

const CARGO_TOML_TEMPLATE: &str = r#"[package]
name = "{{package_name}}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "bootstrap"
path = "src/main.rs"

[dependencies]
{{dependencies}}

{{profile}}

{{lambda_metadata}}
"#;

const BUILD_SCRIPT_TEMPLATE: &str = r#"#!/bin/bash
# Generated build script for cargo-lambda

set -e

# Set optimization flags
export RUSTFLAGS="-C link-arg=-s -C opt-level=z -C codegen-units=1"
export CARGO_PROFILE_RELEASE_LTO=true
export CARGO_PROFILE_RELEASE_PANIC="abort"

# Build with cargo-lambda
cargo lambda build \
    --release \
    {{architecture}} \
    --output-format zip \
    --lambda-dir ./target/lambda

# Additional optimization
if command -v upx > /dev/null; then
    echo "Compressing binary with UPX..."
    upx --best target/lambda/*/bootstrap
fi

# Generate deployment package
HANDLER_NAME=$(basename $(pwd))
cp target/lambda/${HANDLER_NAME}/bootstrap.zip ${HANDLER_NAME}.zip
echo "Lambda package: ${HANDLER_NAME}.zip ($(du -h ${HANDLER_NAME}.zip | cut -f1))"
"#;

const SAM_TEMPLATE: &str = r#"AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31

Globals:
  Function:
    Runtime: provided.al2
    Architectures:
      - {{architecture}}
    MemorySize: {{memory_size}}
    Timeout: {{timeout}}

Resources:
  {{function_name}}Function:
    Type: AWS::Serverless::Function
    Properties:
      CodeUri: target/lambda/{{function_name}}/
      Handler: bootstrap
      Environment:
        Variables:
          RUST_LOG: info
      Events:
        Api:
          Type: Api
          Properties:
            Path: /{proxy+}
            Method: ANY
"#;

const CDK_CONSTRUCT_TEMPLATE: &str = r#"import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as cdk from 'aws-cdk-lib';

export class {{function_name}}Lambda extends cdk.Construct {
  public readonly function: lambda.Function;

  constructor(scope: cdk.Construct, id: string) {
    super(scope, id);

    this.function = new lambda.Function(this, '{{function_name}}', {
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      handler: 'bootstrap',
      code: lambda.Code.fromAsset('target/lambda/{{function_name}}'),
      memorySize: {{memory_size}},
      timeout: cdk.Duration.seconds({{timeout}}),
      environment: {
        RUST_LOG: 'info',
      },
    });
  }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::LambdaAnnotations;

    fn create_test_context() -> LambdaGenerationContext {
        LambdaGenerationContext {
            event_type: Some(LambdaEventType::ApiGatewayProxyRequest),
            response_type: "ApiGatewayProxyResponse".to_string(),
            handler_body: "Ok(ApiGatewayProxyResponse::default())".to_string(),
            imports: vec!["use serde_json;".to_string()],
            dependencies: vec![],
            annotations: LambdaAnnotations::default(),
            function_name: "handler".to_string(),
            module_name: "my_lambda".to_string(),
        }
    }

    #[test]
    fn test_basic_handler_generation() {
        let generator = LambdaCodeGenerator::new();
        let context = create_test_context();
        
        let handler = generator.generate_handler(&context).unwrap();
        assert!(handler.contains("async fn handler"));
        assert!(handler.contains("ApiGatewayProxyRequest"));
        assert!(handler.contains("LambdaEvent"));
    }

    #[test]
    fn test_cargo_toml_generation() {
        let generator = LambdaCodeGenerator::new();
        let context = create_test_context();
        
        let cargo_toml = generator.generate_cargo_toml(&context).unwrap();
        assert!(cargo_toml.contains("lambda_runtime"));
        assert!(cargo_toml.contains("aws-lambda-events"));
        assert!(cargo_toml.contains("[profile.lambda]"));
    }

    #[test]
    fn test_sqs_batch_processor() {
        let generator = LambdaCodeGenerator::new();
        let mut context = create_test_context();
        context.event_type = Some(LambdaEventType::SqsEvent);
        context.annotations.batch_failure_reporting = true;
        
        let handler = generator.generate_handler(&context).unwrap();
        assert!(handler.contains("SqsBatchResponse"));
        assert!(handler.contains("batch_item_failures"));
    }

    #[test]
    fn test_eventbridge_handler() {
        let generator = LambdaCodeGenerator::new();
        let mut context = create_test_context();
        context.event_type = Some(LambdaEventType::EventBridgeEvent(Some("OrderEvent".to_string())));
        context.annotations.custom_serialization = true;
        
        let handler = generator.generate_handler(&context).unwrap();
        assert!(handler.contains("EventBridgeEvent"));
    }

    #[test]
    fn test_full_project_generation() {
        let generator = LambdaCodeGenerator::new();
        let context = create_test_context();
        
        let project = generator.generate_lambda_project(&context).unwrap();
        assert!(!project.handler_code.is_empty());
        assert!(!project.cargo_toml.is_empty());
        assert!(!project.build_script.is_empty());
        assert!(!project.readme.is_empty());
    }

    #[test]
    fn test_optimization_profile() {
        let profile = OptimizationProfile {
            opt_level: "s".to_string(),
            lto: false,
            ..OptimizationProfile::default()
        };
        
        let generator = LambdaCodeGenerator::new().with_optimization_profile(profile);
        let context = create_test_context();
        
        let cargo_toml = generator.generate_cargo_toml(&context).unwrap();
        assert!(cargo_toml.contains("opt-level = \"s\""));
        assert!(cargo_toml.contains("lto = false"));
    }
}