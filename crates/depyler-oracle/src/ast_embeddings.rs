//! AST Embeddings for Code2Vec-style code representation (Issue #210).
//!
//! Provides AST-to-vector embeddings for both Python HIR and Rust AST,
//! enabling semantic structural comparison beyond text-based features.
//!
//! ## Architecture (Phase 2 Upgrade)
//!
//! ```text
//! Python Source → rustpython-parser → AST → Path Contexts → Code2Vec Embedding
//!                                                                  │
//!                                                                  ▼
//!                                                           Combined Features
//!                                                                  ↑
//! Rust Source → syn::parse_file → AST → Path Contexts → Code2Vec Embedding
//! ```
//!
//! ## Phase 2 Features (GH-210)
//!
//! - Proper Python AST via `rustpython-parser` (not heuristic line parsing)
//! - Proper Rust AST via `syn` crate
//! - Enhanced path context extraction with AST node types
//! - Function signature, parameter, and body analysis

use aprender::primitives::Matrix;
use rustpython_parser::{parse, Mode};
use serde::{Deserialize, Serialize};
use syn::visit::Visit;

/// Configuration for AST embedding extraction
#[derive(Debug, Clone)]
pub struct AstEmbeddingConfig {
    /// Maximum path length in AST traversal (default: 8)
    pub max_path_length: usize,
    /// Maximum number of path contexts per function (default: 200)
    pub max_path_contexts: usize,
    /// Embedding dimension (default: 128)
    pub embedding_dim: usize,
    /// Whether to include terminal nodes (default: true)
    pub include_terminals: bool,
}

impl Default for AstEmbeddingConfig {
    fn default() -> Self {
        Self {
            max_path_length: 8,
            max_path_contexts: 200,
            embedding_dim: 128,
            include_terminals: true,
        }
    }
}

/// A path context in Code2Vec style: (start_terminal, path, end_terminal)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PathContext {
    /// Starting terminal node (e.g., variable name, literal)
    pub start_terminal: String,
    /// Path through AST nodes (e.g., "FunctionDef|arguments|Name")
    pub path: String,
    /// Ending terminal node
    pub end_terminal: String,
}

/// AST embedding for a code snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstEmbedding {
    /// The embedding vector
    pub vector: Vec<f32>,
    /// Number of path contexts extracted
    pub path_count: usize,
    /// Source code hash for caching
    pub source_hash: u64,
}

impl AstEmbedding {
    /// Create an empty embedding with the given dimension
    #[must_use]
    pub fn empty(dim: usize) -> Self {
        Self {
            vector: vec![0.0; dim],
            path_count: 0,
            source_hash: 0,
        }
    }

    /// Convert to a row matrix
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        Matrix::from_vec(1, self.vector.len(), self.vector.clone())
            .expect("Embedding dimensions are correct")
    }

    /// Compute cosine similarity between this embedding and another.
    ///
    /// Returns a value in [-1.0, 1.0] where:
    /// - 1.0 means identical direction (most similar)
    /// - 0.0 means orthogonal (unrelated)
    /// - -1.0 means opposite direction (least similar)
    ///
    /// Since embeddings are L2-normalized, this is simply the dot product.
    /// Returns 0.0 if either embedding is a zero vector.
    #[must_use]
    pub fn cosine_similarity(&self, other: &Self) -> f32 {
        if self.vector.len() != other.vector.len() {
            return 0.0;
        }

        // For normalized vectors, cosine similarity = dot product
        let dot_product: f32 = self
            .vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a * b)
            .sum();

        dot_product
    }

    /// Check if this embedding is similar to another above a threshold.
    ///
    /// Default threshold of 0.8 indicates high structural similarity.
    #[must_use]
    pub fn is_similar_to(&self, other: &Self, threshold: f32) -> bool {
        self.cosine_similarity(other) >= threshold
    }
}

// =============================================================================
// GH-210 Phase 2: Python AST Visitor using rustpython-parser
// =============================================================================

/// Visitor for extracting path contexts from Python AST
struct PythonPathVisitor {
    paths: Vec<PathContext>,
    max_path_length: usize,
    current_path: Vec<String>,
}

impl PythonPathVisitor {
    fn new(max_path_length: usize) -> Self {
        Self {
            paths: Vec::new(),
            max_path_length,
            current_path: Vec::new(),
        }
    }

    /// Visit a parsed Python module
    fn visit_module(&mut self, module: &rustpython_parser::ast::Mod) {
        use rustpython_parser::ast::*;

        match module {
            Mod::Module(ModModule { body, .. }) => {
                for stmt in body {
                    self.visit_stmt(stmt);
                }
            }
            Mod::Interactive(ModInteractive { body, .. }) => {
                for stmt in body {
                    self.visit_stmt(stmt);
                }
            }
            Mod::Expression(ModExpression { body, .. }) => {
                self.visit_expr(body);
            }
            Mod::FunctionType(_) => {}
        }
    }

