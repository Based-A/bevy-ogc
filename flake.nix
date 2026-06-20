{
  description = "Nix development environment for the bevy-ogc crate";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    devkitNix.url = "github:bandithedoge/devkitNix";
  };

  outputs =
    {
      self,
      nixpkgs,
      devkitNix,
    }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        config = {
          allowUnfree = true;
          allowUnfreePredicate = _: true;
        };
        overlays = [ devkitNix.overlays.default ];
      };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell.override { stdenv = pkgs.devkitNix.stdenvPPC; } {
        #strictDeps = true;

        buildInputs = with pkgs; [
          # Rust Tools
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          rustup
          udev
          clang
          llvmPackages.libclang
          pkg-config
          zstd
          rustPlatform.bindgenHook
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustPlatform.bindgenHook
        ];

        env = {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
          with pkgs;
          [
            udev
            clang
            pkg-config
            zstd
            rustPlatform.bindgenHook
            # any other libraries that need to be dynamically linked to
          ]
        );

        shellHook = ''
          export PATH="/home/$USER/.cargo/bin:$PATH"
          export PATH="${
            pkgs.lib.makeBinPath [ "${pkgs.devkitNix.devkitPPC}/opt/devkitpro/devkitPPC" ]
          }:$PATH"
        '';
      };

    };

}
