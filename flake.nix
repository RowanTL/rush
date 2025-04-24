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
            rust-bin.beta.latest.default
            # rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
          ];
          packages = with pkgs; [
            rust-bin.beta.latest.clippy
            rust-bin.beta.latest.rust-analyzer
            rust-bin.beta.latest.rustfmt
            # rust-bin.beta.latest.rustup
            rust-bin.beta.rust-src
            jetbrains.rust-rover
          ];
          shellHook = ''
            export SHELL=${pkgs.lib.getExe pkgs.bashInteractive}
            export RUST_SRC=${pkgs.rust-bin.beta.latest.rust-src}
          '';
        };
      }
    );
}
