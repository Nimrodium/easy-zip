{
  pkgs ? import <nixpkgs> { },
}:
let
  lib = pkgs.lib;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "sticky";
  version = "1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
}
