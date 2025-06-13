#!/bin/bash
set -euo pipefail

echo "🔧 Building Depyler WASM module..."

# Change to WASM crate directory
cd "$(dirname "$0")/../../crates/depyler-wasm"

# Clean previous build
rm -rf pkg/ target/

# Build with wasm-pack
echo "📦 Running wasm-pack build..."
wasm-pack build \
  --target web \
  --out-dir ../../playground/public/wasm \
  --release

# Change to output directory
cd ../../playground/public/wasm

# Optimize with wasm-opt if available
if command -v wasm-opt &> /dev/null; then
  echo "🔧 Optimizing WASM with wasm-opt..."
  wasm-opt -Oz \
    --enable-bulk-memory \
    -o optimized.wasm \
    depyler_wasm_bg.wasm
  
  # Replace original with optimized
  mv optimized.wasm depyler_wasm_bg.wasm
else
  echo "⚠️  wasm-opt not found, skipping optimization"
fi

# Compress for size measurement
echo "📊 Measuring bundle size..."
gzip -9 < depyler_wasm_bg.wasm > depyler.wasm.gz

# Check size
SIZE_BYTES=$(stat -c%s depyler.wasm.gz)
SIZE_KB=$((SIZE_BYTES / 1024))

echo "📏 WASM bundle size: ${SIZE_KB}KB (gzipped)"

# Verify size budget
BUDGET_KB=1500
if [ $SIZE_KB -gt $BUDGET_KB ]; then
  echo "❌ Error: WASM size ${SIZE_KB}KB exceeds ${BUDGET_KB}KB budget"
  exit 1
else
  echo "✅ WASM size within budget (${SIZE_KB}KB / ${BUDGET_KB}KB)"
fi

# Clean up temporary compression file
rm depyler.wasm.gz

# Create manifest file
cat > manifest.json << EOF
{
  "version": "$(date -u +%Y%m%d-%H%M%S)",
  "size_kb": $SIZE_KB,
  "build_time": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "optimized": $(command -v wasm-opt &> /dev/null && echo "true" || echo "false"),
  "files": [
    "depyler_wasm.js",
    "depyler_wasm_bg.wasm",
    "depyler_wasm.d.ts"
  ]
}
EOF

echo "✅ WASM build complete!"
echo "📂 Output: playground/public/wasm/"
echo "📄 Manifest: playground/public/wasm/manifest.json"