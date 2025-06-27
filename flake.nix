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
          baseCraneLib =
            cs:
            (inputs.crane.mkLib pkgs).overrideToolchain (
              inputs'.fenix.packages.combine (
                with inputs'.fenix.packages;
                [
                  minimal.rustc
                  minimal.cargo
                  targets.wasm32-wasip2.latest.rust-std
                ]
                ++ cs
              )
            );
          craneLib = baseCraneLib [ ];
          craneLibDev = baseCraneLib (
            with inputs'.fenix.packages.complete;
            [
              clippy
              rustfmt
              rust-analyzer
              rust-src
            ]
          );
        in
        {

          packages.default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
            doCheck = false;
            CARGO_BUILD_TARGET = "wasm32-wasip2";
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              wasm-pack
            ];

            inputsFrom = [ (craneLibDev.devShell { }) ];
          };
        };
    };
}
