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
    pkgs.wayland
    pkgs.glfw
    pkgs.pkg-config
    pkgs.raylib
    pkgs.cmake
    pkgs.alsa-lib
    pkgs.libx11
    pkgs.libxrandr
    pkgs.libxi
    pkgs.libxcursor
    pkgs.libxinerama
    pkgs.libxkbcommon
    pkgs.libxext
    pkgs.libxrender
    pkgs.libxfixes
    pkgs.libglvnd

    # Runtime binaries
    pkgs.grim
    pkgs.scrot
  ];

  env.LD_LIBRARY_PATH = lib.makeLibraryPath [
    pkgs.raylib
    pkgs.cmake
    pkgs.alsa-lib
    pkgs.libx11
    pkgs.libxrandr
    pkgs.libxi
    pkgs.libxcursor
    pkgs.libxinerama
    pkgs.libxkbcommon
    pkgs.libxext
    pkgs.libxrender
    pkgs.libxfixes
    pkgs.libglvnd
  ];

  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
