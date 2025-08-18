{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{ systems, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;
      perSystem =
        {
          inputs',
          pkgs,
          ...
        }:

        let
          inherit (pkgs) lib;

          baseCraneLib =
            cs:
            (inputs.crane.mkLib pkgs).overrideToolchain (
              inputs'.fenix.packages.combine (
                with inputs'.fenix.packages;
                [
                  minimal.rustc
                  minimal.cargo
                  complete.clippy
                  complete.rustfmt
                  targets.wasm32-wasip2.latest.rust-std
                ]
                ++ cs
              )
            );
          craneLib = baseCraneLib [ ];
          craneLibDev = baseCraneLib (
            with inputs'.fenix.packages.complete;
            [
              rust-analyzer
              rust-src
            ]
          );

          cranePkg =
            o@{
              cargoTest ? false,
              ...
            }:
            craneLib.buildPackage (
              {
                src = lib.cleanSourceWith {
                  src = ./.;
                  filter =
                    path: type: (builtins.match ".*wit$" path != null) || (craneLib.filterCargoSources path type);
                };
                checkPhaseCargoCommand = "cargo fmt --check && cargo clippy ${
                  if cargoTest then "&& cargo test" else ""
                }";
                nativeBuildInputs = with pkgs; [ mold ];
              }
              // o
            );
        in
        {

          packages = {
            default = cranePkg {
              cargoTest = true;
            };

            wasm = cranePkg {
              CARGO_BUILD_TARGET = "wasm32-wasip2";
            };
          };

          devShells = {
            default = pkgs.mkShell {
              packages =
                let
                  unocss-language-server = pkgs.callPackage ./nix/unocss-language-server.nix { };
                in
                with pkgs;
                commonPkgs
                ++ [
                  taplo
                  mold
                  nodejs_latest
                  pnpm
                  dprint
                  eslint
                  vscode-langservers-extracted
                  vtsls
                  svelte-language-server
                  emmet-language-server
                  unocss-language-server
                  stylelint-lsp
                ];

              inputsFrom = [ (craneLibDev.devShell { }) ];
            };
          };
        };
    };
}
