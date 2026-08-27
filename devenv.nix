{
  pkgs,
  lib,
  config,
  ...
}:
{
  languages = {
    rust.enable = true;
  };

  packages = [
    # Build
    pkgs.clang
    pkgs.raylib
    pkgs.wayland
    pkgs.glfw
    pkgs.pkg-config
    pkgs.libX11
    pkgs.libXi
    pkgs.libXinerama
    pkgs.libXrandr
    pkgs.libXcursor
    pkgs.libxcb

    # Runtime binaries
    pkgs.grim
    pkgs.scrot
  ];

  env.LD_LIBRARY_PATH = lib.makeLibraryPath [
    pkgs.raylib
    pkgs.libx11
    pkgs.libxrandr
    pkgs.libxi
    pkgs.libxcursor
    pkgs.libxinerama
    pkgs.libxext
    pkgs.libxrender
    pkgs.libxfixes
    pkgs.libglvnd
  ];
}
