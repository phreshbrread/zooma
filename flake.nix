{
  description        = "Zooma flake";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    # Nix package
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname   = "zooma";
      version = "1.0.0";
      src     = ./.;
      cargoLock.lockFile = ./Cargo.lock;

      nativeBuildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        cmake
        rustPlatform.bindgenHook
        llvmPackages.libclang
        makeWrapper
      ];

      buildInputs = with pkgs; [
        raylib
        cmake
        libx11
        libxcb
        libxau
        libxdmcp
        libxinerama
        libxcursor
        libxi
        clang
        wayland
        libGL
        glfw
        scrot
        grim
      ];

      postFixup = ''
        wrapProgram $out/bin/zooma \
        --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.grim pkgs.scrot ]} \
        --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath [ pkgs.libGL pkgs.glfw pkgs.raylib pkgs.libX11 pkgs.libXcursor pkgs.libXi pkgs.libXinerama pkgs.libxkbcommon ]}
      '';
    };

    # Dev shell
    devShells.${system}.default = pkgs.mkShell {
      inputsFrom = [ self.packages.${system}.default ];
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        cmake
        rustPlatform.bindgenHook
        llvmPackages.libclang
        raylib
        cmake
        libx11
        libxcb
        libxau
        libxdmcp
        libxinerama
        libxcursor
        libxi
        clang
        wayland
        libGL
        glfw
        scrot
        grim
      ];

      BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";

      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.libGL}/lib:${pkgs.libx11}/lib:${pkgs.libxrandr}/lib:${pkgs.libxinerama}/lib:${pkgs.libxcursor}/lib:${pkgs.libxi}/lib:$LD_LIBRARY_PATH
      '';
    };
  };
}

