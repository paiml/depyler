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
        let inherits_base = edges.iter().any(|(node, edge)| {
            node.id == "Base" && edge.kind == EdgeKind::Inherits
        });
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
}
