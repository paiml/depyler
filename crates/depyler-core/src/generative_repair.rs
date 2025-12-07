//! Generative Code Repair Engine
//!
//! Integrates entrenar's MCTS (Monte Carlo Tree Search) and GAN capabilities
//! for generative code synthesis and repair.
//!
//! # Overview
//!
//! The generative repair engine uses:
//! - **MCTS Search**: For exploring the space of possible AST transformations
//! - **GAN Discriminator**: For validating generated Rust code (future)
//!
//! # Architecture
//!
//! ```text
//! HIR → CodeState → MCTS Search → Best Action → RustAst
//!                       ↓
//!              GAN Discriminator (validation)
//! ```
//!
//! # Feature Flag
//!
//! This module requires the `generative` feature to enable MCTS functionality.
//! Without the feature, a stub implementation is provided.

use crate::hir::HirModule;
use anyhow::Result;
use proc_macro2::TokenStream;
use std::hash::{Hash, Hasher};

#[cfg(feature = "generative")]
use entrenar::search::{
    Action, ActionSpace, MctsConfig, MctsSearch, Reward, State, StateSpace,
};

/// Configuration for the generative repair engine
#[derive(Debug, Clone)]
pub struct GenerativeRepairConfig {
    /// Maximum MCTS iterations
    pub max_iterations: usize,
    /// Exploration constant for UCB1
    pub exploration_constant: f64,
    /// Maximum simulation depth
    pub max_simulation_depth: usize,
    /// Whether to use GAN discriminator for validation
    pub use_discriminator: bool,
    /// Random seed for reproducibility (0 = random)
    pub seed: u64,
}

impl Default for GenerativeRepairConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            exploration_constant: std::f64::consts::SQRT_2,
            max_simulation_depth: 50,
            use_discriminator: false,
            seed: 0,
        }
    }
}

/// Represents the state of code generation (partial AST)
#[derive(Debug, Clone)]
#[allow(dead_code)] // is_complete is used only when "generative" feature is enabled
pub struct CodeState {
    /// Token representation of the partial AST
    tokens: Vec<String>,
    /// Whether this is a terminal (complete) state
    is_complete: bool,
}

impl CodeState {
    /// Create a new code state from tokens
    pub fn new(tokens: Vec<String>) -> Self {
        let is_complete = tokens.iter().any(|t| t == "EOF");
        Self { tokens, is_complete }
    }

    /// Create an empty initial state
    pub fn initial() -> Self {
        Self {
            tokens: vec![],
            is_complete: false,
        }
    }

    /// Get the current tokens
    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }
}

impl PartialEq for CodeState {
    fn eq(&self, other: &Self) -> bool {
        self.tokens == other.tokens
    }
}

impl Eq for CodeState {}

impl Hash for CodeState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tokens.hash(state);
    }
}

#[cfg(feature = "generative")]
impl State for CodeState {
    fn is_terminal(&self) -> bool {
        self.is_complete
    }
}

/// AST transformation action
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeAction {
    /// Name of the transformation
    name: String,
    /// Token to add/modify
    token: String,
}

impl CodeAction {
    /// Create a new code action
    pub fn new(name: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            token: token.into(),
        }
    }
}

#[cfg(feature = "generative")]
impl Action for CodeAction {
    fn name(&self) -> &str {
        &self.name
    }
}

/// State space for code generation
#[cfg(feature = "generative")]
pub struct CodeStateSpace {
    /// Target patterns to match (for reward calculation)
    target_patterns: Vec<String>,
}

#[cfg(feature = "generative")]
impl CodeStateSpace {
    /// Create a new code state space
    pub fn new(target_patterns: Vec<String>) -> Self {
        Self { target_patterns }
    }
}

#[cfg(feature = "generative")]
impl StateSpace<CodeState, CodeAction> for CodeStateSpace {
    fn apply(&self, state: &CodeState, action: &CodeAction) -> CodeState {
        let mut new_tokens = state.tokens.clone();
        new_tokens.push(action.token.clone());
        CodeState::new(new_tokens)
    }

    fn evaluate(&self, state: &CodeState) -> Reward {
        // Simple reward: 1.0 if tokens contain all target patterns, 0.0 otherwise
        let tokens_str = state.tokens.join(" ");
        let matches = self.target_patterns
            .iter()
            .filter(|p| tokens_str.contains(*p))
            .count();

        if self.target_patterns.is_empty() {
            0.5 // Neutral if no patterns
        } else {
            matches as f64 / self.target_patterns.len() as f64
        }
    }

    fn clone_space(&self) -> Box<dyn StateSpace<CodeState, CodeAction> + Send + Sync> {
        Box::new(Self {
            target_patterns: self.target_patterns.clone(),
        })
    }
}

