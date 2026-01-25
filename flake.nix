{
  description = "Depyler - Python to Rust transpiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "depyler-dev";

          buildInputs = with pkgs; [
            # Rust toolchain via overlay
            rustToolchain

            # Required for native-tls (OpenSSL) - GH-218
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

          # OpenSSL environment variables
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          # Library paths
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.openssl
            pkgs.zlib
            pkgs.libgit2
          ];

          shellHook = ''
            echo "Depyler development environment (Nix Flake)"
            echo "============================================"
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            echo ""
            echo "To build: cargo build --release"
            echo "To install: cargo install --path crates/depyler"
            echo "To install from crates.io: cargo install depyler"
          '';
        };

        # Package definition for `nix build`
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "depyler";
          version = "3.22.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
            zlib
            libgit2
          ];

          # Skip tests during build (run separately)
          doCheck = false;

          meta = with pkgs.lib; {
            description = "Python to Rust transpiler focusing on energy-efficient, safe code generation";
            homepage = "https://github.com/paiml/depyler";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      }
    );
}
