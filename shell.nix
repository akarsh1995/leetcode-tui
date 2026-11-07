{ pkgs ? import <nixpkgs> { } }:

let
  package = pkgs.callPackage ./default.nix { };
in
pkgs.mkShell {
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
}
