{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs-unstable,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs-unstable { inherit system; };
      in rec
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "vpr";
          version = "0.2.1";
          cargoLock.lockFile = ./Cargo.lock;
          src = ./.;
        };
        # Executed by `nix run`
        apps.default = utils.lib.mkApp {drv = packages.default;};
      }
    );
}
