{
  lib,
  stdenv,
  rustPlatform,
  pkg-config,
  openssl,
  glib,
  gtk3,
  libsoup_3,
  webkitgtk_4_1,
  darwin,
}:

let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
rustPlatform.buildRustPackage {
  pname = "leetui";
  inherit (cargoToml.workspace.package) version;

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [
    openssl
  ]
  ++ lib.optionals stdenv.isLinux [
    glib
    gtk3
    libsoup_3
    webkitgtk_4_1
  ]
  ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
    darwin.apple_sdk.frameworks.WebKit
  ];

  meta = with lib; {
    description = cargoToml.workspace.package.description;
    homepage = cargoToml.workspace.package.homepage;
    license = licenses.mit;
    mainProgram = "leetui";
  };
}