    fn visit_stmt(&mut self, stmt: &rustpython_parser::ast::Stmt) {
        use rustpython_parser::ast::*;

        match stmt {
            Stmt::FunctionDef(StmtFunctionDef {
                name, args, body, ..
            }) => {
                self.current_path.push("FunctionDef".to_string());

                // Path: FunctionDef -> name
                self.paths.push(PathContext {
                    start_terminal: "FunctionDef".to_string(),
                    path: self.current_path.join("|"),
                    end_terminal: name.to_string(),
                });

                // Extract parameter names
                for arg in &args.args {
                    self.paths.push(PathContext {
                        start_terminal: name.to_string(),
                        path: "FunctionDef|arguments|arg".to_string(),
                        end_terminal: arg.def.arg.to_string(),
                    });
                }

                // Visit body statements
                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::AsyncFunctionDef(StmtAsyncFunctionDef {
                name, args, body, ..
            }) => {
                self.current_path.push("AsyncFunctionDef".to_string());

                self.paths.push(PathContext {
                    start_terminal: "AsyncFunctionDef".to_string(),
                    path: self.current_path.join("|"),
                    end_terminal: name.to_string(),
                });

                for arg in &args.args {
                    self.paths.push(PathContext {
                        start_terminal: name.to_string(),
                        path: "AsyncFunctionDef|arguments|arg".to_string(),
                        end_terminal: arg.def.arg.to_string(),
                    });
                }

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::ClassDef(StmtClassDef { name, body, .. }) => {
                self.current_path.push("ClassDef".to_string());

                self.paths.push(PathContext {
                    start_terminal: "ClassDef".to_string(),
                    path: self.current_path.join("|"),
                    end_terminal: name.to_string(),
                });

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::Assign(StmtAssign { targets, value, .. }) => {
                for target in targets {
                    if let Expr::Name(ExprName { id, .. }) = target {
                        let value_type = self.expr_type_name(value);
                        self.paths.push(PathContext {
                            start_terminal: id.to_string(),
                            path: "Assign".to_string(),
                            end_terminal: value_type,
                        });
                    }
                }
                self.visit_expr(value);
            }
            Stmt::AnnAssign(StmtAnnAssign {
                target,
                value,
                annotation,
                ..
            }) => {
                if let Expr::Name(ExprName { id, .. }) = target.as_ref() {
                    let ann_type = self.expr_type_name(annotation);
                    self.paths.push(PathContext {
                        start_terminal: id.to_string(),
                        path: "AnnAssign".to_string(),
                        end_terminal: ann_type,
                    });
                }
                if let Some(val) = value {
                    self.visit_expr(val);
                }
            }
            Stmt::Return(StmtReturn {
                value: Some(val), ..
            }) => {
                let val_type = self.expr_type_name(val);
                self.paths.push(PathContext {
                    start_terminal: "Return".to_string(),
                    path: "return".to_string(),
                    end_terminal: val_type,
                });
                self.visit_expr(val);
            }
            Stmt::Return(StmtReturn { value: None, .. }) => {}
            Stmt::For(StmtFor {
                target, iter, body, ..
            }) => {
                self.current_path.push("For".to_string());

                if let Expr::Name(ExprName { id, .. }) = target.as_ref() {
                    self.paths.push(PathContext {
                        start_terminal: id.to_string(),
                        path: "For|target".to_string(),
                        end_terminal: self.expr_type_name(iter),
                    });
                }

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::If(StmtIf {
                test, body, orelse, ..
            }) => {
                self.current_path.push("If".to_string());
                self.visit_expr(test);

                for stmt in body {
                    self.visit_stmt(stmt);
                }
                for stmt in orelse {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::While(StmtWhile { test, body, .. }) => {
                self.current_path.push("While".to_string());
                self.visit_expr(test);

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::With(StmtWith { body, .. }) => {
                self.current_path.push("With".to_string());

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_path.pop();
            }
            Stmt::Expr(StmtExpr { value, .. }) => {
                self.visit_expr(value);
            }
            _ => {}
        }
    }

    fn visit_expr(&mut self, expr: &rustpython_parser::ast::Expr) {
        use rustpython_parser::ast::*;

        // Limit path depth
        if self.current_path.len() >= self.max_path_length {
            return;
        }

        match expr {
            Expr::Call(ExprCall { func, args, .. }) => {
                let func_name = self.expr_type_name(func);
                self.paths.push(PathContext {
                    start_terminal: "Call".to_string(),
                    path: "call".to_string(),
                    end_terminal: func_name,
                });

                for arg in args {
                    self.visit_expr(arg);
                }
            }
            Expr::BinOp(ExprBinOp {
                left, op, right, ..
            }) => {
                let op_str = format!("{:?}", op);
                self.paths.push(PathContext {
                    start_terminal: self.expr_type_name(left),
                    path: format!("BinOp|{}", op_str),
                    end_terminal: self.expr_type_name(right),
                });
            }
            Expr::Compare(ExprCompare {
                left,
                ops,
                comparators,
                ..
            }) => {
                if !ops.is_empty() && !comparators.is_empty() {
                    let op_str = format!("{:?}", ops[0]);
                    self.paths.push(PathContext {
                        start_terminal: self.expr_type_name(left),
                        path: format!("Compare|{}", op_str),
                        end_terminal: self.expr_type_name(&comparators[0]),
                    });
                }
            }
            Expr::Attribute(ExprAttribute { value, attr, .. }) => {
                self.paths.push(PathContext {
                    start_terminal: self.expr_type_name(value),
                    path: "Attribute".to_string(),
                    end_terminal: attr.to_string(),
                });
            }
            Expr::Subscript(ExprSubscript { value, slice, .. }) => {
                self.paths.push(PathContext {
                    start_terminal: self.expr_type_name(value),
                    path: "Subscript".to_string(),
                    end_terminal: self.expr_type_name(slice),
                });
            }
            Expr::List(ExprList { elts, .. }) | Expr::Tuple(ExprTuple { elts, .. }) => {
                for elt in elts {
                    self.visit_expr(elt);
                }
            }
            Expr::Dict(ExprDict { keys, values, .. }) => {
                for (key, val) in keys.iter().zip(values.iter()) {
                    if let Some(k) = key {
                        self.paths.push(PathContext {
                            start_terminal: self.expr_type_name(k),
                            path: "Dict".to_string(),
                            end_terminal: self.expr_type_name(val),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    /// Get a type name for an expression (for terminal nodes)
    fn expr_type_name(&self, expr: &rustpython_parser::ast::Expr) -> String {
        use rustpython_parser::ast::*;

        match expr {
            Expr::Name(ExprName { id, .. }) => id.to_string(),
            Expr::Constant(ExprConstant { value, .. }) => match value {
                Constant::Int(_) => "int".to_string(),
                Constant::Float(_) => "float".to_string(),
                Constant::Str(_) => "str".to_string(),
                Constant::Bool(_) => "bool".to_string(),
                Constant::None => "None".to_string(),
                _ => "constant".to_string(),
            },
            Expr::List(_) => "list".to_string(),
            Expr::Dict(_) => "dict".to_string(),
            Expr::Tuple(_) => "tuple".to_string(),
            Expr::Set(_) => "set".to_string(),
            Expr::Call(ExprCall { func, .. }) => {
                format!("{}()", self.expr_type_name(func))
            }
            Expr::Attribute(ExprAttribute { attr, .. }) => attr.to_string(),
            _ => "expr".to_string(),
        }
    }
}

// =============================================================================
// GH-210 Phase 2: Rust AST Visitor using syn
// =============================================================================

/// Visitor for extracting path contexts from Rust AST
struct RustPathVisitor {
    paths: Vec<PathContext>,
    max_path_length: usize,
    current_path: Vec<String>,
}

impl RustPathVisitor {
    fn new(max_path_length: usize) -> Self {
        Self {
            paths: Vec::new(),
            max_path_length,
            current_path: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for RustPathVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.current_path.push("ItemFn".to_string());

        let func_name = node.sig.ident.to_string();
        self.paths.push(PathContext {
            start_terminal: "fn".to_string(),
            path: self.current_path.join("|"),
            end_terminal: func_name.clone(),
        });

        // Extract parameter names
        for input in &node.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = input {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = pat_ident.ident.to_string();
                    let type_str = quote::quote!(#pat_type.ty).to_string();
                    self.paths.push(PathContext {
                        start_terminal: func_name.clone(),
                        path: "fn|param".to_string(),
                        end_terminal: format!(
                            "{}:{}",
                            param_name,
                            type_str.chars().take(20).collect::<String>()
                        ),
                    });
                }
            }
        }

        // Extract return type
        if let syn::ReturnType::Type(_, ty) = &node.sig.output {
            let ret_type = quote::quote!(#ty).to_string();
            self.paths.push(PathContext {
                start_terminal: func_name.clone(),
                path: "fn|return".to_string(),
                end_terminal: ret_type.chars().take(30).collect(),
            });
        }

        syn::visit::visit_item_fn(self, node);
        self.current_path.pop();
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.current_path.push("ImplItemFn".to_string());

        let method_name = node.sig.ident.to_string();
        self.paths.push(PathContext {
            start_terminal: "impl_fn".to_string(),
            path: self.current_path.join("|"),
            end_terminal: method_name.clone(),
        });

        // Extract parameter names
        for input in &node.sig.inputs {
            match input {
                syn::FnArg::Receiver(_) => {
                    self.paths.push(PathContext {
                        start_terminal: method_name.clone(),
                        path: "impl_fn|self".to_string(),
                        end_terminal: "self".to_string(),
                    });
                }
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        self.paths.push(PathContext {
                            start_terminal: method_name.clone(),
                            path: "impl_fn|param".to_string(),
                            end_terminal: pat_ident.ident.to_string(),
                        });
                    }
                }
            }
        }

        syn::visit::visit_impl_item_fn(self, node);
        self.current_path.pop();
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.current_path.push("ItemStruct".to_string());

        let struct_name = node.ident.to_string();
        self.paths.push(PathContext {
            start_terminal: "struct".to_string(),
            path: self.current_path.join("|"),
            end_terminal: struct_name.clone(),
        });

        // Extract field names
        for field in &node.fields {
            if let Some(ident) = &field.ident {
                let field_name = ident.to_string();
                let type_str = quote::quote!(#field.ty).to_string();
                self.paths.push(PathContext {
                    start_terminal: struct_name.clone(),
                    path: "struct|field".to_string(),
                    end_terminal: format!(
                        "{}:{}",
                        field_name,
                        type_str.chars().take(20).collect::<String>()
                    ),
                });
            }
        }

        syn::visit::visit_item_struct(self, node);
        self.current_path.pop();
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        self.current_path.push("ItemImpl".to_string());

        let impl_type = quote::quote!(#node.self_ty).to_string();
        self.paths.push(PathContext {
            start_terminal: "impl".to_string(),
            path: self.current_path.join("|"),
            end_terminal: impl_type.chars().take(30).collect(),
        });

        syn::visit::visit_item_impl(self, node);
        self.current_path.pop();
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        // Limit path depth
        if self.current_path.len() >= self.max_path_length {
            return;
        }

        if let syn::Pat::Ident(pat_ident) = &node.pat {
            let var_name = pat_ident.ident.to_string();
            let mutability = if pat_ident.mutability.is_some() {
                "mut "
            } else {
                ""
            };

            self.paths.push(PathContext {
                start_terminal: "let".to_string(),
                path: "Local".to_string(),
                end_terminal: format!("{}{}", mutability, var_name),
            });
        }

        syn::visit::visit_local(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if self.current_path.len() >= self.max_path_length {
            return;
        }

        let func_str = quote::quote!(#node.func).to_string();
        self.paths.push(PathContext {
            start_terminal: "call".to_string(),
            path: "ExprCall".to_string(),
            end_terminal: func_str.chars().take(30).collect(),
        });

        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if self.current_path.len() >= self.max_path_length {
            return;
        }

        let method_name = node.method.to_string();
        self.paths.push(PathContext {
            start_terminal: "method_call".to_string(),
            path: "ExprMethodCall".to_string(),
            end_terminal: method_name,
        });

        syn::visit::visit_expr_method_call(self, node);
    }
}

/// AST Embedder for extracting Code2Vec-style embeddings
pub struct AstEmbedder {
    config: AstEmbeddingConfig,
}

impl AstEmbedder {
    /// Create a new AST embedder with the given configuration
    #[must_use]
    pub fn new(config: AstEmbeddingConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(AstEmbeddingConfig::default())
    }

    /// Extract embedding from Python source code
    #[must_use]
    pub fn embed_python(&self, source: &str) -> AstEmbedding {
        let path_contexts = self.extract_python_paths(source);
        self.paths_to_embedding(&path_contexts, source)
    }

    /// Extract embedding from Rust source code
    #[must_use]
    pub fn embed_rust(&self, source: &str) -> AstEmbedding {
        let path_contexts = self.extract_rust_paths(source);
        self.paths_to_embedding(&path_contexts, source)
    }

    /// Extract path contexts from Python source using rustpython-parser (GH-210 Phase 2)
    fn extract_python_paths(&self, source: &str) -> Vec<PathContext> {
        let mut paths = Vec::new();

        // Parse Python source using rustpython-parser
        let parsed = match parse(source, Mode::Module, "<embedded>") {
            Ok(ast) => ast,
            Err(_) => {
                // Fall back to heuristic extraction on parse failure
                return self.extract_python_paths_heuristic(source);
            }
        };

        // Extract paths from the AST using visitor pattern
        let mut visitor = PythonPathVisitor::new(self.config.max_path_length);
        visitor.visit_module(&parsed);
        paths.extend(visitor.paths);

        // Limit to max contexts
        paths.truncate(self.config.max_path_contexts);
        paths
    }

    /// Fallback heuristic Python path extraction (when parser fails)
    fn extract_python_paths_heuristic(&self, source: &str) -> Vec<PathContext> {
        let mut paths = Vec::new();

        for line in source.lines() {
            let line = line.trim();
            if line.starts_with("def ") {
                if let Some(name_end) = line.find('(') {
                    let func_name = line[4..name_end].trim();
                    paths.push(PathContext {
                        start_terminal: "FunctionDef".to_string(),
                        path: "def".to_string(),
                        end_terminal: func_name.to_string(),
                    });
                }
            }

            if line.contains('=') && !line.contains("==") {
                if let Some(eq_pos) = line.find('=') {
                    let lhs = line[..eq_pos].trim();
                    let rhs = line[eq_pos + 1..].trim();
                    if !lhs.is_empty() && !rhs.is_empty() {
                        paths.push(PathContext {
                            start_terminal: lhs.to_string(),
                            path: "Assign".to_string(),
                            end_terminal: rhs.chars().take(20).collect(),
                        });
                    }
                }
            }
        }

        paths.truncate(self.config.max_path_contexts);
        paths
    }

    /// Extract path contexts from Rust source using syn (GH-210 Phase 2)
    fn extract_rust_paths(&self, source: &str) -> Vec<PathContext> {
        let mut paths = Vec::new();

        // Parse Rust source using syn
        let parsed = match syn::parse_file(source) {
            Ok(file) => file,
            Err(_) => {
                // Fall back to heuristic extraction on parse failure
                return self.extract_rust_paths_heuristic(source);
            }
        };

        // Extract paths from the AST using visitor pattern
        let mut visitor = RustPathVisitor::new(self.config.max_path_length);
        visitor.visit_file(&parsed);
        paths.extend(visitor.paths);

        // Limit to max contexts
        paths.truncate(self.config.max_path_contexts);
        paths
    }

    /// Fallback heuristic Rust path extraction (when syn fails)
    fn extract_rust_paths_heuristic(&self, source: &str) -> Vec<PathContext> {
        let mut paths = Vec::new();

        for line in source.lines() {
            let line = line.trim();

            if line.starts_with("fn ") || line.starts_with("pub fn ") {
                let start = if line.starts_with("pub fn ") { 7 } else { 3 };
                if let Some(paren) = line.find('(') {
                    let func_name = line[start..paren].trim();
                    paths.push(PathContext {
                        start_terminal: "FnDef".to_string(),
                        path: "fn".to_string(),
                        end_terminal: func_name.to_string(),
                    });
                }
            }

            if line.starts_with("let ") {
                if let Some(eq_pos) = line.find('=') {
                    let binding = line[4..eq_pos].trim().trim_start_matches("mut ");
                    paths.push(PathContext {
                        start_terminal: "LetBinding".to_string(),
                        path: "let".to_string(),
                        end_terminal: binding.split(':').next().unwrap_or("").trim().to_string(),
                    });
                }
            }
        }

        paths.truncate(self.config.max_path_contexts);
        paths
    }

    /// Convert path contexts to embedding vector
    fn paths_to_embedding(&self, paths: &[PathContext], source: &str) -> AstEmbedding {
        let dim = self.config.embedding_dim;
        let mut embedding = vec![0.0f32; dim];

        // Simple bag-of-paths embedding using hash-based features
        for path in paths {
            let path_str = format!(
                "{}|{}|{}",
                path.start_terminal, path.path, path.end_terminal
            );
            let hash = self.hash_string(&path_str);

            // Distribute hash across embedding dimensions
            for i in 0..4 {
                let idx = ((hash >> (i * 16)) as usize) % dim;
                embedding[idx] += 1.0;
            }
        }

        // Normalize embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut embedding {
                *x /= norm;
            }
        }

        AstEmbedding {
            vector: embedding,
            path_count: paths.len(),
            source_hash: self.hash_string(source),
        }
    }

    /// Simple string hash for feature indexing
    fn hash_string(&self, s: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Get the embedding dimension
    #[must_use]
    pub fn embedding_dim(&self) -> usize {
        self.config.embedding_dim
    }
}

/// Combined feature extractor using AST embeddings + keyword features
pub struct CombinedEmbeddingExtractor {
    ast_embedder: AstEmbedder,
}

impl CombinedEmbeddingExtractor {
    /// Create a new combined extractor
    #[must_use]
    pub fn new(config: AstEmbeddingConfig) -> Self {
        Self {
            ast_embedder: AstEmbedder::new(config),
        }
    }

    /// Create with defaults
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(AstEmbeddingConfig::default())
    }

    /// Extract combined features from Python source and error message
    #[must_use]
    pub fn extract_features(
        &self,
        python_source: &str,
        rust_source: &str,
        error_message: &str,
    ) -> CombinedFeatures {
        let python_embedding = self.ast_embedder.embed_python(python_source);
        let rust_embedding = self.ast_embedder.embed_rust(rust_source);
        let keyword_features = crate::features::ErrorFeatures::from_error_message(error_message);

        CombinedFeatures {
            python_embedding,
            rust_embedding,
            keyword_features,
        }
    }

    /// Get total feature dimension
    #[must_use]
    pub fn total_dim(&self) -> usize {
        // Python embedding + Rust embedding + keyword features
        self.ast_embedder.embedding_dim() * 2 + crate::features::ErrorFeatures::DIM
    }
}

/// Combined features from AST embeddings and keyword extraction
#[derive(Debug, Clone)]
pub struct CombinedFeatures {
    /// Python AST embedding
    pub python_embedding: AstEmbedding,
    /// Rust AST embedding
    pub rust_embedding: AstEmbedding,
    /// Keyword-based features
    pub keyword_features: crate::features::ErrorFeatures,
}

impl CombinedFeatures {
    /// Convert to a feature vector
    #[must_use]
    pub fn to_vec(&self) -> Vec<f32> {
        let mut features = Vec::new();
        features.extend(&self.python_embedding.vector);
        features.extend(&self.rust_embedding.vector);
        features.extend(self.keyword_features.to_vec());
        features
    }

    /// Convert to a row matrix
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        let vec = self.to_vec();
        let dim = vec.len();
        Matrix::from_vec(1, dim, vec).expect("Feature dimensions are correct")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_embedding_config_default() {
        let config = AstEmbeddingConfig::default();
        assert_eq!(config.max_path_length, 8);
        assert_eq!(config.max_path_contexts, 200);
        assert_eq!(config.embedding_dim, 128);
        assert!(config.include_terminals);
    }

    #[test]
    fn test_embed_python_simple() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
def hello(name):
    message = "Hello, " + name
    return message
"#;
        let embedding = embedder.embed_python(source);

        assert_eq!(embedding.vector.len(), 128);
        assert!(embedding.path_count > 0, "Should extract path contexts");
        assert!(embedding.source_hash != 0, "Should have source hash");
    }

    #[test]
    fn test_embed_rust_simple() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
fn hello(name: &str) -> String {
    let message = format!("Hello, {}", name);
    message
}
"#;
        let embedding = embedder.embed_rust(source);

        assert_eq!(embedding.vector.len(), 128);
        assert!(embedding.path_count > 0, "Should extract path contexts");
    }

    #[test]
    fn test_embedding_normalization() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(): pass";
        let embedding = embedder.embed_python(source);

        // Check embedding is normalized (L2 norm ≈ 1.0)
        let norm: f32 = embedding.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (norm - 1.0).abs() < 0.01 || norm == 0.0,
            "Embedding should be normalized, got {}",
            norm
        );
    }

    #[test]
    fn test_similar_code_similar_embeddings() {
        let embedder = AstEmbedder::with_defaults();

        let source1 = r#"
def add(a, b):
    result = a + b
    return result
"#;
        let source2 = r#"
def add(x, y):
    sum = x + y
    return sum
"#;
        let source3 = r#"
class Foo:
    def __init__(self):
        self.data = []
"#;

        let emb1 = embedder.embed_python(source1);
        let emb2 = embedder.embed_python(source2);
        let emb3 = embedder.embed_python(source3);

        // Cosine similarity
        let sim_1_2: f32 = emb1
            .vector
            .iter()
            .zip(&emb2.vector)
            .map(|(a, b)| a * b)
            .sum();
        let sim_1_3: f32 = emb1
            .vector
            .iter()
            .zip(&emb3.vector)
            .map(|(a, b)| a * b)
            .sum();

        // Similar functions should have higher similarity than different structures
        assert!(
            sim_1_2 > sim_1_3,
            "Similar functions should have higher similarity: {} vs {}",
            sim_1_2,
            sim_1_3
        );
    }

    #[test]
    fn test_combined_feature_extraction() {
        let extractor = CombinedEmbeddingExtractor::with_defaults();

        let python = "def greet(name): return 'Hello ' + name";
        let rust = "fn greet(name: &str) -> String { format!(\"Hello {}\", name) }";
        let error = "error[E0308]: mismatched types";

        let features = extractor.extract_features(python, rust, error);

        // Check dimensions
        let vec = features.to_vec();
        assert_eq!(
            vec.len(),
            extractor.total_dim(),
            "Feature vector should have correct dimension"
        );

        // Check individual components
        assert_eq!(features.python_embedding.vector.len(), 128);
        assert_eq!(features.rust_embedding.vector.len(), 128);
    }

    #[test]
    fn test_empty_source_handling() {
        let embedder = AstEmbedder::with_defaults();

        let empty_embedding = embedder.embed_python("");
        assert_eq!(empty_embedding.vector.len(), 128);
        assert_eq!(empty_embedding.path_count, 0);
    }

    #[test]
    fn test_to_matrix_conversion() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(): pass";
        let embedding = embedder.embed_python(source);
        let matrix = embedding.to_matrix();

        assert_eq!(matrix.n_rows(), 1);
        assert_eq!(matrix.n_cols(), 128);
    }

    #[test]
    fn test_path_context_extraction_python() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
def calculate(x, y):
    result = x + y
    total = result * 2
    return total
"#;
        let embedding = embedder.embed_python(source);

        // Should extract function def + 2 assignments
        assert!(embedding.path_count >= 3, "Should extract multiple paths");
    }

    #[test]
    fn test_deterministic_embeddings() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(x): return x * 2";

        let emb1 = embedder.embed_python(source);
        let emb2 = embedder.embed_python(source);

        assert_eq!(
            emb1.vector, emb2.vector,
            "Same source should produce same embedding"
        );
        assert_eq!(emb1.source_hash, emb2.source_hash);
    }

    // ==========================================================================
    // GH-210 Phase 2: Proper AST Parsing Tests
    // ==========================================================================

    #[test]
    fn test_phase2_python_proper_ast_parsing() {
        // This tests that rustpython-parser is being used for proper AST extraction
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

class Calculator:
    def __init__(self, value: int = 0):
        self.value = value

    def add(self, x: int) -> int:
        self.value += x
        return self.value
"#;
        let embedding = embedder.embed_python(source);

        // Should extract many path contexts with proper AST parsing
        // Function defs, class def, method defs, parameters, returns, etc.
        assert!(
            embedding.path_count >= 8,
            "Proper AST should extract many paths: got {}",
            embedding.path_count
        );
    }

    #[test]
    fn test_phase2_rust_proper_ast_parsing() {
        // This tests that syn is being used for proper Rust AST extraction
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn add(&mut self, x: i32) -> i32 {
        self.value += x;
        self.value
    }
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
"#;
        let embedding = embedder.embed_rust(source);

        // Should extract struct, impl, methods, function, parameters, etc.
        assert!(
            embedding.path_count >= 8,
            "Proper AST should extract many paths: got {}",
            embedding.path_count
        );
    }

    #[test]
    fn test_phase2_python_async_function() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
async def fetch_data(url: str) -> dict:
    response = await client.get(url)
    return response.json()
"#;
        let embedding = embedder.embed_python(source);

        // Should extract async function def, parameters, etc.
        assert!(
            embedding.path_count >= 2,
            "Should extract async function paths: got {}",
            embedding.path_count
        );
    }

    #[test]
    fn test_phase2_rust_struct_fields() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
struct Point {
    x: f64,
    y: f64,
    label: String,
}
"#;
        let embedding = embedder.embed_rust(source);

        // Should extract struct + 3 fields
        assert!(
            embedding.path_count >= 4,
            "Should extract struct fields: got {}",
            embedding.path_count
        );
    }

    #[test]
    fn test_phase2_heuristic_fallback_on_parse_error() {
        let embedder = AstEmbedder::with_defaults();
        // Invalid Python syntax - should fall back to heuristic extraction
        let source = r#"
def broken(
    # This has syntax error
def another(): pass
"#;
        // Should not panic, uses heuristic fallback
        let embedding = embedder.embed_python(source);
        assert_eq!(embedding.vector.len(), 128);
    }

    #[test]
    fn test_phase2_python_class_extraction() {
        let embedder = AstEmbedder::with_defaults();
        let source = r#"
class DataProcessor:
    def __init__(self, name: str):
        self.name = name
        self.items = []

    def process(self, data: list) -> list:
        results = []
        for item in data:
            results.append(self.transform(item))
        return results

    def transform(self, item):
        return item.upper()
"#;
        let embedding = embedder.embed_python(source);

        // Class + 3 methods + parameters + assignments
        assert!(
            embedding.path_count >= 6,
            "Should extract class structure: got {}",
            embedding.path_count
        );
    }

    // ==========================================================================
    // Cosine Similarity Tests
    // ==========================================================================

    #[test]
    fn test_cosine_similarity_identical_embeddings() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(x): return x * 2";
        let emb = embedder.embed_python(source);

        let similarity = emb.cosine_similarity(&emb);

        // Identical embeddings should have similarity ~1.0
        assert!(
            (similarity - 1.0).abs() < 0.01,
            "Identical embeddings should have similarity ~1.0, got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_similar_code() {
        let embedder = AstEmbedder::with_defaults();

        let source1 = r#"
def add(a, b):
    result = a + b
    return result
"#;
        let source2 = r#"
def add(x, y):
    sum = x + y
    return sum
"#;

        let emb1 = embedder.embed_python(source1);
        let emb2 = embedder.embed_python(source2);

        let similarity = emb1.cosine_similarity(&emb2);

        // Similar functions should have positive similarity
        assert!(
            similarity > 0.1,
            "Similar functions should have similarity > 0.1, got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_different_code() {
        let embedder = AstEmbedder::with_defaults();

        let source1 = r#"
def simple_add(a, b):
    return a + b
"#;
        let source2 = r#"
class ComplexProcessor:
    def __init__(self):
        self.data = []
        self.cache = {}

    def process(self, items):
        for item in items:
            self.data.append(item)
        return self.data
"#;

        let emb1 = embedder.embed_python(source1);
        let emb2 = embedder.embed_python(source2);

        let similarity = emb1.cosine_similarity(&emb2);

        // Different structures should have lower similarity
        assert!(
            similarity < 0.8,
            "Different code structures should have similarity < 0.8, got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_empty_embeddings() {
        let emb1 = AstEmbedding::empty(128);
        let emb2 = AstEmbedding::empty(128);

        let similarity = emb1.cosine_similarity(&emb2);

        // Empty (zero) vectors have dot product 0
        assert!(
            similarity.abs() < 0.01,
            "Empty embeddings should have similarity ~0, got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_mismatched_dimensions() {
        let emb1 = AstEmbedding::empty(128);
        let emb2 = AstEmbedding::empty(64);

        let similarity = emb1.cosine_similarity(&emb2);

        // Mismatched dimensions should return 0
        assert_eq!(similarity, 0.0, "Mismatched dimensions should return 0");
    }

    #[test]
    fn test_cosine_similarity_range() {
        let embedder = AstEmbedder::with_defaults();

        let sources = [
            "def foo(): pass",
            "def bar(x): return x",
            "class Baz: pass",
            "x = 1 + 2",
        ];

        let embeddings: Vec<_> = sources.iter().map(|s| embedder.embed_python(s)).collect();

        // All similarities should be in valid range [-1, 1] (with epsilon for float precision)
        for (i, emb1) in embeddings.iter().enumerate() {
            for (j, emb2) in embeddings.iter().enumerate() {
                let sim = emb1.cosine_similarity(emb2);
                assert!(
                    (-1.001..=1.001).contains(&sim),
                    "Similarity at ({}, {}) out of range: {}",
                    i,
                    j,
                    sim
                );
            }
        }
    }

    #[test]
    fn test_is_similar_to_above_threshold() {
        let embedder = AstEmbedder::with_defaults();
        let source = "def foo(x): return x * 2";
        let emb = embedder.embed_python(source);

        // Identical embedding should be similar at any positive threshold
        assert!(
            emb.is_similar_to(&emb, 0.99),
            "Identical embeddings should be similar"
        );
    }

    #[test]
    fn test_is_similar_to_below_threshold() {
        let embedder = AstEmbedder::with_defaults();

        let source1 = "def simple(): pass";
        let source2 = r#"
class Complex:
    def __init__(self):
        self.data = []
    def method(self, x):
        return x
"#;

        let emb1 = embedder.embed_python(source1);
        let emb2 = embedder.embed_python(source2);

        // Different structures with high threshold should not be similar
        assert!(
            !emb1.is_similar_to(&emb2, 0.95),
            "Different structures should not be similar at 0.95 threshold"
        );
    }

    #[test]
    fn test_cosine_similarity_symmetry() {
        let embedder = AstEmbedder::with_defaults();

        let emb1 = embedder.embed_python("def foo(a): return a + 1");
        let emb2 = embedder.embed_python("class Bar: pass");

        let sim_1_2 = emb1.cosine_similarity(&emb2);
        let sim_2_1 = emb2.cosine_similarity(&emb1);

        assert!(
            (sim_1_2 - sim_2_1).abs() < 0.0001,
            "Cosine similarity should be symmetric: {} vs {}",
            sim_1_2,
            sim_2_1
        );
    }

    #[test]
    fn test_cosine_similarity_cross_language() {
        let embedder = AstEmbedder::with_defaults();

        let python_source = r#"
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;
        let rust_source = r#"
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
"#;

        let py_emb = embedder.embed_python(python_source);
        let rs_emb = embedder.embed_rust(rust_source);

        let similarity = py_emb.cosine_similarity(&rs_emb);

        // Cross-language similarity should be valid (may be lower due to AST diff)
        assert!(
            (-1.0..=1.0).contains(&similarity),
            "Cross-language similarity should be valid: {}",
            similarity
        );
    }
}
