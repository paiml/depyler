#!/bin/bash
# Prepare release for Depyler
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the version from command line or Cargo.toml
if [ $# -eq 0 ]; then
    VERSION=$(grep -E "^version" Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
else
    VERSION=$1
fi

echo -e "${GREEN}Preparing release v${VERSION}${NC}"

# Function to update version in a file
update_version() {
    local file=$1
    local pattern=$2
    local replacement=$3
    
    if [ -f "$file" ]; then
        echo "Updating version in $file"
        sed -i.bak "$pattern" "$file" && rm "${file}.bak"
    else
        echo -e "${YELLOW}Warning: $file not found${NC}"
    fi
}

# Update version in workspace Cargo.toml
echo -e "\n${GREEN}1. Updating version in Cargo.toml files${NC}"
update_version "Cargo.toml" "s/^version = .*/version = \"$VERSION\"/" ""

# Update version in release workflow
echo -e "\n${GREEN}2. Updating version in release workflow${NC}"
update_version ".github/workflows/release.yml" "s/VERSION=\"[^\"]*\"/VERSION=\"$VERSION\"/" ""

# Update version in CHANGELOG
echo -e "\n${GREEN}3. Updating CHANGELOG.md${NC}"
if grep -q "\[Unreleased\]" CHANGELOG.md; then
    # Add date to unreleased section
    DATE=$(date +%Y-%m-%d)
    sed -i.bak "s/## \[Unreleased\]/## [$VERSION] - $DATE/" CHANGELOG.md && rm CHANGELOG.md.bak
    
    # Add new unreleased section
    sed -i.bak "/## \[$VERSION\]/i\\
## [Unreleased]\\
\\
" CHANGELOG.md && rm CHANGELOG.md.bak
    
    # Update links
    sed -i.bak "s|\[Unreleased\]:.*|\[Unreleased\]: https://github.com/paiml/depyler/compare/v$VERSION...HEAD\\
[$VERSION]: https://github.com/paiml/depyler/releases/tag/v$VERSION|" CHANGELOG.md && rm CHANGELOG.md.bak
else
    echo -e "${YELLOW}Warning: No [Unreleased] section found in CHANGELOG.md${NC}"
fi

# Run tests to ensure everything works
echo -e "\n${GREEN}4. Running tests${NC}"
cargo test --workspace

# Check formatting
echo -e "\n${GREEN}5. Checking formatting${NC}"
cargo fmt --all -- --check

# Run clippy
echo -e "\n${GREEN}6. Running clippy${NC}"
cargo clippy --all-targets --all-features -- -D warnings

# Build release binary to ensure it compiles
echo -e "\n${GREEN}7. Building release binary${NC}"
cargo build --release --bin depyler

# Test the binary
echo -e "\n${GREEN}8. Testing release binary${NC}"
./target/release/depyler --version

# Generate lockfile
echo -e "\n${GREEN}9. Updating Cargo.lock${NC}"
cargo update --workspace

echo -e "\n${GREEN}✅ Release preparation complete!${NC}"
echo -e "\nNext steps:"
echo -e "1. Review and commit changes:"
echo -e "   ${YELLOW}git add -A && git commit -m \"chore: prepare release v$VERSION\"${NC}"
echo -e "2. Create and push tag:"
echo -e "   ${YELLOW}git tag -a v$VERSION -m \"Release v$VERSION\"${NC}"
echo -e "   ${YELLOW}git push origin main --tags${NC}"
echo -e "3. GitHub Actions will automatically:"
echo -e "   - Create a GitHub release"
echo -e "   - Build binaries for all platforms"
echo -e "   - Generate and upload install.sh"
echo -e "   - Create checksums"

# Checklist
echo -e "\n${GREEN}Pre-release checklist:${NC}"
echo "[ ] CHANGELOG.md updated with all changes"
echo "[ ] Version numbers consistent across all files"
echo "[ ] All tests passing"
echo "[ ] No clippy warnings"
echo "[ ] Documentation up to date"
echo "[ ] Examples working correctly"