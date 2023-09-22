# power-panel

## Description

Power Panel for NixOS, a reboot, shutdown GUI used with Hyprland.

![Screenshot](docs/ui-screenshot.png)

> Reboot, shutdown, and close/exit buttons are shown in the screenshot.

Authentication is not facilitated by this application. It is assumed that the process is allowed to authenticate the sudo commands with a fingerprint reader or other means. You can run executable in a shell that has already authenticated with sudo. Most display managers do not generally permit or encourage running GUIs as root as they are a security risk. This application is intended to be run as a user. See [#3](https://github.com/iancleary/power-panel/issues/3) for more information.
