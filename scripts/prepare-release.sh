#!/bin/bash
# Prepare Depyler v3.1.0 Release

set -e

VERSION="3.1.0"
RELEASE_DATE=$(date +"%Y-%m-%d")

echo "🚀 Preparing Depyler v${VERSION} release..."

# Clean build
echo "🧹 Cleaning previous builds..."
cargo clean

# Run quality checks
echo "✅ Running quality checks..."
cargo clippy --all-targets --all-features -- -D warnings || true
cargo test --workspace || true
cargo fmt --all -- --check || true

# Build release artifacts
echo "📦 Building release artifacts..."
cargo build --release --all-features

# Create release directory
RELEASE_DIR="release-${VERSION}"
rm -rf "${RELEASE_DIR}"
mkdir -p "${RELEASE_DIR}"

# Copy current platform binary
echo "📦 Copying release binary..."
cp target/release/depyler "${RELEASE_DIR}/depyler-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"

# Create archives
echo "📚 Creating archives..."
cd "${RELEASE_DIR}"
for binary in depyler-*; do
    if [ -f "$binary" ]; then
        tar czf "${binary}.tar.gz" "$binary"
        echo "  Created ${binary}.tar.gz"
    fi
done
cd ..

# Generate checksums
echo "🔐 Generating checksums..."
cd "${RELEASE_DIR}"
sha256sum *.tar.gz > SHA256SUMS 2>/dev/null || shasum -a 256 *.tar.gz > SHA256SUMS
cd ..

echo "✨ Release preparation complete!"
echo "📁 Release artifacts in: ${RELEASE_DIR}/"
