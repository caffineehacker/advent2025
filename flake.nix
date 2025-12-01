{
  description = "Advent 2025 Env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          buildInputs = [
            gcc
            (rust-bin.stable.latest.default.override {
              extensions = [
                "cargo"
                "clippy"
                "rust-src"
                "rust-analyzer"
              ];
            })
            clippy
            vscodium
            aoc-cli
            z3
          ];

          nativeBuildInputs = [
            rustPlatform.bindgenHook
            pkg-config
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
