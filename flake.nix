{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      naersk,
      flake-utils,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        naersk' = pkgs.callPackage naersk { };
      in
      rec {
        defaultPackage = naersk'.buildPackage { src = ./.; };
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
          ];
        };
      }

    );

  # let
  #   supportedSystems = [ "x86_64-linux" ];
  #   forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
  #   pkgsFor = nixpkgs.legacyPackages;
  # in
  # {
  #   packages = forAllSystems (system: {
  #     default = pkgsFor.${system}.callPackage ./. { };
  #   });
  # };
}
