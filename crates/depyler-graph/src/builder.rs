//! Dependency Graph Builder
//!
//! Parses Python source into a directed dependency graph where:
//! - Nodes represent functions, classes, and modules
//! - Edges represent calls, imports, and inheritance

use crate::{GraphEdge, GraphError, GraphNode};
use petgraph::graph::{DiGraph, NodeIndex};
use rustpython_parser::{ast, Parse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Kind of node in the dependency graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// A function definition
    Function,
    /// A class definition
    Class,
    /// A method within a class
    Method,
    /// A module import
    Module,
    /// A global variable/constant
    Variable,
}

/// Kind of edge in the dependency graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    /// Function/method call
    Calls,
    /// Import dependency
    Imports,
    /// Class inheritance
    Inherits,
    /// Attribute access
    Accesses,
    /// Variable reference
    References,
}

/// The dependency graph structure
pub struct DependencyGraph {
    /// The underlying petgraph
    pub graph: DiGraph<GraphNode, GraphEdge>,
    /// Node lookup by ID
    pub node_map: HashMap<String, NodeIndex>,
    /// Source file path
    pub source_file: PathBuf,
    /// Source code for line calculation
    source: String,
}

impl DependencyGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            source_file: PathBuf::new(),
            source: String::new(),
        }
    }

    /// Get the number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) -> NodeIndex {
        let id = node.id.clone();
        let idx = self.graph.add_node(node);
        self.node_map.insert(id, idx);
        idx
    }

    /// Add an edge between nodes
    pub fn add_edge(&mut self, from: &str, to: &str, edge: GraphEdge) -> bool {
        if let (Some(&from_idx), Some(&to_idx)) = (self.node_map.get(from), self.node_map.get(to)) {
            self.graph.add_edge(from_idx, to_idx, edge);
            true
        } else {
            false
        }
    }

    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&GraphNode> {
        self.node_map.get(id).map(|&idx| &self.graph[idx])
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut GraphNode> {
        if let Some(&idx) = self.node_map.get(id) {
            Some(&mut self.graph[idx])
        } else {
            None
        }
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> Vec<String> {
        self.node_map.keys().cloned().collect()
    }

    /// Get incoming edges for a node
    pub fn incoming_edges(&self, id: &str) -> Vec<(&GraphNode, &GraphEdge)> {
        let Some(&idx) = self.node_map.get(id) else {
            return vec![];
        };

        self.graph
            .neighbors_directed(idx, petgraph::Direction::Incoming)
            .filter_map(|neighbor_idx| {
                let edge_idx = self.graph.find_edge(neighbor_idx, idx)?;
                Some((&self.graph[neighbor_idx], &self.graph[edge_idx]))
            })
            .collect()
    }

    /// Get outgoing edges for a node
    pub fn outgoing_edges(&self, id: &str) -> Vec<(&GraphNode, &GraphEdge)> {
        let Some(&idx) = self.node_map.get(id) else {
            return vec![];
        };

        self.graph
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .filter_map(|neighbor_idx| {
                let edge_idx = self.graph.find_edge(idx, neighbor_idx)?;
                Some((&self.graph[neighbor_idx], &self.graph[edge_idx]))
            })
            .collect()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing dependency graphs from Python source
pub struct GraphBuilder {
    /// Current class context (for method resolution)
    current_class: Option<String>,
    /// Known function names with their byte offsets
    known_functions: HashMap<String, usize>,
    /// Known class names with their byte offsets
    known_classes: HashMap<String, usize>,
    /// Source code for line calculation
    source: String,
}

impl GraphBuilder {
    /// Create a new graph builder
    pub fn new() -> Self {
        Self {
            current_class: None,
            known_functions: HashMap::new(),
            known_classes: HashMap::new(),
            source: String::new(),
        }
    }

    /// Convert byte offset to line number
    fn offset_to_line(&self, offset: usize) -> usize {
        let mut line = 1;
        for (i, ch) in self.source.chars().enumerate() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
            }
        }
        line
    }

    /// Convert byte offset to column number
    fn offset_to_column(&self, offset: usize) -> usize {
        let mut column = 1;
        for (i, ch) in self.source.chars().enumerate() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                column = 1;
            } else {
                column += 1;
            }
        }
        column
    }

    /// Build a dependency graph from Python source code
    pub fn build_from_source(&mut self, source: &str) -> Result<DependencyGraph, GraphError> {
        self.source = source.to_string();
        let mut graph = DependencyGraph::new();
        graph.source = source.to_string();

        // Parse Python source
        let ast = ast::Suite::parse(source, "<source>")
            .map_err(|e| GraphError::ParseError(e.to_string()))?;

        // First pass: collect all definitions
        self.collect_definitions(&ast);

        // Second pass: build nodes
        for stmt in &ast {
            self.process_statement(stmt, &mut graph)?;
        }

        // Third pass: build edges (calls, inheritance)
        for stmt in &ast {
            self.process_edges(stmt, &mut graph)?;
        }

        Ok(graph)
    }

    /// Collect all function and class definitions
    fn collect_definitions(&mut self, stmts: &[ast::Stmt]) {
        for stmt in stmts {
            match stmt {
                ast::Stmt::FunctionDef(f) => {
                    let offset: usize = u32::from(f.range.start()).try_into().unwrap_or(0);
                    self.known_functions.insert(f.name.to_string(), offset);
                }
                ast::Stmt::ClassDef(c) => {
                    let offset: usize = u32::from(c.range.start()).try_into().unwrap_or(0);
                    self.known_classes.insert(c.name.to_string(), offset);
                    // Collect methods
                    for stmt in &c.body {
                        if let ast::Stmt::FunctionDef(f) = stmt {
                            let method_name = format!("{}.{}", c.name, f.name);
                            let method_offset: usize =
                                u32::from(f.range.start()).try_into().unwrap_or(0);
                            self.known_functions.insert(method_name, method_offset);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Process a statement to build nodes
    fn process_statement(
        &mut self,
        stmt: &ast::Stmt,
        graph: &mut DependencyGraph,
    ) -> Result<(), GraphError> {
        match stmt {
            ast::Stmt::FunctionDef(f) => {
                let offset: usize = u32::from(f.range.start()).try_into().unwrap_or(0);
                let node = GraphNode {
                    id: f.name.to_string(),
                    kind: NodeKind::Function,
                    file: PathBuf::new(),
                    line: self.offset_to_line(offset),
                    column: self.offset_to_column(offset),
                    error_count: 0,
                    impact_score: 0.0,
                };
                graph.add_node(node);
            }
            ast::Stmt::ClassDef(c) => {
                let offset: usize = u32::from(c.range.start()).try_into().unwrap_or(0);
                let class_node = GraphNode {
                    id: c.name.to_string(),
                    kind: NodeKind::Class,
                    file: PathBuf::new(),
                    line: self.offset_to_line(offset),
                    column: self.offset_to_column(offset),
                    error_count: 0,
                    impact_score: 0.0,
                };
                graph.add_node(class_node);

                self.current_class = Some(c.name.to_string());

                // Process methods
                for body_stmt in &c.body {
                    if let ast::Stmt::FunctionDef(f) = body_stmt {
                        let method_offset: usize =
                            u32::from(f.range.start()).try_into().unwrap_or(0);
                        let method_id = format!("{}.{}", c.name, f.name);
                        let method_node = GraphNode {
                            id: method_id,
                            kind: NodeKind::Method,
                            file: PathBuf::new(),
                            line: self.offset_to_line(method_offset),
                            column: self.offset_to_column(method_offset),
                            error_count: 0,
                            impact_score: 0.0,
                        };
                        graph.add_node(method_node);
                    }
                }

                self.current_class = None;
            }
            ast::Stmt::Import(i) => {
                let offset: usize = u32::from(i.range.start()).try_into().unwrap_or(0);
                for alias in &i.names {
                    let node = GraphNode {
                        id: format!("import:{}", alias.name),
                        kind: NodeKind::Module,
                        file: PathBuf::new(),
                        line: self.offset_to_line(offset),
                        column: self.offset_to_column(offset),
                        error_count: 0,
                        impact_score: 0.0,
                    };
                    graph.add_node(node);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Process edges (calls, inheritance)
    fn process_edges(
        &mut self,
        stmt: &ast::Stmt,
        graph: &mut DependencyGraph,
    ) -> Result<(), GraphError> {
        match stmt {
            ast::Stmt::FunctionDef(f) => {
                let caller = f.name.to_string();
                self.extract_calls_from_body(&f.body, &caller, graph);
            }
            ast::Stmt::ClassDef(c) => {
                // Add inheritance edges
                for base in &c.bases {
                    if let ast::Expr::Name(name) = base {
                        let edge = GraphEdge {
                            kind: EdgeKind::Inherits,
                            weight: 1.0,
                        };
                        graph.add_edge(c.name.as_ref(), name.id.as_ref(), edge);
                    }
                }

                // Process method bodies
                for body_stmt in &c.body {
                    if let ast::Stmt::FunctionDef(f) = body_stmt {
                        let caller = format!("{}.{}", c.name, f.name);
                        self.extract_calls_from_body(&f.body, &caller, graph);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Extract call edges from a function body
    fn extract_calls_from_body(
        &self,
        body: &[ast::Stmt],
        caller: &str,
        graph: &mut DependencyGraph,
    ) {
        for stmt in body {
            self.extract_calls_from_stmt(stmt, caller, graph);
        }
    }

    /// Extract calls from a statement
    fn extract_calls_from_stmt(&self, stmt: &ast::Stmt, caller: &str, graph: &mut DependencyGraph) {
        match stmt {
            ast::Stmt::Expr(e) => {
                self.extract_calls_from_expr(&e.value, caller, graph);
            }
            ast::Stmt::Return(r) => {
                if let Some(value) = &r.value {
                    self.extract_calls_from_expr(value, caller, graph);
                }
            }
            ast::Stmt::Assign(a) => {
                self.extract_calls_from_expr(&a.value, caller, graph);
            }
            ast::Stmt::If(i) => {
                self.extract_calls_from_expr(&i.test, caller, graph);
                self.extract_calls_from_body(&i.body, caller, graph);
                self.extract_calls_from_body(&i.orelse, caller, graph);
            }
            ast::Stmt::For(f) => {
                self.extract_calls_from_expr(&f.iter, caller, graph);
                self.extract_calls_from_body(&f.body, caller, graph);
            }
            ast::Stmt::While(w) => {
                self.extract_calls_from_expr(&w.test, caller, graph);
                self.extract_calls_from_body(&w.body, caller, graph);
            }
            _ => {}
        }
    }

    /// Extract calls from an expression
    fn extract_calls_from_expr(&self, expr: &ast::Expr, caller: &str, graph: &mut DependencyGraph) {
        match expr {
            ast::Expr::Call(c) => {
                // Get the called function name
                let callee = match c.func.as_ref() {
                    ast::Expr::Name(n) => Some(n.id.to_string()),
                    ast::Expr::Attribute(a) => {
                        if let ast::Expr::Name(n) = a.value.as_ref() {
                            Some(format!("{}.{}", n.id, a.attr))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(callee_name) = callee {
                    // Only add edge if callee exists in graph
                    if self.known_functions.contains_key(&callee_name)
                        || self.known_classes.contains_key(&callee_name)
                    {
                        let edge = GraphEdge {
                            kind: EdgeKind::Calls,
                            weight: 1.0,
                        };
                        graph.add_edge(caller, &callee_name, edge);
                    }
                }

                // Process arguments recursively
                for arg in &c.args {
                    self.extract_calls_from_expr(arg, caller, graph);
                }
            }
            ast::Expr::BinOp(b) => {
                self.extract_calls_from_expr(&b.left, caller, graph);
                self.extract_calls_from_expr(&b.right, caller, graph);
            }
            ast::Expr::Compare(c) => {
                self.extract_calls_from_expr(&c.left, caller, graph);
                for comparator in &c.comparators {
                    self.extract_calls_from_expr(comparator, caller, graph);
                }
            }
            ast::Expr::List(l) => {
                for elt in &l.elts {
                    self.extract_calls_from_expr(elt, caller, graph);
                }
            }
            ast::Expr::Dict(d) => {
                for key in d.keys.iter().flatten() {
                    self.extract_calls_from_expr(key, caller, graph);
                }
                for value in &d.values {
                    self.extract_calls_from_expr(value, caller, graph);
                }
            }
            _ => {}
        }
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_function() {
        let python = r#"
def foo():
    return 42
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        assert_eq!(graph.node_count(), 1);
        assert!(graph.get_node("foo").is_some());
    }

    #[test]
    fn test_build_function_call() {
        let python = r#"
def foo():
    return 42

def bar():
    return foo()
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        // bar should have an edge to foo
        let edges = graph.outgoing_edges("bar");
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].0.id, "foo");
    }

    #[test]
    fn test_build_class_inheritance() {
        let python = r#"
class Base:
    def method(self):
        pass

class Derived(Base):
    def method(self):
        pass
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        // Should have 2 classes + 2 methods
        assert!(graph.node_count() >= 4);

        // Derived should inherit from Base
        let edges = graph.outgoing_edges("Derived");
        let inherits_base = edges
            .iter()
            .any(|(node, edge)| node.id == "Base" && edge.kind == EdgeKind::Inherits);
        assert!(inherits_base);
    }

    #[test]
    fn test_node_kind() {
        let python = r#"
def func():
    pass

class MyClass:
    def method(self):
        pass
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let func_node = graph.get_node("func").unwrap();
        assert_eq!(func_node.kind, NodeKind::Function);

        let class_node = graph.get_node("MyClass").unwrap();
        assert_eq!(class_node.kind, NodeKind::Class);

        let method_node = graph.get_node("MyClass.method").unwrap();
        assert_eq!(method_node.kind, NodeKind::Method);
    }

    #[test]
    fn test_incoming_outgoing_edges() {
        let python = r#"
def a():
    return 1

def b():
    return a()

def c():
    return a()
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        // a should have 2 incoming edges (from b and c)
        let incoming = graph.incoming_edges("a");
        assert_eq!(incoming.len(), 2);

        // b should have 1 outgoing edge (to a)
        let outgoing = graph.outgoing_edges("b");
        assert_eq!(outgoing.len(), 1);
    }

    #[test]
    fn test_line_numbers() {
        let python = r#"def foo():
    return 42

def bar():
    return 100
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let foo_node = graph.get_node("foo").unwrap();
        assert_eq!(foo_node.line, 1);

        let bar_node = graph.get_node("bar").unwrap();
        assert_eq!(bar_node.line, 4);
    }

    #[test]
    fn test_empty_source() {
        let python = "";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_invalid_python_source() {
        let python = "def foo(:\n    return";
        let mut builder = GraphBuilder::new();
        let result = builder.build_from_source(python);
        assert!(result.is_err());
        match result {
            Err(crate::GraphError::ParseError(msg)) => {
                assert!(!msg.is_empty());
            }
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn test_dependency_graph_default() {
        let graph = DependencyGraph::default();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_builder_default() {
        let builder = GraphBuilder::default();
        // GraphBuilder::default() should be equivalent to new()
        let _builder2 = GraphBuilder::new();
        // Both should parse the same source identically
        drop(builder);
    }

    #[test]
    fn test_get_node_nonexistent() {
        let graph = DependencyGraph::new();
        assert!(graph.get_node("nonexistent").is_none());
    }

    #[test]
    fn test_get_node_mut() {
        let python = "def foo():\n    return 42\n";
        let mut builder = GraphBuilder::new();
        let mut graph = builder.build_from_source(python).unwrap();

        // Mutate error count
        let node = graph.get_node_mut("foo").unwrap();
        node.error_count = 5;

        // Verify mutation persisted
        let node = graph.get_node("foo").unwrap();
        assert_eq!(node.error_count, 5);
    }

    #[test]
    fn test_get_node_mut_nonexistent() {
        let mut graph = DependencyGraph::new();
        assert!(graph.get_node_mut("nonexistent").is_none());
    }

    #[test]
    fn test_node_ids() {
        let python = "def alpha():\n    pass\n\ndef beta():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let ids = graph.node_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"alpha".to_string()));
        assert!(ids.contains(&"beta".to_string()));
    }

    #[test]
    fn test_add_edge_missing_from_node() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let mut graph = builder.build_from_source(python).unwrap();

        let edge = GraphEdge {
            kind: EdgeKind::Calls,
            weight: 1.0,
        };
        let added = graph.add_edge("nonexistent", "foo", edge);
        assert!(!added);
    }

    #[test]
    fn test_add_edge_missing_to_node() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let mut graph = builder.build_from_source(python).unwrap();

        let edge = GraphEdge {
            kind: EdgeKind::Calls,
            weight: 1.0,
        };
        let added = graph.add_edge("foo", "nonexistent", edge);
        assert!(!added);
    }

    #[test]
    fn test_add_edge_both_missing() {
        let mut graph = DependencyGraph::new();
        let edge = GraphEdge {
            kind: EdgeKind::Calls,
            weight: 1.0,
        };
        let added = graph.add_edge("a", "b", edge);
        assert!(!added);
    }

    #[test]
    fn test_add_edge_success() {
        let python = "def foo():\n    pass\n\ndef bar():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let mut graph = builder.build_from_source(python).unwrap();

        let edge = GraphEdge {
            kind: EdgeKind::Calls,
            weight: 2.0,
        };
        let added = graph.add_edge("bar", "foo", edge);
        assert!(added);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_incoming_edges_nonexistent_node() {
        let graph = DependencyGraph::new();
        let edges = graph.incoming_edges("nonexistent");
        assert!(edges.is_empty());
    }

    #[test]
    fn test_outgoing_edges_nonexistent_node() {
        let graph = DependencyGraph::new();
        let edges = graph.outgoing_edges("nonexistent");
        assert!(edges.is_empty());
    }

    #[test]
    fn test_import_statement_creates_module_node() {
        let python = "import os\n\ndef foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let import_node = graph.get_node("import:os");
        assert!(import_node.is_some());
        assert_eq!(import_node.unwrap().kind, NodeKind::Module);
    }

    #[test]
    fn test_multiple_imports() {
        let python = "import os\nimport sys\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        assert!(graph.get_node("import:os").is_some());
        assert!(graph.get_node("import:sys").is_some());
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_multiple_classes_with_methods() {
        let python = r#"
class Dog:
    def bark(self):
        pass

class Cat:
    def meow(self):
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        assert!(graph.get_node("Dog").is_some());
        assert!(graph.get_node("Cat").is_some());
        assert!(graph.get_node("Dog.bark").is_some());
        assert!(graph.get_node("Cat.meow").is_some());
        assert_eq!(graph.node_count(), 4);
    }

    #[test]
    fn test_chained_calls_edges() {
        let python = r#"
def a():
    return 1

def b():
    return a()

def c():
    return b()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);

        // c -> b
        let c_out = graph.outgoing_edges("c");
        assert_eq!(c_out.len(), 1);
        assert_eq!(c_out[0].0.id, "b");
        assert_eq!(c_out[0].1.kind, EdgeKind::Calls);

        // b -> a
        let b_out = graph.outgoing_edges("b");
        assert_eq!(b_out.len(), 1);
        assert_eq!(b_out[0].0.id, "a");
    }

    #[test]
    fn test_function_calling_multiple_functions() {
        let python = r#"
def x():
    return 1

def y():
    return 2

def z():
    return x() + y()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let z_out = graph.outgoing_edges("z");
        assert_eq!(z_out.len(), 2);
        let callee_names: Vec<&str> = z_out.iter().map(|(n, _)| n.id.as_str()).collect();
        assert!(callee_names.contains(&"x"));
        assert!(callee_names.contains(&"y"));
    }

    #[test]
    fn test_class_with_multiple_methods() {
        let python = r#"
class Calc:
    def add(self):
        pass
    def sub(self):
        pass
    def mul(self):
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        assert!(graph.get_node("Calc").is_some());
        assert!(graph.get_node("Calc.add").is_some());
        assert!(graph.get_node("Calc.sub").is_some());
        assert!(graph.get_node("Calc.mul").is_some());
        // 1 class + 3 methods
        assert_eq!(graph.node_count(), 4);
    }

    #[test]
    fn test_edge_kind_variants() {
        assert_ne!(EdgeKind::Calls, EdgeKind::Imports);
        assert_ne!(EdgeKind::Calls, EdgeKind::Inherits);
        assert_ne!(EdgeKind::Calls, EdgeKind::Accesses);
        assert_ne!(EdgeKind::Calls, EdgeKind::References);

        // Test Clone and Copy
        let kind = EdgeKind::Calls;
        let _cloned = kind;
        let _copied = kind;
    }

    #[test]
    fn test_node_kind_variants() {
        assert_ne!(NodeKind::Function, NodeKind::Class);
        assert_ne!(NodeKind::Function, NodeKind::Method);
        assert_ne!(NodeKind::Function, NodeKind::Module);
        assert_ne!(NodeKind::Function, NodeKind::Variable);

        // Test Clone and Copy
        let kind = NodeKind::Function;
        let _cloned = kind;
        let _copied = kind;
    }

    #[test]
    fn test_node_kind_serde_roundtrip() {
        let kind = NodeKind::Function;
        let json = serde_json::to_string(&kind).unwrap();
        let deserialized: NodeKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, deserialized);
    }

    #[test]
    fn test_edge_kind_serde_roundtrip() {
        let kind = EdgeKind::Inherits;
        let json = serde_json::to_string(&kind).unwrap();
        let deserialized: EdgeKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, deserialized);
    }

    #[test]
    fn test_offset_to_line_boundary() {
        let mut builder = GraphBuilder::new();
        builder.source = "line1\nline2\nline3\n".to_string();
        // offset 0 => line 1
        assert_eq!(builder.offset_to_line(0), 1);
        // offset 5 ('\n') => line 1 (before incrementing for the \n)
        assert_eq!(builder.offset_to_line(5), 1);
        // offset 6 ('l' of line2) => line 2
        assert_eq!(builder.offset_to_line(6), 2);
    }

    #[test]
    fn test_offset_to_column_boundary() {
        let mut builder = GraphBuilder::new();
        builder.source = "abc\ndef\n".to_string();
        // offset 0 => column 1
        assert_eq!(builder.offset_to_column(0), 1);
        // offset 1 => column 2
        assert_eq!(builder.offset_to_column(1), 2);
        // offset 4 => first char of next line => column 1
        assert_eq!(builder.offset_to_column(4), 1);
        // offset 5 => column 2
        assert_eq!(builder.offset_to_column(5), 2);
    }

    #[test]
    fn test_calls_in_if_body() {
        let python = r#"
def helper():
    return 1

def main():
    if True:
        helper()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let out = graph.outgoing_edges("main");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0.id, "helper");
    }

    #[test]
    fn test_calls_in_for_loop() {
        let python = r#"
def process():
    return 1

def main():
    for i in range(10):
        process()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let out = graph.outgoing_edges("main");
        // process is called (range is external so no edge)
        assert!(out.iter().any(|(n, _)| n.id == "process"));
    }

    #[test]
    fn test_calls_in_while_loop() {
        let python = r#"
def check():
    return True

def main():
    while check():
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "check"));
    }

    #[test]
    fn test_calls_in_assignment() {
        let python = r#"
def compute():
    return 42

def main():
    x = compute()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let out = graph.outgoing_edges("main");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0.id, "compute");
    }

    // ========================================================================
    // S9B7: Coverage tests for builder
    // ========================================================================

    #[test]
    fn test_s9b7_offset_to_line_empty_source() {
        let mut builder = GraphBuilder::new();
        builder.source = String::new();
        assert_eq!(builder.offset_to_line(0), 1);
        assert_eq!(builder.offset_to_line(100), 1);
    }

    #[test]
    fn test_s9b7_offset_to_column_empty_source() {
        let mut builder = GraphBuilder::new();
        builder.source = String::new();
        assert_eq!(builder.offset_to_column(0), 1);
    }

    #[test]
    fn test_s9b7_calls_in_if_else() {
        let python = r#"
def branch_a():
    return 1

def branch_b():
    return 2

def main():
    if True:
        branch_a()
    else:
        branch_b()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert_eq!(out.len(), 2);
        let names: Vec<&str> = out.iter().map(|(n, _)| n.id.as_str()).collect();
        assert!(names.contains(&"branch_a"));
        assert!(names.contains(&"branch_b"));
    }

    #[test]
    fn test_s9b7_calls_in_return_stmt() {
        let python = r#"
def helper():
    return 42

def main():
    return helper()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0.id, "helper");
    }

    #[test]
    fn test_s9b7_calls_via_binop() {
        let python = r#"
def left():
    return 1

def right():
    return 2

def main():
    return left() + right()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn test_s9b7_method_calls_method() {
        let python = r#"
class Foo:
    def helper(self):
        return 1

    def caller(self):
        return self.helper()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        // class + 2 methods
        assert_eq!(graph.node_count(), 3);
    }

    #[test]
    fn test_s9b7_node_ids_empty_graph() {
        let graph = DependencyGraph::new();
        assert!(graph.node_ids().is_empty());
    }

    #[test]
    fn test_s9b7_add_node_returns_index() {
        let mut graph = DependencyGraph::new();
        let node = GraphNode {
            id: "test_node".to_string(),
            kind: NodeKind::Function,
            file: std::path::PathBuf::new(),
            line: 1,
            column: 1,
            error_count: 0,
            impact_score: 0.0,
        };
        let idx = graph.add_node(node);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.graph[idx].id, "test_node");
    }

    #[test]
    fn test_s9b7_calls_nested_in_args() {
        let python = r#"
def inner():
    return 1

def outer(x):
    return x

def main():
    return outer(inner())
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.len() >= 2);
    }

    #[test]
    fn test_s9b7_class_calling_function() {
        let python = r#"
def utility():
    return 42

class MyClass:
    def method(self):
        utility()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("MyClass.method");
        assert!(out.iter().any(|(n, _)| n.id == "utility"));
    }

    #[test]
    fn test_multiple_inheritance_edges() {
        let python = r#"
class A:
    pass

class B:
    pass

class C(A, B):
    pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let c_out = graph.outgoing_edges("C");
        assert_eq!(c_out.len(), 2);
        let base_names: Vec<&str> = c_out.iter().map(|(n, _)| n.id.as_str()).collect();
        assert!(base_names.contains(&"A"));
        assert!(base_names.contains(&"B"));

        // All inheritance edges
        for (_, edge) in &c_out {
            assert_eq!(edge.kind, EdgeKind::Inherits);
        }
    }

    // ========================================================================
    // DEPYLER-99MODE-S11: Coverage tests for untested extract_calls branches
    // ========================================================================

    #[test]
    fn test_s11_calls_in_dict_values() {
        let python = r#"
def val_func():
    return 42

def main():
    d = {1: val_func()}
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "val_func"));
    }

    #[test]
    fn test_s11_calls_in_list_elements() {
        let python = r#"
def elem_func():
    return 1

def main():
    items = [elem_func(), 2, 3]
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "elem_func"));
    }

    #[test]
    fn test_s11_calls_in_compare_expr() {
        let python = r#"
def check():
    return 5

def main():
    if check() > 3:
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "check"));
    }

    #[test]
    fn test_s11_calls_in_compare_comparators() {
        let python = r#"
def threshold():
    return 10

def main():
    if 5 > threshold():
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "threshold"));
    }

    #[test]
    fn test_s11_calls_via_expr_stmt() {
        // Covers extract_calls_from_stmt for Stmt::Expr
        let python = r#"
def side_effect():
    pass

def main():
    side_effect()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0.id, "side_effect");
    }

    #[test]
    fn test_s11_calls_in_for_body() {
        let python = r#"
def process(x):
    return x

def main():
    for i in [1, 2]:
        process(i)
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "process"));
    }

    #[test]
    fn test_s11_calls_in_while_body() {
        let python = r#"
def tick():
    pass

def main():
    while True:
        tick()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "tick"));
    }

    #[test]
    fn test_s11_calls_with_class_constructor() {
        // Calling a class is like calling its __init__
        let python = r#"
class Widget:
    def render(self):
        pass

def main():
    w = Widget()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        // Widget is a known class, so edge should exist
        assert!(out.iter().any(|(n, _)| n.id == "Widget"));
    }

    #[test]
    fn test_s11_no_edge_for_external_function() {
        let python = r#"
def main():
    print("hello")
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        // print is not a known function, so no edge
        let out = graph.outgoing_edges("main");
        assert!(out.is_empty());
    }

    #[test]
    fn test_s11_calls_in_for_iter_expr() {
        let python = r#"
def get_items():
    return [1, 2, 3]

def main():
    for x in get_items():
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "get_items"));
    }

    #[test]
    fn test_s11_offset_to_line_multiline() {
        let mut builder = GraphBuilder::new();
        builder.source = "a\nb\nc\nd\ne".to_string();
        assert_eq!(builder.offset_to_line(0), 1); // 'a'
        assert_eq!(builder.offset_to_line(2), 2); // 'b'
        assert_eq!(builder.offset_to_line(4), 3); // 'c'
        assert_eq!(builder.offset_to_line(8), 5); // 'e'
    }

    // ========================================================================
    // S12: Deep coverage tests for builder
    // ========================================================================

    #[test]
    fn test_s12_calls_in_dict_keys_and_values() {
        let python = r#"
def key_fn():
    return "k"

def val_fn():
    return 42

def main():
    d = {key_fn(): val_fn()}
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        let names: Vec<&str> = out.iter().map(|(n, _)| n.id.as_str()).collect();
        assert!(names.contains(&"key_fn"), "Missing key_fn edge");
        assert!(names.contains(&"val_fn"), "Missing val_fn edge");
    }

    #[test]
    fn test_s12_calls_in_list_multiple() {
        let python = r#"
def a():
    return 1
def b():
    return 2
def main():
    items = [a(), b(), 3]
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        assert!(out.iter().any(|(n, _)| n.id == "a"));
        assert!(out.iter().any(|(n, _)| n.id == "b"));
    }

    #[test]
    fn test_s12_calls_in_compare_multiple_comparators() {
        // a < b() < c() -- multiple comparators
        let python = r#"
def lower():
    return 1
def upper():
    return 10
def main():
    if lower() < 5 < upper():
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let out = graph.outgoing_edges("main");
        let names: Vec<&str> = out.iter().map(|(n, _)| n.id.as_str()).collect();
        assert!(names.contains(&"lower"));
        assert!(names.contains(&"upper"));
    }

    #[test]
    fn test_s12_attribute_call_detection() {
        // self.method() style calls
        let python = r#"
class Foo:
    def helper(self):
        return 1

    def caller(self):
        x = self.helper()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        // Verify class and methods are parsed
        assert!(graph.get_node("Foo").is_some());
        assert!(graph.get_node("Foo.helper").is_some());
        assert!(graph.get_node("Foo.caller").is_some());
    }

    #[test]
    fn test_s12_empty_class_body() {
        let python = r#"
class Empty:
    pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        assert!(graph.get_node("Empty").is_some());
        assert_eq!(graph.get_node("Empty").unwrap().kind, NodeKind::Class);
    }

    #[test]
    fn test_s12_class_no_bases() {
        let python = r#"
class Standalone:
    def method(self):
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        // No inheritance edges
        let edges = graph.outgoing_edges("Standalone");
        assert!(edges.iter().all(|(_, e)| e.kind != EdgeKind::Inherits));
    }

    #[test]
    fn test_s12_from_import_not_tracked() {
        // from-imports (ImportFrom) are not currently tracked as module nodes
        // Only `import X` creates module nodes
        let python = "from os import path\n\ndef foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        // from-import falls into _ => {} catch-all, so no import node
        assert!(graph.get_node("import:os").is_none());
        // But function should still be tracked
        assert!(graph.get_node("foo").is_some());
    }

    #[test]
    fn test_s12_graph_node_serde() {
        let node = GraphNode {
            id: "test".to_string(),
            kind: NodeKind::Variable,
            file: std::path::PathBuf::from("file.py"),
            line: 42,
            column: 8,
            error_count: 3,
            impact_score: 1.5,
        };
        let json = serde_json::to_string(&node).unwrap();
        let back: GraphNode = serde_json::from_str(&json).unwrap();
        assert_eq!(back.kind, NodeKind::Variable);
        assert_eq!(back.line, 42);
        assert_eq!(back.error_count, 3);
    }

    #[test]
    fn test_s12_edge_kind_all_variants_serde() {
        for kind in [
            EdgeKind::Calls,
            EdgeKind::Imports,
            EdgeKind::Inherits,
            EdgeKind::Accesses,
            EdgeKind::References,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: EdgeKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }

    #[test]
    fn test_s12_node_kind_all_variants_serde() {
        for kind in [
            NodeKind::Function,
            NodeKind::Class,
            NodeKind::Method,
            NodeKind::Module,
            NodeKind::Variable,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: NodeKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }

    #[test]
    fn test_s12_deeply_nested_call_chain() {
        let python = r#"
def d():
    return 1
def c():
    return d()
def b():
    return c()
def a():
    return b()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        assert_eq!(graph.edge_count(), 3); // a->b, b->c, c->d
        let d_incoming = graph.incoming_edges("d");
        assert_eq!(d_incoming.len(), 1);
        assert_eq!(d_incoming[0].0.id, "c");
    }
}
