{
  description = "Terminal UI for leetcode - Browse, solve, run and submit leetcode questions from TUI";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        packages = {
          default = pkgs.callPackage ./default.nix { };
        };

        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/leetui";
          };
          leetui = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/leetui";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl
            rust-bin.stable.latest.default
            rust-analyzer
            rustfmt
            clippy
          ];
          
          CARGO_INCREMENTAL = "1";
          RUST_BACKTRACE = "1";
        };
      }
    );
}
