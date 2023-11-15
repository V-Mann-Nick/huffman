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
    toolchain = rust.fromToolchainFile {
      file = ./toolchain.toml;
      sha256 = "sha256-rLP8+fTxnPHoR96ZJiCa/5Ans1OojI7MLsmSqR2ip8o=";
    };
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
