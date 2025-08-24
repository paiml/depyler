# Depyler v3.1.0 Multi-Platform Distribution Strategy

This document outlines the comprehensive distribution strategy for Depyler across major package ecosystems, enabling Python-to-Rust transpilation accessible to developers worldwide.

## 📦 Distribution Targets

### 1. **Cargo/crates.io** (Primary)
**Target**: Rust developers, systems programmers
```bash
# Installation
cargo install depyler
depyler transpile script.py --verify
```

**Status**: ✅ READY
- Already published to crates.io
- Primary distribution channel
- Full feature support including verification

---

### 2. **PyPI/pip** 
**Target**: Python developers wanting to transpile their code
```bash
# Installation (after publishing)
pip install depyler
python -m depyler transpile script.py
```

**Implementation Plan**:
1. Create Python wrapper using PyO3/maturin
2. Package with setuptools/poetry
3. Include pre-built wheels for major platforms
4. Test with: `pip install -e .`

**Benefits**: Direct access for Python developers, seamless integration

---

### 3. **Homebrew** 
**Target**: macOS/Linux developers
```bash
# Installation (after acceptance)
brew install depyler
depyler transpile --target=ruchy script.py
```

**Implementation**:
```ruby
class Depyler < Formula
  desc "Python-to-Rust transpiler with verification"
  homepage "https://github.com/paiml/depyler"
  version "3.1.0"
  
  if OS.mac?
    url "https://github.com/paiml/depyler/releases/download/v3.1.0/depyler-3.1.0-x86_64-apple-darwin.tar.gz"
    sha256 "SHA256_HERE"
  elsif OS.linux?
    url "https://github.com/paiml/depyler/releases/download/v3.1.0/depyler-3.1.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "SHA256_HERE"
  end
  
  def install
    bin.install "depyler"
  end
end
```

---

### 4. **npm/npx** 
**Target**: JavaScript/TypeScript developers, CI/CD pipelines
```bash
# Installation (after publishing)
npm install -g @depyler/cli
npx depyler transpile script.py --wasm
```

**Package Structure**:
```json
{
  "name": "@depyler/cli",
  "version": "3.1.0",
  "bin": {
    "depyler": "./bin/depyler"
  },
  "scripts": {
    "postinstall": "node install.js"
  }
}
```

**Benefits**: Easy CI/CD integration, no Rust toolchain needed

---

### 5. **Docker Hub** 
**Target**: Containerized deployments, cloud environments
```bash
# Usage (after publishing)
docker run -v $(pwd):/workspace depyler/depyler:3.1.0 transpile /workspace/script.py
docker run -it depyler/depyler:3.1.0 repl
```

**Dockerfile**:
```dockerfile
FROM rust:1.83-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features "full"

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/depyler /usr/local/bin/
ENTRYPOINT ["depyler"]
```

---

### 6. **Arch Linux AUR** 
**Target**: Arch Linux users
```bash
# Installation (after submission)
yay -S depyler
# or
paru -S depyler
```

**PKGBUILD**:
```bash
pkgname=depyler
pkgver=3.1.0
pkgrel=1
pkgdesc="Python-to-Rust transpiler with progressive verification"
arch=('x86_64' 'aarch64')
url="https://github.com/paiml/depyler"
license=('MIT' 'Apache')
depends=('gcc-libs')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/depyler" "$pkgdir/usr/bin/depyler"
}
```

---

### 7. **Debian/Ubuntu PPA** 
**Target**: Ubuntu/Debian servers and desktops
```bash
# Installation (after PPA setup)
sudo add-apt-repository ppa:paiml/depyler
sudo apt update
sudo apt install depyler
```

**debian/control**:
```
Package: depyler
Version: 3.1.0
Architecture: amd64
Maintainer: Depyler Contributors
Description: Python-to-Rust transpiler
 Transpiles Python code to safe, idiomatic Rust with
 progressive verification and energy-efficient code generation.
```

---

### 8. **Chocolatey** (Windows)
**Target**: Windows developers
```powershell
# Installation (after approval)
choco install depyler
depyler transpile script.py --output script.rs
```

**depyler.nuspec**:
```xml
<?xml version="1.0"?>
<package>
  <metadata>
    <id>depyler</id>
    <version>3.1.0</version>
    <title>Depyler</title>
    <authors>Depyler Contributors</authors>
    <projectUrl>https://github.com/paiml/depyler</projectUrl>
    <description>Python-to-Rust transpiler with verification</description>
  </metadata>
</package>
```

---

### 9. **WebAssembly/WASM** 
**Target**: Browser-based tools, online playgrounds
```javascript
// Usage in browser
import init, { transpile } from '@depyler/wasm';
await init();
const rustCode = transpile(pythonCode, { verify: true });
```

