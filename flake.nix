{
  description = "Terminal UI for leetcode - Browse, solve, run and submit leetcode questions from TUI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        package = pkgs.callPackage ./default.nix { };
      in
      {
        packages.default = package;

        apps.default = {
          type = "app";
          program = "${package}/bin/leetui";
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ package ];
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
          ];

          RUST_BACKTRACE = "1";
          CARGO_INCREMENTAL = "1";
        };
      }
    );
}
