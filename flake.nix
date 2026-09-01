{
  description = "Zooma flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    version = cargoToml.package.version;

    buildDeps = [
      pkgs.cargo
      pkgs.rustc
      pkgs.rustfmt
      pkgs.clippy
      pkgs.rust-analyzer
      pkgs.cmake
      pkgs.rustPlatform.bindgenHook
      pkgs.llvmPackages.libclang
      pkgs.makeWrapper
    ];
    runtimeDeps = [
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
      pkgs.scrot
      pkgs.grim
    ];
  in
  {
    # Dev shell
    devShells.${system}.default = pkgs.mkShell {
      nativeBuildInputs = buildDeps;
      buildInputs = runtimeDeps;

      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeDeps}"
      '';
    };


    # Nix package
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = cargoToml.package.name;
      inherit version;
      src     = ./.;
      cargoLock.lockFile = ./Cargo.lock;

      nativeBuildInputs = buildDeps;
      buildInputs = runtimeDeps;

      postFixup = ''
        wrapProgram $out/bin/zooma \
        --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.grim pkgs.scrot ]} \
        --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath [ pkgs.libGL pkgs.glfw pkgs.raylib pkgs.libX11 pkgs.libXcursor pkgs.libXi pkgs.libXinerama pkgs.libxkbcommon ]}
      '';
    };
  };
}

