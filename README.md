# power-panel

## Description

Power Panel for NixOS, a reboot, shutdown GUI used with Hyprland.

![Screenshot](docs/ui-screenshot.png)

> Reboot, shutdown, and close/exit buttons are shown in the screenshot.

Authentication is not facilitated by this application. It is assumed that the process is allowed to authenticate the sudo commands with a fingerprint reader or other means. You can run executable in a shell that has already authenticated with sudo. Most display managers do not generally permit or encourage running GUIs as root as they are a security risk. This application is intended to be run as a user. See [#3](https://github.com/iancleary/power-panel/issues/3) for more information.

## Installation

Copy and import this [nix file](https://github.com/iancleary/nixos-config/blob/83a508afa45e4e07d8dee010abf43a877eee7cfe/modules/desktop/hyprland/power-panel.nix) into your configuration:

```nix
# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).

{ inputs, pkgs, ... }:
let
  # Mixing unstable and stable channels
  # https://nixos.wiki/index.php?title=FAQ&oldid=3528#How_can_I_install_a_package_from_unstable_while_remaining_on_the_stable_channel.3F
  pkgs-unstable = (import inputs.nixpkgs-unstable) {
    system = "x86_64-linux";
  };
in
{
  # https://nixos.org/manual/nixos/stable/index.html#sec-customising-packages
  environment.systemPackages =
    let
      power-panel = with pkgs-unstable; rustPlatform.buildRustPackage rec {
        pname = "power-panel";
        version = "v1.0.0";

        src = fetchFromGitHub {
          owner = "iancleary";
          repo = pname;
          rev = version;
          hash = "sha256-DKjbxdvtmxSuy2I0/N2Ed0xAnLa1d5rn/XjCGPV3UHE=";
        };

        # https://nixos.wiki/wiki/Packaging/Quirks_and_Caveats
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          wayland
          gtk4
        ];

        cargoHash = "sha256-XOSOaL7j93xrX/IhZzhpP5NR7Q9MyG2BSFGpsMwe82Q=";

        meta = with lib; {
          description = "Rust GUI app to restart and shutdown a system";
          homepage = "https://github.com/iancleary/power-panel";
          license = licenses.mit;
          maintainers = with maintainers; [ iancleary ];
          platforms = [ "x86_64-linux" "aarch64-linux" ];
        };
      };
    in
    [
      power-panel
    ];

}

```

For my use case, I integrated this file into my nixos-config this way; [checkout the files linked here](https://github.com/iancleary/nixos-config/commit/83a508afa45e4e07d8dee010abf43a877eee7cfe) to bind to a keyboard shortcut with [Hyprland](https://hyprland.org/).

## Why does this exist

I created this to learn and allow myself to reboot or shutdown quickly within a Hyprland windowing manager session. I also use `waybar` and it doesn't provide a power menu (like GNOME or KDE do, etc.).

It was a fun way to learn nix and rust packaging as well :).