/// Action space for code generation
#[cfg(feature = "generative")]
pub struct CodeActionSpace {
    /// Available actions from any state
    available_actions: Vec<CodeAction>,
}

#[cfg(feature = "generative")]
impl CodeActionSpace {
    /// Create a new code action space with default Rust tokens
    pub fn new() -> Self {
        Self {
            available_actions: vec![
                CodeAction::new("add_fn", "fn"),
                CodeAction::new("add_let", "let"),
                CodeAction::new("add_return", "return"),
                CodeAction::new("add_if", "if"),
                CodeAction::new("add_else", "else"),
                CodeAction::new("add_for", "for"),
                CodeAction::new("add_while", "while"),
                CodeAction::new("add_match", "match"),
                CodeAction::new("add_struct", "struct"),
                CodeAction::new("add_impl", "impl"),
                CodeAction::new("add_pub", "pub"),
                CodeAction::new("add_mut", "mut"),
                CodeAction::new("add_ref", "&"),
                CodeAction::new("add_semicolon", ";"),
                CodeAction::new("add_brace_open", "{"),
                CodeAction::new("add_brace_close", "}"),
                CodeAction::new("add_paren_open", "("),
                CodeAction::new("add_paren_close", ")"),
                CodeAction::new("add_arrow", "->"),
                CodeAction::new("add_i32", "i32"),
                CodeAction::new("add_bool", "bool"),
                CodeAction::new("add_string", "String"),
                CodeAction::new("complete", "EOF"),
            ],
        }
    }
}

#[cfg(feature = "generative")]
impl Default for CodeActionSpace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "generative")]
impl ActionSpace<CodeState, CodeAction> for CodeActionSpace {
    fn legal_actions(&self, state: &CodeState) -> Vec<CodeAction> {
        if state.is_terminal() {
            vec![]
        } else {
            self.available_actions.clone()
        }
    }
}

/// Generative repair engine for synthesizing Rust code from HIR
pub struct GenerativeRepair {
    config: GenerativeRepairConfig,
}

impl GenerativeRepair {
    /// Create a new generative repair engine with default config
    pub fn new() -> Self {
        Self {
            config: GenerativeRepairConfig::default(),
        }
    }

    /// Create a new generative repair engine with custom config
    pub fn with_config(config: GenerativeRepairConfig) -> Self {
        Self { config }
    }

    /// Synthesize Rust code from HIR using MCTS-guided search
    ///
    /// # Arguments
    ///
    /// * `hir` - The High-level Intermediate Representation to synthesize from
    ///
    /// # Returns
    ///
    /// Returns the synthesized Rust code as a TokenStream
    #[cfg(feature = "generative")]
    pub fn synthesize(&self, hir: &HirModule) -> Result<TokenStream> {
        // Extract target patterns from HIR
        let target_patterns = self.extract_target_patterns(hir);

        // Create MCTS components
        let mcts_config = MctsConfig {
            max_iterations: self.config.max_iterations,
            exploration_constant: self.config.exploration_constant,
            max_simulation_depth: self.config.max_simulation_depth,
            ..Default::default()
        };

        let initial_state = CodeState::initial();
        let action_space = CodeActionSpace::new();
        let state_space = CodeStateSpace::new(target_patterns);

        // Run MCTS search
        let mut mcts = if self.config.seed > 0 {
            MctsSearch::with_seed(initial_state, &action_space, mcts_config, self.config.seed)
        } else {
            MctsSearch::new(initial_state, &action_space, mcts_config)
        };

        let result = mcts.search(&state_space, &action_space, None);

        // Convert resulting state to TokenStream
        if let Some(state) = result.resulting_state {
            self.tokens_to_stream(&state)
        } else {
            Ok(TokenStream::new())
        }
    }

    /// Stub implementation when generative feature is disabled
    #[cfg(not(feature = "generative"))]
    pub fn synthesize(&self, _hir: &HirModule) -> Result<TokenStream> {
        // Stub implementation - requires "generative" feature for MCTS
        Ok(TokenStream::new())
    }

    /// Extract target patterns from HIR for guiding MCTS search
    #[cfg(feature = "generative")]
    fn extract_target_patterns(&self, hir: &HirModule) -> Vec<String> {
        let mut patterns = Vec::new();

        // Add function names as targets
        for func in &hir.functions {
            patterns.push(format!("fn {}", func.name));

            // Add parameter patterns
            for param in &func.params {
                patterns.push(param.name.clone());
            }

            // Add return type pattern if present
            if !matches!(func.ret_type, crate::hir::Type::Unknown | crate::hir::Type::None) {
                patterns.push("->".to_string());
            }
        }

        // Add struct names
        for class in &hir.classes {
            patterns.push(format!("struct {}", class.name));
        }

        patterns
    }

