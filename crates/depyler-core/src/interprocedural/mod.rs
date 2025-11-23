//! Interprocedural analysis for cross-function mutation and borrowing inference
//!
//! This module implements interprocedural analysis to detect mutations and borrowing
//! requirements that span function boundaries. This enables:
//! - Detecting when a parameter is passed to a function that mutates it
//! - Propagating mutability requirements up the call chain
//! - Auto-inserting borrow operators at call sites
//! - Improving type inference across function boundaries
//!
//! # Architecture
//!
//! The analysis proceeds in phases:
//! 1. **Signature Collection**: Extract function signatures with parameter types and mutability
//! 2. **Call Graph Construction**: Build call graph to track function relationships
//! 3. **Mutation Propagation**: Use fixpoint iteration to propagate mutations through calls
//! 4. **Code Generation**: Apply inferred borrowing strategies and insert borrow operators
//!
//! # Example
//!
//! ```rust
//! use depyler_core::interprocedural::InterproceduralAnalyzer;
//! use depyler_core::hir::HirModule;
//!
//! let module = HirModule { /* ... */ };
//! let analyzer = InterproceduralAnalyzer::new(&module);
//! let analysis = analyzer.analyze();
//!
//! // Check if a parameter needs to be mutable
//! if analysis.is_param_mutated("use_helper", "state") {
//!     // Generate &mut State instead of &State
//! }
//! ```

pub mod call_analyzer;
pub mod call_graph;
pub mod diagnostics;
pub mod flow_analyzer;
pub mod intraprocedural;
pub mod mutation_propagation;
pub mod signature_registry;

pub use call_analyzer::{BorrowInsertion, CallAnalysisResult, CallSiteAnalyzer};
pub use call_graph::{CallGraph, CallGraphBuilder};
pub use diagnostics::{DiagnosticsGenerator, Evidence, MutabilityDiagnostic};
pub use flow_analyzer::{
    Confidence, FlowAnalysisResults, FlowAnalyzer, MutabilityKind, MutabilityReason,
    MutabilitySignature, ParameterMutability,
};
pub use intraprocedural::{
    ArgumentInfo, ArgumentSource, CallSite, IntraproceduralAnalyzer, IntraproceduralSummary,
    LocalMutability, Location, ParameterUsageAnalysis,
};
pub use mutation_propagation::{MutationInfo, MutationPropagator, PropagationResult};
pub use signature_registry::{FunctionSignature, FunctionSignatureRegistry, ParamSignature};

use crate::hir::HirModule;
use std::collections::{HashMap, HashSet};

/// Main interprocedural analysis coordinator (legacy)
///
/// Orchestrates the different phases of interprocedural analysis:
/// 1. Builds function signature registry
/// 2. Constructs call graph
/// 3. Propagates mutations through function calls
/// 4. Produces analysis results for code generation
#[derive(Debug)]
pub struct InterproceduralAnalyzer<'a> {
    /// Function signature registry
    registry: FunctionSignatureRegistry,
    /// Call graph
    call_graph: CallGraph,
    /// Mutation propagation results
    mutations: HashMap<String, MutationInfo>,
    /// Reference to the module being analyzed
    module: &'a HirModule,
}

/// New precise flow-based analyzer (recommended)
///
/// Uses bidirectional flow analysis for precise mutability inference:
/// 1. Intraprocedural analysis per function
/// 2. Call graph construction
/// 3. Bottom-up minimal mutability inference
/// 4. Top-down refinement
/// 5. Fixed-point iteration for recursion
pub struct PreciseInterproceduralAnalyzer<'a> {
    module: &'a HirModule,
    intraprocedural_summaries: HashMap<String, IntraproceduralSummary>,
    call_graph: CallGraph,
}

impl<'a> PreciseInterproceduralAnalyzer<'a> {
    /// Create a new precise analyzer
    pub fn new(module: &'a HirModule) -> Self {
        Self {
            module,
            intraprocedural_summaries: HashMap::new(),
            call_graph: CallGraph::new(),
        }
    }

    /// Run the complete flow-based analysis
    pub fn analyze(mut self) -> FlowAnalysisResults {
        // Phase 1: Intraprocedural analysis
        for func in &self.module.functions {
            let analyzer = IntraproceduralAnalyzer::new(func);
            let summary = analyzer.analyze();
            self.intraprocedural_summaries
                .insert(func.name.clone(), summary);
        }

        // Phase 2: Build call graph
        self.build_call_graph();

        // Phase 3: Flow analysis
        let flow_analyzer = FlowAnalyzer::new(&self.call_graph, self.intraprocedural_summaries);
        flow_analyzer.analyze()
    }

