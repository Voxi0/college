{
  inputs = {
    # Nice flake framework or whatever
    flake-parts.url = "github:hercules-ci/flake-parts";

    # Nixpkgs software repository
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";

    # Provides nightly Rust toolchain
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} {
    systems = inputs.nixpkgs.lib.platforms.all;
    perSystem = {system, pkgs, ...}: {
      # Configure `nixpkgs` instance
      # The Fenix overlay provides the Rust nightly toolchain
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [inputs.fenix.overlays.default];
      };

      devShells.default = pkgs.mkShellNoCC {
        # This is required for `independent-mini-project` which is a proper GUI application
        # I'm running Niri which is a Wayland compositor so to build that, I need Wayland libraries
        # This environment variable tells the dynamic linker where to search for shared dynamic libraries
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          wayland # Wayland client
          libxkbcommon # For input
        ]);

        # Toolchain
        nativeBuildInputs = with pkgs; [
          # Toolchain
          clang
          rust-analyzer-nightly
          (fenix.complete.withComponents ["cargo" "rustc" "rust-src" "rustfmt"])
        ];
      };
    };
  };
}
