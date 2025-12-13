#!/bin/bash
# Depyler Installation Script
# Automatically installs the latest version of Depyler

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS and architecture
OS=$(uname -s | tr '[[:upper:]]' '[[:lower:]]')
ARCH=$(uname -m)

# Map to release naming convention
case "$OS" in
    darwin)
        OS="macos"
        ;;
    linux)
        OS="linux"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH="x64"
        ;;
    aarch64|arm64)
        ARCH="arm64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

BINARY_NAME="depyler-${OS}-${ARCH}"

echo -e "${GREEN}🚀 Installing Depyler${NC}"
echo "   OS: $OS"
echo "   Architecture: $ARCH"
echo ""

# Check for required tools
if ! command -v curl &> /dev/null; then
    echo -e "${RED}Error: curl is required but not installed${NC}"
    exit 1
fi

# Get latest release URL
echo "📦 Fetching latest release..."
LATEST_URL=$(curl -s https://api.github.com/repos/paiml/depyler/releases/latest | \
    grep "browser_download_url.*${BINARY_NAME}.tar.gz" | \
    cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo -e "${YELLOW}Binary not found for ${BINARY_NAME}${NC}"
    echo "Falling back to cargo installation..."
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo is required for source installation${NC}"
        echo "Install Rust from https://rustup.rs/"
        exit 1
    fi
    
    cargo install depyler
    echo -e "${GREEN}✅ Depyler installed via cargo${NC}"
    exit 0
fi

# Download and install
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR" || exit 1

echo "📥 Downloading from: $LATEST_URL"
curl -L -o depyler.tar.gz "$LATEST_URL"

echo "📂 Extracting..."
tar xzf depyler.tar.gz

# Install to /usr/local/bin or ~/.local/bin
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/depyler"
    sudo chmod +x "$INSTALL_DIR/depyler"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    mv "$BINARY_NAME" "$INSTALL_DIR/depyler"
    chmod +x "$INSTALL_DIR/depyler"
    
    # Add to PATH if needed
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo -e "${YELLOW}Adding $INSTALL_DIR to PATH${NC}"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc 2>/dev/null || true
        export PATH="$HOME/.local/bin:$PATH"
    fi
fi

# Cleanup
cd / || exit 1
if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
    rm -rf "$TEMP_DIR"
fi

# Verify installation
if command -v depyler &> /dev/null; then
    VERSION=$(depyler --version | cut -d' ' -f2)
    echo -e "${GREEN}✅ Depyler v${VERSION} installed successfully!${NC}"
    echo ""
    echo "Get started:"
    echo "  depyler --help              # Show help"
    echo "  depyler transpile file.py   # Transpile Python to Rust"
    echo "  depyler agent start         # Start background agent"
    echo ""
    echo "For Claude Code integration, see: https://github.com/paiml/depyler/blob/main/AGENT.md"
else
    echo -e "${RED}Installation may have failed. Please check your PATH${NC}"
    exit 1
fi