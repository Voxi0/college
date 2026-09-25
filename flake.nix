{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} {
    systems = inputs.nixpkgs.lib.platforms.all;
    perSystem = {system, pkgs, ...}: {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [inputs.fenix.overlays.default];
      };

      devShells.default = pkgs.mkShellNoCC {
        nativeBuildInputs = with pkgs; [
          clang
          rust-analyzer-nightly
          (fenix.complete.withComponents ["cargo" "rustc" "rust-src" "rustfmt"])
        ];
      };
    };
  };
}