**Build Command**:
```bash
wasm-pack build --target web --out-dir pkg crates/depyler-wasm
```

---

### 10. **GitHub Releases** 
**Target**: Direct binary downloads
```bash
# Direct download
curl -L https://github.com/paiml/depyler/releases/download/v3.1.0/depyler-x86_64-linux -o depyler
chmod +x depyler
./depyler --version
```

**Release Artifacts**:
- `depyler-x86_64-linux`
- `depyler-x86_64-darwin`
- `depyler-x86_64-windows.exe`
- `depyler-aarch64-linux`
- `depyler-aarch64-darwin`

---

## 🚀 Release Automation

### GitHub Actions Workflow
```yaml
name: Multi-Platform Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-release:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
          
      - name: Build release binary
        run: cargo build --release --target ${{ matrix.target }}
        
      - name: Create release artifact
        run: |
          tar czf depyler-${{ github.ref_name }}-${{ matrix.target }}.tar.gz \
            -C target/${{ matrix.target }}/release depyler
            
      - name: Upload to GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: depyler-*.tar.gz
```

---

## 📋 Distribution Checklist

### Pre-Release
- [ ] Update version in `Cargo.toml` (workspace and all crates)
- [ ] Update CHANGELOG.md
- [ ] Run full test suite: `cargo test --workspace`
- [ ] Run quality gates: `make lint`
- [ ] Build all targets: `cargo build --release`
- [ ] Test WASM build: `wasm-pack build`

### Platform Packages
- [ ] **crates.io**: `cargo publish --dry-run`
- [ ] **PyPI**: Build wheels with `maturin build`
- [ ] **npm**: Test with `npm pack`
- [ ] **Docker**: Multi-arch build test
- [ ] **Homebrew**: Calculate SHA256 checksums
- [ ] **AUR**: Update PKGBUILD with new version

### Post-Release
- [ ] Create GitHub Release with binaries
- [ ] Publish to crates.io: `cargo publish`
- [ ] Push Docker images: `docker push depyler/depyler:3.1.0`
- [ ] Submit to package managers
- [ ] Update documentation site
- [ ] Announce on social media

---

## 🎯 Success Metrics

### Accessibility Goals
- **Before**: `cargo install` only (requires Rust toolchain)
- **After**: 10+ distribution channels, no Rust required

### Target Reach
- **PyPI**: ~10M Python developers
- **npm**: ~20M JavaScript developers
- **Docker Hub**: Enterprise/cloud deployments
- **Homebrew**: ~5M macOS developers
- **Package Managers**: Linux distributions

### Integration Points
- **Python projects**: `pip install depyler`
- **CI/CD pipelines**: Docker and npm integration
- **Web tools**: WASM for browser-based transpilation
- **IDEs**: Language server protocol support

---

## 📊 Phased Rollout Plan

### Phase 1: Core (Week 1)
1. **GitHub Releases** - Direct binary downloads
2. **crates.io** - Rust ecosystem
3. **Docker Hub** - Containerized deployments

### Phase 2: Language Ecosystems (Week 2-3)
1. **PyPI** - Python developers
2. **npm** - JavaScript/TypeScript developers
3. **WASM** - Browser-based tools

### Phase 3: Package Managers (Month 1-2)
1. **Homebrew** - macOS/Linux
2. **AUR** - Arch Linux
3. **APT/PPA** - Ubuntu/Debian
4. **Chocolatey** - Windows

---

## 🔧 Automation Tools

### Release Script
```bash
#!/bin/bash
# release-all.sh
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)

echo "📦 Building Depyler v$VERSION for all platforms..."

# Build all targets
for target in x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-msvc; do
  echo "Building for $target..."
  cross build --release --target $target
done

# Create tarballs
for binary in target/*/release/depyler*; do
  if [[ -f "$binary" ]]; then
    tar czf "$(basename $binary)-$VERSION.tar.gz" "$binary"
  fi
done

# Build WASM
wasm-pack build --target web --out-dir pkg crates/depyler-wasm

# Build Python wheels
maturin build --release

echo "✅ All builds complete!"
```

---

## 📚 Documentation

Each distribution channel includes:
- Installation instructions
- Quick start guide
- Platform-specific notes
- Troubleshooting section
- Links to main documentation

---

## 🤝 Community Support

- **GitHub Issues**: Platform-specific problems
- **Discord**: Real-time support
- **Documentation**: Comprehensive guides
- **Examples**: Platform-specific examples

**Making Python-to-Rust transpilation accessible to developers everywhere!** 🚀