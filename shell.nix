let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  mkShell {
    buildInputs = [
      nixpkgs.rustc
      nixpkgs.cargo
      nixpkgs.rustfmt
      nixpkgs.clippy
      nixpkgs.rust-analyzer
      nixpkgs.libiconv
      nixpkgs.llvmPackages.lldb
      nixpkgs.darwin.apple_sdk.frameworks.Security
    ];
  }
