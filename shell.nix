{ pkgs ? import <nixpkgs> { } }:

let
  # Development tools
  devTools = with pkgs; [
    # Rust tooling
    cargo              # Rust package manager
    rustc              # Rust compiler
    rust-analyzer      # LSP Server
    rustfmt           # Formatter
    clippy            # Linter
    
    # System dependencies
    pkg-config
    openssl
    openssl.dev
  ];

  # Environment variables for development
  envVars = {
    # Enable better rust development experience
    RUST_BACKTRACE = "1";
    RUST_LOG = "info";
    CARGO_INCREMENTAL = "1";
    
    # OpenSSL configuration
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  };
in
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  
  # Add development tools
  buildInputs = devTools;
  
  # Set environment variables
  inherit (envVars) RUST_BACKTRACE RUST_LOG CARGO_INCREMENTAL OPENSSL_DIR OPENSSL_LIB_DIR;
}
