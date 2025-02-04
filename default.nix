{ pkgs ? import <nixpkgs> {} }:

let
  # Common dependencies for Rust crates
  commonDeps = attrs: {
    buildInputs = [ pkgs.pkg-config pkgs.openssl ];
    nativeBuildInputs = [ 
      pkgs.pkg-config 
      pkgs.openssl
      pkgs.rustc
      pkgs.cargo
    ];
  };

  # Crate-specific configurations
  crateConfigs = {
    leetcode-tui-config = attrs: {
      CARGO_CRATE_NAME = "leetcode_tui_config";
    };
    leetcode-tui-rs = attrs: (commonDeps attrs // {
      CARGO_CRATE_NAME = "leetcode_tui_rs";
    });
  };

  # Import and configure the workspace
  cargoNix = import ./Cargo.nix {
    inherit pkgs;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // crateConfigs;
  };
in
cargoNix.workspaceMembers."leetcode-tui-rs".build 