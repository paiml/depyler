#!/usr/bin/env bash
#
# generate_example_tickets.sh - Generate Roadmap Tickets for All Examples
#
# Purpose: Create individual tickets (DEPYLER-XXXX) for each example file
# Author: Depyler Team
# Date: 2025-10-07
# Ticket: DEPYLER-0027
#
# Usage:
#   ./scripts/generate_example_tickets.sh > example_tickets.md

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLES_DIR="$PROJECT_ROOT/examples"

# Starting ticket ID
TICKET_START=29

# Find all .rs examples (exclude target directories)
mapfile -t EXAMPLES < <(find "$EXAMPLES_DIR" -type f -name "*.rs" -not -path "*/target/*" | sort)

TICKET_ID=$TICKET_START

echo "## 📋 Example Validation Tickets (66 examples)"
echo ""
echo "**Status**: Generated $(date -u +"%Y-%m-%d")"
echo "**Parent Ticket**: DEPYLER-0027"
echo "**Total Examples**: ${#EXAMPLES[@]}"
echo ""
echo "### Quality Gates (ALL must pass):"
echo ""
echo "1. ✅ **cargo clippy**: Zero warnings (\`--all-targets -- -D warnings\`)"
echo "2. ✅ **cargo test**: 100% pass rate (\`--all-features\`)"
echo "3. ✅ **cargo llvm-cov**: ≥80% coverage (\`--fail-under-lines 80\`)"
echo "4. ✅ **pmat tdg**: A- grade or higher (\`--min-grade A-\`)"
echo "5. ✅ **pmat complexity**: ≤10 cyclomatic (\`--max-cyclomatic 10\`)"
echo "6. ✅ **pmat satd**: Zero SATD (\`--fail-on-violation\`)"
echo ""
echo "---"
echo ""

# Group by category
echo "### 🎯 Priority 0: Showcase Examples (User-Facing)"
echo ""
for example in "${EXAMPLES[@]}"; do
    if [[ $example == */showcase/* ]]; then
        basename=$(basename "$example")
        relpath="${example#$PROJECT_ROOT/}"
        echo "#### **DEPYLER-$(printf "%04d" $TICKET_ID)**: Validate \`$basename\`"
        echo "- **File**: \`$relpath\`"
        echo "- **Priority**: P0 (Showcase - Critical)"
        echo "- **Status**: ⏳ Pending Validation"
        echo "- **Quality Gates**: [ ] Clippy [ ] Tests [ ] Coverage [ ] TDG [ ] Complexity [ ] SATD"
        echo ""
        ((TICKET_ID++))
    fi
done

echo "### 🔧 Priority 1: Core Feature Examples"
echo ""
for example in "${EXAMPLES[@]}"; do
    if [[ $example == */mathematical/* ]] || [[ $example =~ test_.*\.rs$ && ! $example =~ /showcase/ ]]; then
        basename=$(basename "$example")
        relpath="${example#$PROJECT_ROOT/}"
        echo "#### **DEPYLER-$(printf "%04d" $TICKET_ID)**: Validate \`$basename\`"
        echo "- **File**: \`$relpath\`"
        echo "- **Priority**: P1 (Core Feature)"
        echo "- **Status**: ⏳ Pending Validation"
        echo "- **Quality Gates**: [ ] Clippy [ ] Tests [ ] Coverage [ ] TDG [ ] Complexity [ ] SATD"
        echo ""
        ((TICKET_ID++))
    fi
done

echo "### 📦 Priority 2: Advanced Examples"
echo ""
for example in "${EXAMPLES[@]}"; do
    # Skip if already categorized
    if [[ $example == */showcase/* ]] || \
       [[ $example == */mathematical/* ]] || \
       [[ $example =~ test_.*\.rs$ && ! $example =~ /showcase/ ]]; then
        continue
    fi
    basename=$(basename "$example")
    relpath="${example#$PROJECT_ROOT/}"
    echo "#### **DEPYLER-$(printf "%04d" $TICKET_ID)**: Validate \`$basename\`"
    echo "- **File**: \`$relpath\`"
    echo "- **Priority**: P2 (Advanced)"
    echo "- **Status**: ⏳ Pending Validation"
    echo "- **Quality Gates**: [ ] Clippy [ ] Tests [ ] Coverage [ ] TDG [ ] Complexity [ ] SATD"
    echo ""
    ((TICKET_ID++))
done

echo "---"
echo ""
echo "## Validation Commands"
echo ""
echo "\`\`\`bash"
echo "# Validate all examples"
echo "make validate-examples"
echo ""
echo "# Validate specific example"
echo "make validate-example FILE=examples/showcase/binary_search.rs"
echo ""
echo "# Update ticket status after validation"
echo "./scripts/update_example_ticket_status.sh DEPYLER-XXXX [PASSED|FAILED]"
echo "\`\`\`"
