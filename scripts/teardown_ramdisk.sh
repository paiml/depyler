#!/bin/bash
# DEPYLER RAM DISK TEARDOWN
# Safely unmounts and destroys the RAM disk
#
# Usage: ./scripts/teardown_ramdisk.sh

RAMDISK_NAME="DepylerRAM"
RAMDISK_PATH="/Volumes/${RAMDISK_NAME}"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           DEPYLER RAM DISK TEARDOWN                          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

if mount | grep -q "${RAMDISK_PATH}"; then
    echo "Current RAM disk usage:"
    df -h "${RAMDISK_PATH}"
    echo ""

    echo "Ejecting RAM disk..."
    diskutil eject "${RAMDISK_PATH}"

    echo "✓ RAM disk ejected. Memory freed."
else
    echo "No RAM disk mounted at ${RAMDISK_PATH}"
fi

# Unset environment variables
unset TMPDIR
unset CARGO_TARGET_DIR
echo "✓ Environment variables cleared."
