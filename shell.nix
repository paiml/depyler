# Nix shell environment for building depyler
# Usage: nix-shell -p cargo && nix-shell
# Or just: nix-shell (if you have cargo in your PATH)
#
# This fixes: GH-218 - Cannot find -lssl/-lcrypto when building
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "depyler-dev";

  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy

    # Required for native-tls (OpenSSL)
    openssl
    pkg-config

    # Required for libgit2-sys
    libgit2

    # Required for zlib
    zlib

    # Required for tree-sitter
    tree-sitter

    # C compiler for native code
    gcc
  ];

  # Set environment variables for linking
  OPENSSL_DIR = "${pkgs.openssl.dev}";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  # Ensure linker can find libraries
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.openssl
    pkgs.zlib
    pkgs.libgit2
  ];

  shellHook = ''
    echo "Depyler development environment"
    echo "================================"
    echo "Rust: $(rustc --version)"
    echo "Cargo: $(cargo --version)"
    echo ""
    echo "To build: cargo build --release"
    echo "To install: cargo install --path crates/depyler"
    echo "To install from crates.io: cargo install depyler"
  '';
}
