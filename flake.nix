{
  description = "A user-friendly TUI client for Matrix written in Rust.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        nightly = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            nightly
            cargo
            cargo-edit
            cargo-watch
            rustfmt
            clippy
            openssl
            pkg-config
            cmake
          ];

          RUST_SRC_PATH = "${nightly}/lib/rustlib/src/rust/library";
          # RUST_BACKTRACE = 1;
        };
      }
    );
}
