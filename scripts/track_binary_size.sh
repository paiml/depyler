#!/bin/bash

# Binary Size Tracking and Optimization Script
# Following rash project's size optimization standards

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SIZE_THRESHOLD_MB=5  # Maximum acceptable size in MB for min-size build
RELEASE_THRESHOLD_MB=10  # Maximum acceptable size in MB for release build
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo -e "${BLUE}=== Depyler Binary Size Analysis ===${NC}"

# Function to get file size in bytes
get_file_size() {
    local file="$1"
    if [ -f "$file" ]; then
        stat -c%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# Function to format bytes to human readable
format_bytes() {
    local bytes="$1"
    if [ "$bytes" -eq 0 ]; then
        echo "N/A"
    elif [ "$bytes" -lt 1024 ]; then
        echo "${bytes}B"
    elif [ "$bytes" -lt 1048576 ]; then
        echo "$(( bytes / 1024 ))KB"
    elif [ "$bytes" -lt 1073741824 ]; then
        echo "$(( bytes / 1048576 ))MB"
    else
        echo "$(( bytes / 1073741824 ))GB"
    fi
}

# Function to get section sizes using 'size' command
get_section_info() {
    local binary="$1"
    if [ -f "$binary" ] && command -v size > /dev/null; then
        size "$binary" | awk 'NR==2 {print $1 ";" $2 ";" $3 ";" $4}'
    else
        echo "0;0;0;0"
    fi
}

# Function to build and measure size
build_and_measure() {
    local profile="$1"
    local profile_name="$2"
    local target_dir="$3"
    local binary_name="depyler"
    
    echo -e "${YELLOW}Building $profile_name...${NC}"
    
    # Build with specific profile
    if [ "$profile" = "min-size" ]; then
        cargo build --profile min-size --bin "$binary_name" > /dev/null 2>&1
    elif [ "$profile" = "release" ]; then
        cargo build --release --bin "$binary_name" > /dev/null 2>&1
    elif [ "$profile" = "dev" ]; then
        cargo build --bin "$binary_name" > /dev/null 2>&1
    else
        echo -e "${RED}Unknown profile: $profile${NC}"
        return 1
    fi
    
    local binary_path="$target_dir/$binary_name"
    
    if [ ! -f "$binary_path" ]; then
        echo -e "${RED}Binary not found: $binary_path${NC}"
        return 1
    fi
    
    # Get size information
    local file_size=$(get_file_size "$binary_path")
    local section_info=$(get_section_info "$binary_path")
    
    IFS=';' read -r text_size data_size bss_size total_size <<< "$section_info"
    
    echo -e "${GREEN}$profile_name Build Results:${NC}"
    echo "  File size:     $(format_bytes $file_size) ($file_size bytes)"
    echo "  Text section:  $(format_bytes $text_size) ($text_size bytes)"
    echo "  Data section:  $(format_bytes $data_size) ($data_size bytes)"
    echo "  BSS section:   $(format_bytes $bss_size) ($bss_size bytes)"
    echo "  Total in RAM:  $(format_bytes $total_size) ($total_size bytes)"
    
    # Size validation
    local size_mb=$(( file_size / 1048576 ))
    
    if [ "$profile" = "min-size" ] && [ "$size_mb" -gt "$SIZE_THRESHOLD_MB" ]; then
        echo -e "${RED}❌ Min-size build exceeds threshold: ${size_mb}MB > ${SIZE_THRESHOLD_MB}MB${NC}"
        return 1
    elif [ "$profile" = "release" ] && [ "$size_mb" -gt "$RELEASE_THRESHOLD_MB" ]; then
        echo -e "${RED}❌ Release build exceeds threshold: ${size_mb}MB > ${RELEASE_THRESHOLD_MB}MB${NC}"
        return 1
    else
        echo -e "${GREEN}✅ Size within acceptable limits${NC}"
    fi
    
    echo
    return 0
}

# Function to test stripping impact
test_stripping() {
    echo -e "${YELLOW}Testing symbol stripping impact...${NC}"
    
    # Build release version
    cargo build --release --bin depyler > /dev/null 2>&1
    
    local original_binary="target/release/depyler"
    local stripped_binary="$TEMP_DIR/depyler_stripped"
    
    if [ ! -f "$original_binary" ]; then
        echo -e "${RED}Release binary not found${NC}"
        return 1
    fi
    
    # Copy and strip
    cp "$original_binary" "$stripped_binary"
    strip "$stripped_binary"
    
    local original_size=$(get_file_size "$original_binary")
    local stripped_size=$(get_file_size "$stripped_binary")
    local size_reduction=$(( original_size - stripped_size ))
    local reduction_percent=$(( size_reduction * 100 / original_size ))
    
    echo "Original size:  $(format_bytes $original_size)"
    echo "Stripped size:  $(format_bytes $stripped_size)"
    echo "Size reduction: $(format_bytes $size_reduction) (${reduction_percent}%)"
    echo
}

# Function to test compression
test_compression() {
    echo -e "${YELLOW}Testing compression impact...${NC}"
    
    if ! command -v gzip > /dev/null; then
        echo "gzip not available, skipping compression test"
        return 0
    fi
    
    # Use min-size build for compression
    cargo build --profile min-size --bin depyler > /dev/null 2>&1
    
    local original_binary="target/min-size/depyler"
    
    if [ ! -f "$original_binary" ]; then
        echo -e "${RED}Min-size binary not found${NC}"
        return 1
    fi
    
    # Compress with gzip
    local compressed_file="$TEMP_DIR/depyler.gz"
    gzip -c "$original_binary" > "$compressed_file"
    
    local original_size=$(get_file_size "$original_binary")
    local compressed_size=$(get_file_size "$compressed_file")
    local compression_ratio=$(( compressed_size * 100 / original_size ))
    
    echo "Original size:    $(format_bytes $original_size)"
    echo "Compressed size:  $(format_bytes $compressed_size)"
    echo "Compression ratio: ${compression_ratio}%"
    
    # Test UPX if available
    if command -v upx > /dev/null; then
        echo -e "${YELLOW}Testing UPX compression...${NC}"
        local upx_binary="$TEMP_DIR/depyler_upx"
        cp "$original_binary" "$upx_binary"
        
        if upx --best "$upx_binary" > /dev/null 2>&1; then
            local upx_size=$(get_file_size "$upx_binary")
            local upx_ratio=$(( upx_size * 100 / original_size ))
            
            echo "UPX compressed:   $(format_bytes $upx_size)"
            echo "UPX ratio:        ${upx_ratio}%"
        else
            echo "UPX compression failed"
        fi
    else
        echo "UPX not available"
    fi
    
    echo
}

# Function to analyze dependencies impact
analyze_dependencies() {
    echo -e "${YELLOW}Analyzing dependency impact on binary size...${NC}"
    
    # Build with minimal features
    echo "Building with minimal features..."
    cargo build --release --no-default-features --bin depyler > /dev/null 2>&1
    local minimal_size=$(get_file_size "target/release/depyler")
    
    # Build with default features
    echo "Building with default features..."
    cargo build --release --bin depyler > /dev/null 2>&1
    local default_size=$(get_file_size "target/release/depyler")
    
    # Build with all features
    echo "Building with all features..."
    cargo build --release --all-features --bin depyler > /dev/null 2>&1
    local full_size=$(get_file_size "target/release/depyler")
    
    echo "Minimal features: $(format_bytes $minimal_size)"
    echo "Default features: $(format_bytes $default_size)"
    echo "All features:     $(format_bytes $full_size)"
    
    if [ "$minimal_size" -gt 0 ] && [ "$default_size" -gt 0 ]; then
        local default_overhead=$(( (default_size - minimal_size) * 100 / minimal_size ))
        echo "Default overhead: ${default_overhead}%"
    fi
    
    if [ "$default_size" -gt 0 ] && [ "$full_size" -gt 0 ]; then
        local full_overhead=$(( (full_size - default_size) * 100 / default_size ))
        echo "Full features overhead: ${full_overhead}%"
    fi
    
    echo
}

# Function to generate size report
generate_size_report() {
    local report_file="binary_size_report.md"
    
    echo "# Depyler Binary Size Report" > "$report_file"
    echo "" >> "$report_file"
    echo "**Generated:** $(date)" >> "$report_file"
    echo "" >> "$report_file"
    
    echo "## Build Profiles" >> "$report_file"
    echo "" >> "$report_file"
    
    # Capture current build sizes
    local dev_size="N/A"
    local release_size="N/A"
    local min_size="N/A"
    
    if [ -f "target/debug/depyler" ]; then
        dev_size=$(format_bytes $(get_file_size "target/debug/depyler"))
    fi
    
    if [ -f "target/release/depyler" ]; then
        release_size=$(format_bytes $(get_file_size "target/release/depyler"))
    fi
    
    if [ -f "target/min-size/depyler" ]; then
        min_size=$(format_bytes $(get_file_size "target/min-size/depyler"))
    fi
    
    echo "| Profile | Size |" >> "$report_file"
    echo "|---------|------|" >> "$report_file"
    echo "| Development | $dev_size |" >> "$report_file"
    echo "| Release | $release_size |" >> "$report_file"
    echo "| Min-Size | $min_size |" >> "$report_file"
    echo "" >> "$report_file"
    
    echo "## Optimization Recommendations" >> "$report_file"
    echo "" >> "$report_file"
    echo "- Use \`--profile min-size\` for distribution builds" >> "$report_file"
    echo "- Consider stripping symbols with \`strip\` command" >> "$report_file"
    echo "- Use compression (gzip/UPX) for network distribution" >> "$report_file"
    echo "- Minimize feature flags for specific use cases" >> "$report_file"
    
    echo "Size report generated: $report_file"
}

# Main execution
main() {
    echo "Starting comprehensive binary size analysis..."
    echo
    
    # Build and measure different profiles
    build_and_measure "dev" "Development" "target/debug" || true
    build_and_measure "release" "Release" "target/release" || true
    build_and_measure "min-size" "Min-Size" "target/min-size" || true
    
    # Additional analysis
    test_stripping
    test_compression
    analyze_dependencies
    
    # Generate report
    generate_size_report
    
    echo -e "${GREEN}✅ Binary size analysis complete!${NC}"
    echo "Check binary_size_report.md for detailed results."
}

# Run main function
main "$@"