    /// Build call graph from intraprocedural summaries
    fn build_call_graph(&mut self) {
        // Add all functions
        for func_name in self.intraprocedural_summaries.keys() {
            self.call_graph.add_function(func_name.clone());
        }

        // Add call edges
        for (func_name, summary) in &self.intraprocedural_summaries {
            for call_site in &summary.all_call_sites {
                self.call_graph
                    .add_call(func_name.clone(), call_site.callee.clone());
            }
        }
    }
}

impl<'a> InterproceduralAnalyzer<'a> {
    /// Create a new analyzer for the given module
    pub fn new(module: &'a HirModule) -> Self {
        // Phase 1: Build signature registry
        let registry = FunctionSignatureRegistry::from_module(module);

        // Phase 2: Build call graph
        let call_graph = CallGraphBuilder::new(&registry).build(module);

        Self {
            registry,
            call_graph,
            mutations: HashMap::new(),
            module,
        }
    }

    /// Run the complete interprocedural analysis
    pub fn analyze(&mut self) -> InterproceduralAnalysis {
        // Phase 3: Propagate mutations
        let mut propagator =
            MutationPropagator::new(&self.registry, &self.call_graph).with_module(self.module);

        let result = propagator.propagate();
        self.mutations = result.mutations.clone();

        // Update registry with mutation information
        self.update_registry_with_mutations();

        InterproceduralAnalysis {
            registry: &self.registry,
            call_graph: &self.call_graph,
            mutations: &self.mutations,
            converged: result.converged,
            iterations: result.iterations,
        }
    }

    /// Update the signature registry with propagated mutation information
    fn update_registry_with_mutations(&mut self) {
        for (func_name, mutation_info) in &self.mutations {
            if let Some(sig) = self.registry.signatures.get_mut(func_name) {
                for param in &mut sig.params {
                    if mutation_info.mutated_params.contains(&param.name) {
                        param.set_mutated(true);
                    }
                }
            }
        }
    }

    /// Check if a function parameter is mutated (directly or through calls)
    pub fn is_param_mutated(&self, func_name: &str, param_name: &str) -> bool {
        if let Some(mutation_info) = self.mutations.get(func_name) {
            mutation_info.mutated_params.contains(param_name)
        } else {
            false
        }
    }

    /// Get the required borrowing strategy for a parameter
    pub fn get_param_borrowing(&self, func_name: &str, param_name: &str) -> Option<BorrowKind> {
        if self.is_param_mutated(func_name, param_name) {
            Some(BorrowKind::MutableBorrow)
        } else if self.is_param_borrowed(func_name, param_name) {
            Some(BorrowKind::ImmutableBorrow)
        } else {
            None
        }
    }

    /// Check if a parameter is borrowed (immutably)
    fn is_param_borrowed(&self, func_name: &str, param_name: &str) -> bool {
        if let Some(mutation_info) = self.mutations.get(func_name) {
            mutation_info.borrowed_params.contains(param_name)
        } else {
            false
        }
    }
}

/// Results of interprocedural analysis
#[derive(Debug)]
pub struct InterproceduralAnalysis<'a> {
    /// Function signature registry
    pub registry: &'a FunctionSignatureRegistry,
    /// Call graph
    pub call_graph: &'a CallGraph,
    /// Mutation information per function
    pub mutations: &'a HashMap<String, MutationInfo>,
    /// Whether the analysis converged
    pub converged: bool,
    /// Number of fixpoint iterations
    pub iterations: usize,
}

impl<'a> InterproceduralAnalysis<'a> {
    /// Check if a parameter needs to be mutable
    pub fn is_param_mutated(&self, func_name: &str, param_name: &str) -> bool {
        if let Some(mutation_info) = self.mutations.get(func_name) {
            mutation_info.mutated_params.contains(param_name)
        } else {
            false
        }
    }

    /// Get all functions that call a given function
    pub fn get_callers(&self, func_name: &str) -> Vec<&str> {
        self.call_graph.get_callers(func_name)
    }

    /// Get all functions called by a given function
    pub fn get_callees(&self, func_name: &str) -> Vec<&str> {
        self.call_graph.get_callees(func_name)
    }
}

/// Kind of borrowing required for a parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorrowKind {
    /// Immutable borrow (&T)
    ImmutableBorrow,
    /// Mutable borrow (&mut T)
    MutableBorrow,
    /// Take ownership (T)
    Move,
}

impl BorrowKind {
    /// Convert to Rust syntax
    pub fn to_rust_syntax(&self) -> &'static str {
        match self {
            BorrowKind::ImmutableBorrow => "&",
            BorrowKind::MutableBorrow => "&mut ",
            BorrowKind::Move => "",
        }
    }
}
