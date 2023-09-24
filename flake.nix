{
  description = "Telemetryd flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    {
      overlay = final: prev: {
        telemetryd = final.rustPlatform.buildRustPackage {
          pname = "telemetryd";
          version = "0.1.0";
          src = ./.;
          cargoBuildFlags = "--package telemetryd";
          cargoLock.lockFile = ./Cargo.lock;
        };
      };
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ self.overlay ]; };
      in {
        defaultPackage = self.packages."${system}".telemetryd;
        packages.default = self.packages."${system}".telemetryd;
        packages.telemetryd = pkgs.telemetryd;

        devShell = pkgs.mkShell {
          # TODO
          buildInputs = [ ];
        };
      });
}
