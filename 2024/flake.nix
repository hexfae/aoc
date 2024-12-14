{
  description = "Advent of Code Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    nix-filter,
    crane,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain (p:
          p.rust-bin.nightly.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "rustc-codegen-cranelift-preview"
            ];
          });
        crateNameFromCargoToml = craneLib.crateNameFromCargoToml {cargoToml = ./Cargo.toml;};
        pkgDef = {
          inherit (crateNameFromCargoToml) pname version;
          src = nix-filter.lib.filter {
            root = ./.;
            include = [
              ./src
              ./Cargo.toml
              ./Cargo.lock
            ];
          };

          nativeBuildInputs = with pkgs; [
            mold
            clang
            pkg-config
          ];

          buildInputs = [];
        };

        cargoArtifacts = craneLib.buildDepsOnly pkgDef;
        aoc2024 = craneLib.buildPackage (pkgDef
          // {
            inherit cargoArtifacts;
          });
      in {
        checks = {
          inherit aoc2024;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = aoc2024;
        };
        packages.default = aoc2024;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};
          shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath pkgDef.buildInputs}";'';
        };
      }
    );
}