    /// Convert code state tokens to TokenStream
    #[cfg(feature = "generative")]
    fn tokens_to_stream(&self, state: &CodeState) -> Result<TokenStream> {
        let code = state.tokens()
            .iter()
            .filter(|t| *t != "EOF")
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");

        // Try to parse as Rust code
        match code.parse::<TokenStream>() {
            Ok(ts) => Ok(ts),
            Err(_) => {
                // Return empty if parsing fails
                Ok(TokenStream::new())
            }
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &GenerativeRepairConfig {
        &self.config
    }
}

impl Default for GenerativeRepair {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a generative synthesis operation
#[derive(Debug, Clone)]
pub struct SynthesisResult {
    /// Whether synthesis was successful
    pub success: bool,
    /// Generated code (if successful)
    pub code: Option<String>,
    /// Number of MCTS iterations performed
    pub iterations: usize,
    /// Expected reward of the best path
    pub expected_reward: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_empty_hir() -> HirModule {
        HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        }
    }

    #[test]
    fn test_generative_synthesis_stub() {
        // TDD Red: This test validates the basic API exists
        let repair = GenerativeRepair::new();
        let hir = create_empty_hir();

        // Attempt to call synthesize
        let result = repair.synthesize(&hir);

        // For now, just verify it doesn't panic and returns Ok
        assert!(result.is_ok(), "synthesize should return Ok for empty HIR");
    }

    #[test]
    fn test_generative_repair_config_default() {
        let config = GenerativeRepairConfig::default();
        assert_eq!(config.max_iterations, 100);
        assert!(config.exploration_constant > 0.0);
        assert_eq!(config.max_simulation_depth, 50);
        assert!(!config.use_discriminator);
        assert_eq!(config.seed, 0);
    }

    #[test]
    fn test_generative_repair_with_config() {
        let config = GenerativeRepairConfig {
            max_iterations: 500,
            exploration_constant: 2.0,
            max_simulation_depth: 100,
            use_discriminator: true,
            seed: 42,
        };

        let repair = GenerativeRepair::with_config(config);
        assert_eq!(repair.config().max_iterations, 500);
        assert!(repair.config().use_discriminator);
        assert_eq!(repair.config().seed, 42);
    }

    #[test]
    fn test_code_state_creation() {
        let state = CodeState::new(vec!["fn".to_string(), "test".to_string()]);
        assert_eq!(state.tokens().len(), 2);
        assert!(!state.is_complete);
    }

    #[test]
    fn test_code_state_terminal() {
        let state = CodeState::new(vec!["fn".to_string(), "EOF".to_string()]);
        assert!(state.is_complete);
    }

    #[test]
    fn test_code_action_creation() {
        let action = CodeAction::new("add_fn", "fn");
        assert_eq!(action.name, "add_fn");
        assert_eq!(action.token, "fn");
    }

    #[test]
    fn test_synthesis_result_default() {
        let result = SynthesisResult {
            success: true,
            code: Some("fn test() {}".to_string()),
            iterations: 100,
            expected_reward: 0.95,
        };
        assert!(result.success);
        assert!(result.code.is_some());
        assert_eq!(result.iterations, 100);
    }

    #[cfg(feature = "generative")]
    mod generative_tests {
        use super::*;

        #[test]
        fn test_code_action_space_default() {
            let action_space = CodeActionSpace::new();
            let state = CodeState::initial();
            let actions = action_space.legal_actions(&state);

            // Should have available actions
            assert!(!actions.is_empty());

            // Should include common Rust tokens
            let action_names: Vec<_> = actions.iter().map(|a| a.name.as_str()).collect();
            assert!(action_names.contains(&"add_fn"));
            assert!(action_names.contains(&"add_let"));
            assert!(action_names.contains(&"complete"));
        }

        #[test]
        fn test_code_state_space_evaluate() {
            let state_space = CodeStateSpace::new(vec!["fn".to_string(), "test".to_string()]);

            // Empty state
            let empty = CodeState::initial();
            let reward_empty = state_space.evaluate(&empty);
            assert_eq!(reward_empty, 0.0);

            // Partial match
            let partial = CodeState::new(vec!["fn".to_string()]);
            let reward_partial = state_space.evaluate(&partial);
            assert!(reward_partial > 0.0);
            assert!(reward_partial < 1.0);

            // Full match
            let full = CodeState::new(vec!["fn".to_string(), "test".to_string()]);
            let reward_full = state_space.evaluate(&full);
            assert_eq!(reward_full, 1.0);
        }

        #[test]
        fn test_mcts_integration() {
            let config = GenerativeRepairConfig {
                max_iterations: 10,
                seed: 42,
                ..Default::default()
            };

            let repair = GenerativeRepair::with_config(config);
            let hir = create_empty_hir();

            let result = repair.synthesize(&hir);
            assert!(result.is_ok());
        }
    }
}
