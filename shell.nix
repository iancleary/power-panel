{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

  # Additional tooling
  buildInputs = with pkgs; [
    rustup       # Rust toolchain
    just         # Task runner
  ];
}
