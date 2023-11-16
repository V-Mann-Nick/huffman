{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    fenix,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    rust = fenix.packages.${system};
    toolchain = rust.stable.withComponents [
      "rustc"
      "rust-std"
      "cargo"
      "rust-docs"
      "rustfmt"
      "clippy"
      "rust-src"
      "rust-analyzer"
    ];
    rustPlatform = pkgs.makeRustPlatform {
      cargo = toolchain;
      rustc = toolchain;
    };
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    packages.${system}.default = rustPlatform.buildRustPackage {
      pname = cargoToml.package.name;
      version = cargoToml.package.version;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [toolchain];
      RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
    };
    formatter.${system} = pkgs.alejandra;
  };
}
