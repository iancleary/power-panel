{
  # Modified from https://gist.github.com/dit7ya/d479d89021a2534b7fa9fedf6f2fce43
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      buildInputs = with pkgs; [
        vulkan-loader
        wayland
        wayland-protocols
        libxkbcommon
      ];
      my-crate = crane.lib.${system}.buildPackage {
        src = ./.;
        inherit buildInputs;

        nativeBuildInputs = with pkgs; [
          pkg-config
          gtk-layer-shell
          gtk3
        ];
      };
    in {
      checks = {
        inherit my-crate;
      };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        # Extra inputs can be added here
      };
    });
}