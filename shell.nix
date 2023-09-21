{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];

  # Additional tooling
  buildInputs = with pkgs; [
    rustup       # Rust toolchain
    pkg-config   # Build tool
    just         # Task runner
    gtk4         # GUI toolkit
  ];

  # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
