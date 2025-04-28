{
  description = "Simple Rush nix flake";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            rust-bin.beta.latest.complete
          ];
          packages = with pkgs; [
            gdb
            bacon
            python313Packages.pygments # for personal gdb-dashboard use
          ];
          shellHook = ''
            export SHELL=${pkgs.lib.getExe pkgs.bashInteractive}
          '';
        };
        devShells.nightly = with pkgs; mkShell {
          buildInputs = [
            # rust-bin.beta.latest.default
            # rust-bin.selectLatestNightlyWith (toolchain: toolchain.complete)
            rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          ];
          packages = with pkgs; [
            # rust-bin.beta.latest.clippy
            # rust-bin.beta.latest.rust-analyzer
            # rust-bin.beta.latest.rustfmt
            # rust-bin.beta.latest.rust-std
            rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "clippy" "rust-analyzer" "rustfmt" "rust-std" ];
              # targets = [ "arm-unknown-linux-gnueabihf" ];
            })
            gdb
            # gdb-dashboard
            bacon
            python313Packages.pygments # for personal gdb-dashboard use
          ];
          shellHook = ''
            export SHELL=${pkgs.lib.getExe pkgs.bashInteractive}
          '';
        };
      }
    );
}
