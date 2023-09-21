{ pkgs ? import <nixpkgs> { } }:
let 
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    wayland
    gtk4
  ];

  # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

}


