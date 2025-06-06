#!/bin/bash
# AST Inspection Demo for Marco Polo CLI

set -e

echo "🔍 Depyler AST Inspection Demo"
echo "=============================="
echo ""

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}📋 Available Inspection Options${NC}"
echo "1. python-ast - Original Python AST"
echo "2. hir - Depyler High-level Intermediate Representation"  
echo "3. typed-hir - Enhanced HIR with type analysis"
echo ""

echo -e "${YELLOW}🐍 Step 1: Python AST Inspection${NC}"
echo "Showing how Python code is parsed into an AST..."
echo ""
echo "Command: cargo run -- inspect marco_polo_simple.py --repr python-ast --format pretty"
echo ""
(cd ../.. && cargo run -- inspect examples/marco_polo_cli/marco_polo_simple.py --repr python-ast --format pretty) | head -20
echo "..."
echo ""

echo -e "${YELLOW}🦀 Step 2: HIR Inspection${NC}"
echo "Showing Depyler's intermediate representation with types and annotations..."
echo ""
echo "Command: cargo run -- inspect marco_polo_simple.py --repr hir --format pretty"
echo ""
(cd ../.. && cargo run -- inspect examples/marco_polo_cli/marco_polo_simple.py --repr hir --format pretty) | head -30
echo "..."
echo ""

echo -e "${YELLOW}📄 Step 3: JSON Output for Automation${NC}"
echo "Generating machine-readable JSON for integration with other tools..."
echo ""
echo "Command: cargo run -- inspect marco_polo_simple.py --repr hir --format json -o analysis.json"
(cd ../.. && cargo run -- inspect examples/marco_polo_cli/marco_polo_simple.py --repr hir --format json -o examples/marco_polo_cli/analysis.json)
echo "✅ JSON analysis saved to analysis.json"
echo ""

echo -e "${GREEN}📊 Step 4: Analysis Examples${NC}"
echo ""

if command -v jq &> /dev/null; then
    echo "📈 Function count:"
    jq '.functions | length' analysis.json
    echo ""
    
    echo "🔧 Function names:"
    jq -r '.functions[].name' analysis.json
    echo ""
    
    echo "⚡ Functions with optimization annotations:"
    jq -r '.functions[] | select(.annotations.optimization_level != "Conservative") | .name' analysis.json
    echo ""
    
    echo "🎯 Functions with special properties:"
    jq -r '.functions[] | select(.properties.is_pure == true or .properties.panic_free == true) | "\(.name): \(.properties)"' analysis.json
    echo ""
else
    echo "(Install jq for JSON analysis examples)"
    echo "Sample analysis.json content:"
    head -10 analysis.json
    echo "..."
fi

echo -e "${GREEN}🎉 Demo Complete!${NC}"
echo ""
echo "Try these commands yourself:"
echo "• cargo run -- inspect examples/marco_polo_cli/marco_polo_simple.py"
echo "• cargo run -- inspect examples/marco_polo_cli/marco_polo_simple.py --repr python-ast"
echo "• cargo run -- inspect examples/marco_polo_cli/marco_polo_simple.py --format json | jq '.'"
echo ""
echo "For more information, see: docs/ast-inspection.md"

# Clean up
rm -f analysis.json