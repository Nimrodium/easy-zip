{
  pkgs ? import <nixpkgs> { },
}:
let
  lib = pkgs.lib;
  manifest = (lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
}
