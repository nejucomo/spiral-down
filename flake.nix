{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildInputs = with pkgs; [ ];

        defaultPkg = pkgs.rustPlatform.buildRustPackage rec {
          pname = "spiral-down";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [ rustToolchain ];

          inherit buildInputs;

          meta = with pkgs.lib; {
            homepage = "https://github.com/nejucomo/${pname}";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      in
      {
        packages.default = defaultPkg;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ defaultPkg ];

          buildInputs = with pkgs; [
            cargo-shear
            pkg-config
            rust-analyzer
            rustToolchain
            taplo
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}"
          '';
        };
      }
    );
}
