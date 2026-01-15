#!/bin/bash
# DEPYLER RAM DISK SETUP
# Institutionalized configuration for high-performance convergence testing
#
# Usage: source scripts/setup_ramdisk.sh
#
# This script:
# 1. Creates a 64GB RAM disk if not exists
# 2. Sets up required directories
# 3. Exports environment variables for cargo/rustc
# 4. Validates the setup

set -e

RAMDISK_NAME="DepylerRAM"
RAMDISK_PATH="/Volumes/${RAMDISK_NAME}"
RAMDISK_SIZE_GB=64
RAMDISK_SECTORS=$((RAMDISK_SIZE_GB * 1024 * 1024 * 2))  # 512-byte sectors

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           DEPYLER RAM DISK SETUP (INSTITUTIONALIZED)         ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check if RAM disk already exists
if mount | grep -q "${RAMDISK_PATH}"; then
    echo -e "${GREEN}✓ RAM disk already mounted at ${RAMDISK_PATH}${NC}"
else
    echo -e "${YELLOW}Creating ${RAMDISK_SIZE_GB}GB RAM disk...${NC}"

    # Check available memory
    TOTAL_MEM_GB=$(($(sysctl -n hw.memsize) / 1024 / 1024 / 1024))
    if [ "$TOTAL_MEM_GB" -lt 96 ]; then
        echo -e "${RED}ERROR: System has ${TOTAL_MEM_GB}GB RAM. Need at least 96GB for 64GB RAM disk.${NC}"
        echo "Reduce RAMDISK_SIZE_GB in this script if needed."
        return 1
    fi

    # Create and format RAM disk
    DEVICE=$(hdiutil attach -nomount "ram://${RAMDISK_SECTORS}")
    diskutil erasevolume HFS+ "${RAMDISK_NAME}" "${DEVICE}"

    echo -e "${GREEN}✓ RAM disk created at ${RAMDISK_PATH}${NC}"
fi

# Create required directories
echo "Setting up directories..."
mkdir -p "${RAMDISK_PATH}/cargo_target"
mkdir -p "${RAMDISK_PATH}/tmp"
mkdir -p "${RAMDISK_PATH}/rustc_cache"

# Export environment variables
export TMPDIR="${RAMDISK_PATH}/tmp"
export CARGO_TARGET_DIR="${RAMDISK_PATH}/cargo_target"
export CARGO_INCREMENTAL=0
export RUSTFLAGS="${RUSTFLAGS:--C debuginfo=0}"

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                    ENVIRONMENT CONFIGURED                    ║"
echo "╠══════════════════════════════════════════════════════════════╣"
printf "║ %-60s ║\n" "TMPDIR=${TMPDIR}"
printf "║ %-60s ║\n" "CARGO_TARGET_DIR=${CARGO_TARGET_DIR}"
printf "║ %-60s ║\n" "CARGO_INCREMENTAL=${CARGO_INCREMENTAL}"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Show disk status
echo "RAM Disk Status:"
df -h "${RAMDISK_PATH}"
echo ""

# Validation
echo "Validating setup..."
if [[ -w "${RAMDISK_PATH}/tmp" && -w "${RAMDISK_PATH}/cargo_target" ]]; then
    echo -e "${GREEN}✓ All directories writable${NC}"
else
    echo -e "${RED}✗ Directory permission issue${NC}"
    return 1
fi

echo ""
echo -e "${GREEN}RAM disk ready for convergence testing.${NC}"
echo "Run: cargo run --release --bin depyler -- converge --input-dir <path> --max-iterations 1"
