//! Language Server Protocol (LSP) implementation for Depyler
//!
//! Provides LSP server functionality for IDE integration.

use crate::ide::{DiagnosticSeverity, IdeIntegration};
use crate::{hir, DepylerPipeline};
use rustpython_parser::text_size::TextSize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LSP server state
pub struct LspServer {
    documents: HashMap<String, DocumentState>,
    pipeline: DepylerPipeline,
}

struct DocumentState {
    content: String,
    _version: i64,
    ide_integration: IdeIntegration,
    _hir: Option<hir::HirModule>,
}

impl LspServer {
    /// Create a new LSP server instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::lsp::LspServer;
    ///
    /// let server = LspServer::new();
    /// assert!(server.documents.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            pipeline: DepylerPipeline::new(),
        }
    }

    /// Handle document open
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::lsp::LspServer;
    ///
    /// let mut server = LspServer::new();
    /// server.did_open("file.py".to_string(), "def test(): pass".to_string(), 1);
    /// assert!(server.documents.contains_key("file.py"));
    /// ```
    pub fn did_open(&mut self, uri: String, text: String, version: i64) {
        let mut ide = IdeIntegration::new();

        // Try to parse and analyze the document
        if let Ok(hir_module) = self.pipeline.parse_to_hir(&text) {
            ide.index_symbols(&hir_module, &text);

            self.documents.insert(
                uri.clone(),
                DocumentState {
                    content: text,
                    _version: version,
                    ide_integration: ide,
                    _hir: Some(hir_module),
                },
            );
        } else {
            // Document has parse errors
            self.documents.insert(
                uri,
                DocumentState {
                    content: text,
                    _version: version,
                    ide_integration: ide,
                    _hir: None,
                },
            );
        }
    }

    /// Handle document change
    pub fn did_change(&mut self, uri: String, text: String, version: i64) {
        self.did_open(uri, text, version); // Re-analyze
    }

    /// Handle document close
    pub fn did_close(&mut self, uri: String) {
        self.documents.remove(&uri);
    }

    /// Get completions at position
    pub fn completion(&self, uri: &str, position: Position) -> CompletionResponse {
        if let Some(doc) = self.documents.get(uri) {
            let offset = self.position_to_offset(&doc.content, position);
            let prefix = self.get_prefix_at_position(&doc.content, offset);

            let items = doc
                .ide_integration
                .completions_at_position(offset, &prefix)
                .into_iter()
                .map(|item| CompletionItemLsp {
                    label: item.label,
                    kind: Some(match item.kind {
                        crate::ide::CompletionKind::Function => 3,
                        crate::ide::CompletionKind::Class => 7,
                        crate::ide::CompletionKind::Method => 2,
                        crate::ide::CompletionKind::Variable => 6,
                        crate::ide::CompletionKind::Field => 5,
                        crate::ide::CompletionKind::Module => 9,
                    }),
                    detail: item.detail,
                    documentation: item.documentation,
                })
                .collect();

            CompletionResponse { items }
        } else {
            CompletionResponse { items: vec![] }
        }
    }

    /// Get hover information
    pub fn hover(&self, uri: &str, position: Position) -> Option<HoverResponse> {
        if let Some(doc) = self.documents.get(uri) {
            let offset = self.position_to_offset(&doc.content, position);

            if let Some(symbol) = doc.ide_integration.symbol_at_position(offset) {
                let contents = crate::ide::generate_hover_info(symbol);
                return Some(HoverResponse {
                    contents: MarkupContent {
                        kind: "markdown".to_string(),
                        value: contents,
                    },
                });
            }
        }
        None
    }

    /// Get diagnostics for a document
    pub fn diagnostics(&self, uri: &str) -> Vec<DiagnosticLsp> {
        if let Some(doc) = self.documents.get(uri) {
            doc.ide_integration
                .diagnostics()
                .iter()
                .map(|diag| {
                    let start = self.offset_to_position(&doc.content, diag.range.start());
                    let end = self.offset_to_position(&doc.content, diag.range.end());

                    DiagnosticLsp {
                        range: Range { start, end },
                        severity: Some(match diag.severity {
                            DiagnosticSeverity::Error => 1,
                            DiagnosticSeverity::Warning => 2,
                            DiagnosticSeverity::Information => 3,
                            DiagnosticSeverity::Hint => 4,
                        }),
                        code: diag.code.clone(),
                        source: Some(diag.source.clone()),
                        message: diag.message.clone(),
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    /// Go to definition
    pub fn goto_definition(&self, uri: &str, position: Position) -> Option<LocationResponse> {
        if let Some(doc) = self.documents.get(uri) {
            let offset = self.position_to_offset(&doc.content, position);

            // Find symbol at position
            if let Some(symbol) = doc.ide_integration.symbol_at_position(offset) {
                // For now, return the symbol's own location
                // In a full implementation, this would resolve imports, etc.
                let start = self.offset_to_position(&doc.content, symbol.range.start());
                let end = self.offset_to_position(&doc.content, symbol.range.end());

                return Some(LocationResponse {
                    uri: uri.to_string(),
                    range: Range { start, end },
                });
            }
        }
        None
    }

    /// Find references
    pub fn find_references(&self, uri: &str, position: Position) -> Vec<LocationResponse> {
        if let Some(doc) = self.documents.get(uri) {
            let offset = self.position_to_offset(&doc.content, position);

            if let Some(symbol) = doc.ide_integration.symbol_at_position(offset) {
                let refs = doc.ide_integration.find_references(&symbol.name);

                return refs
                    .into_iter()
                    .map(|sym| {
                        let start = self.offset_to_position(&doc.content, sym.range.start());
                        let end = self.offset_to_position(&doc.content, sym.range.end());

                        LocationResponse {
                            uri: uri.to_string(),
                            range: Range { start, end },
                        }
                    })
                    .collect();
            }
        }
        vec![]
    }

    // Helper methods
    fn position_to_offset(&self, text: &str, position: Position) -> TextSize {
        let mut line = 0;
        let mut col = 0;
        let mut offset = 0;

        for ch in text.chars() {
            if line == position.line && col == position.character {
                return TextSize::from(offset as u32);
            }

            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            offset += ch.len_utf8();
        }

        TextSize::from(offset as u32)
    }

    fn offset_to_position(&self, text: &str, offset: TextSize) -> Position {
        let mut line = 0;
        let mut col = 0;
        let mut current_offset = 0;

        for ch in text.chars() {
            let offset_usize: usize = offset.into();
            if current_offset >= offset_usize {
                break;
            }

            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            current_offset += ch.len_utf8();
        }

        Position {
            line,
            character: col,
        }
    }

    fn get_prefix_at_position(&self, text: &str, offset: TextSize) -> String {
        let offset_usize: usize = offset.into();
        let start = text[..offset_usize]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        text[start..offset_usize].to_string()
    }
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

// LSP protocol types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemLsp {
    pub label: String,
    pub kind: Option<i32>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub items: Vec<CompletionItemLsp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverResponse {
    pub contents: MarkupContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupContent {
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticLsp {
    pub range: Range,
    pub severity: Option<i32>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResponse {
    pub uri: String,
    pub range: Range,
}

#[cfg(test)]
#[path = "lsp_tests.rs"]
mod tests;
