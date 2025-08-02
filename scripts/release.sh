#!/bin/bash
# Release script for Depyler - Toyota Way compliant

set -euo pipefail

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if version argument provided
if [ $# -eq 0 ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.1"
    exit 1
fi

VERSION=$1
VERSION_TAG="v${VERSION}"

echo -e "${BLUE}=== Depyler Release Process - ${VERSION_TAG} ===${NC}"
echo "Following Toyota Way: Zero Defects Policy"
echo ""

# Step 1: Run pre-release audit
echo -e "${YELLOW}Step 1: Running pre-release audit...${NC}"
if ./scripts/pre-release-audit.sh; then
    echo -e "${GREEN}✅ All quality gates passed${NC}"
else
    echo -e "${RED}❌ Quality gates failed - release blocked${NC}"
    exit 1
fi

# Step 2: Update version numbers
echo -e "\n${YELLOW}Step 2: Updating version numbers...${NC}"
# Update workspace version
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# Update all internal dependencies
find crates -name "Cargo.toml" -exec sed -i \
    -e "s/depyler-annotations = { version = \".*\"/depyler-annotations = { version = \"=${VERSION}\"/" \
    -e "s/depyler-core = { version = \".*\"/depyler-core = { version = \"=${VERSION}\"/" \
    -e "s/depyler-analyzer = { version = \".*\"/depyler-analyzer = { version = \"=${VERSION}\"/" \
    -e "s/depyler-verify = { version = \".*\"/depyler-verify = { version = \"=${VERSION}\"/" \
    -e "s/depyler-quality = { version = \".*\"/depyler-quality = { version = \"=${VERSION}\"/" \
    -e "s/depyler-mcp = { version = \".*\"/depyler-mcp = { version = \"=${VERSION}\"/" \
    -e "s/depyler-wasm = { version = \".*\"/depyler-wasm = { version = \"=${VERSION}\"/" \
    {} \;

# Update Cargo.lock
cargo update --workspace

echo -e "${GREEN}✅ Version updated to ${VERSION}${NC}"

# Step 3: Run quality checks
echo -e "\n${YELLOW}Step 3: Running quality checks...${NC}"
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps

echo -e "${GREEN}✅ All quality checks passed${NC}"

# Step 4: Commit and tag
echo -e "\n${YELLOW}Step 4: Creating release commit...${NC}"
git add -A
git commit -m "$(cat <<EOF
release: v${VERSION} - Zero Defect Release

Summary of changes following Toyota Way principles:
- Zero SATD policy maintained
- All quality gates passed
- Priority 1 implementation complete

Quality Metrics:
- SATD: 0
- Max Complexity: <20
- Tests: 100% passing
- Clippy: 0 warnings

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"

# Create signed tag
git tag -s -a ${VERSION_TAG} -m "$(cat <<EOF
Release ${VERSION_TAG}

Zero-defect release following Toyota Way.
See CHANGELOG.md for details.
EOF
)"

echo -e "${GREEN}✅ Release commit and tag created${NC}"

# Step 5: Push to GitHub
echo -e "\n${YELLOW}Step 5: Pushing to GitHub...${NC}"
echo -e "${BLUE}Ready to push ${VERSION_TAG} to GitHub.${NC}"
echo -e "This will trigger automated:"
echo "  - GitHub release creation"
echo "  - Publishing to crates.io"
echo "  - Release verification"
echo ""
read -p "Push to GitHub? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin main
    git push origin ${VERSION_TAG}
    echo -e "${GREEN}✅ Pushed to GitHub${NC}"
    echo ""
    echo -e "${BLUE}Monitor the release at:${NC}"
    echo "https://github.com/paiml/depyler/actions"
else
    echo -e "${YELLOW}Release prepared but not pushed.${NC}"
    echo "To push manually:"
    echo "  git push origin main"
    echo "  git push origin ${VERSION_TAG}"
fi

echo -e "\n${GREEN}✨ Release ${VERSION_TAG} complete!${NC}"