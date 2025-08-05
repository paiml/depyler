use depyler_core::lsp::{LspServer, Position, Range};
use proptest::prelude::*;

// Generate valid URIs
prop_compose! {
    fn arb_uri()(
        filename in "[a-zA-Z][a-zA-Z0-9_]{0,15}",
        extension in prop::sample::select(vec!["py", "pyi"])
    ) -> String {
        format!("{}.{}", filename, extension)
    }
}

// Generate simple Python code
prop_compose! {
    fn arb_python_code()(
        func_name in "[a-z][a-z0-9_]{0,10}",
        var_name in "[a-z][a-z0-9_]{0,10}",
        value in 0..100i32
    ) -> String {
        format!("def {}():\n    {} = {}\n    return {}", func_name, var_name, value, var_name)
    }
}

// Generate positions within reasonable bounds
prop_compose! {
    fn arb_position()(
        line in 0..100usize,
        character in 0..100usize
    ) -> Position {
        Position { line, character }
    }
}

// Generate valid version numbers
prop_compose! {
    fn arb_version()(version in 0..1000i64) -> i64 {
        version
    }
}

proptest! {
    #[test]
    fn test_document_operations_never_panic(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version()
    ) {
        let mut server = LspServer::new();
        
        // Open document should never panic
        server.did_open(uri.clone(), text.clone(), version);
        
        // Change document should never panic
        server.did_change(uri.clone(), text, version + 1);
        
        // Close document should never panic
        server.did_close(uri);
    }

    #[test]
    fn test_document_change_idempotence(
        uri in arb_uri(),
        text1 in arb_python_code(),
        text2 in arb_python_code(),
        version in arb_version()
    ) {
        let mut server = LspServer::new();
        
        // Open with first text
        server.did_open(uri.clone(), text1, version);
        
        // Change to second text multiple times
        server.did_change(uri.clone(), text2.clone(), version + 1);
        server.did_change(uri.clone(), text2.clone(), version + 2);
        
        // Should work without issues
        let _ = server.diagnostics(&uri);
    }

    #[test]
    fn test_completion_position_bounds(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version(),
        line in 0..1000usize,
        character in 0..1000usize
    ) {
        let mut server = LspServer::new();
        
        server.did_open(uri.clone(), text, version);
        
        // Completion with out-of-bounds position should not panic
        let pos = Position { line, character };
        let _ = server.completion(&uri, pos);
    }

    #[test]
    fn test_completion_never_panics(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version(),
        pos in arb_position()
    ) {
        let mut server = LspServer::new();
        
        server.did_open(uri.clone(), text, version);
        
        // Completion should never panic
        let _ = server.completion(&uri, pos);
    }

    #[test]
    fn test_hover_never_panics(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version(),
        pos in arb_position()
    ) {
        let mut server = LspServer::new();
        
        server.did_open(uri.clone(), text, version);
        
        // Hover should never panic
        let _ = server.hover(&uri, pos);
    }

    #[test]
    fn test_diagnostics_never_panics(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version()
    ) {
        let mut server = LspServer::new();
        
        server.did_open(uri.clone(), text, version);
        
        // Diagnostics should never panic
        let _ = server.diagnostics(&uri);
    }

    #[test]
    fn test_goto_definition_never_panics(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version(),
        pos in arb_position()
    ) {
        let mut server = LspServer::new();
        
        server.did_open(uri.clone(), text, version);
        
        // Goto definition should never panic
        let _ = server.goto_definition(&uri, pos);
    }

    #[test]
    fn test_find_references_never_panics(
        uri in arb_uri(),
        text in arb_python_code(),
        version in arb_version(),
        pos in arb_position()
    ) {
        let mut server = LspServer::new();
        
        server.did_open(uri.clone(), text, version);
        
        // Find references should never panic
        let _ = server.find_references(&uri, pos);
    }

    #[test]
    fn test_multiple_documents(
        uris in prop::collection::vec(arb_uri(), 1..5),
        texts in prop::collection::vec(arb_python_code(), 1..5),
        version in arb_version()
    ) {
        let mut server = LspServer::new();
        
        // Ensure same number of URIs and texts
        let count = uris.len().min(texts.len());
        
        // Open all documents
        for i in 0..count {
            server.did_open(uris[i].clone(), texts[i].clone(), version + i as i64);
        }
        
        // Try to use features on all documents (should work)
        for i in 0..count {
            let _ = server.diagnostics(&uris[i]);
        }
        
        // Close all documents
        for i in 0..count {
            server.did_close(uris[i].clone());
        }
        
        // Try to use features after close (should handle gracefully)
        for i in 0..count {
            let diagnostics = server.diagnostics(&uris[i]);
            prop_assert!(diagnostics.is_empty());
        }
    }

    #[test]
    fn test_range_construction(
        line1 in 0..100usize,
        char1 in 0..100usize,
        line2 in 0..100usize,
        char2 in 0..100usize
    ) {
        // Construct a valid range by ensuring start <= end
        let (start, end) = if line1 < line2 || (line1 == line2 && char1 <= char2) {
            (Position { line: line1, character: char1 }, Position { line: line2, character: char2 })
        } else {
            (Position { line: line2, character: char2 }, Position { line: line1, character: char1 })
        };
        
        let range = Range { start: start.clone(), end: end.clone() };
        
        // Verify the range is valid
        prop_assert!(
            range.start.line < range.end.line ||
            (range.start.line == range.end.line && range.start.character <= range.end.character)
        );
    }
}

// Additional deterministic tests
#[test]
fn test_edge_case_operations() {
    let mut server = LspServer::new();
    
    // Test with empty document
    server.did_open("empty.py".to_string(), "".to_string(), 1);
    
    // These should not panic
    let _ = server.completion("empty.py", Position { line: 0, character: 0 });
    let _ = server.hover("empty.py", Position { line: 0, character: 0 });
    let _ = server.diagnostics("empty.py");
    let _ = server.goto_definition("empty.py", Position { line: 0, character: 0 });
    let _ = server.find_references("empty.py", Position { line: 0, character: 0 });
}

#[test]
fn test_document_lifecycle() {
    let mut server = LspServer::new();
    
    // Test complete document lifecycle
    let uri = "lifecycle.py".to_string();
    let text1 = "def test(): pass".to_string();
    let text2 = "def test():\n    return 42".to_string();
    
    // Open
    server.did_open(uri.clone(), text1, 1);
    
    // Change
    server.did_change(uri.clone(), text2, 2);
    
    // Use various features
    let _ = server.completion(&uri, Position { line: 1, character: 4 });
    let _ = server.hover(&uri, Position { line: 0, character: 4 });
    let _ = server.diagnostics(&uri);
    
    // Close
    server.did_close(uri.clone());
    
    // After close, operations should handle gracefully
    let _ = server.completion(&uri, Position { line: 0, character: 0 });
    let _ = server.hover(&uri, Position { line: 0, character: 0 });
    let _ = server.diagnostics(&uri);
}