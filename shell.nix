{ pkgs ? import <nixpkgs> {} }:
let
  rust-stable = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
in pkgs.mkShell {
  nix_shell_name = "wasmer";
  buildInputs = [
    rust-stable
    pkgs.openssl
    pkgs.pkg-config
  ];
}
