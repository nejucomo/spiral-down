{
  description = "spiral-down: a 2D visual multi-event countdown display";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # Stable Rust toolchain with all developer components,
        # including rust-analyzer, matching rust-toolchain.toml.
        toolchain =
          with fenix.packages.${system};
          combine [
            stable.rustc
            stable.cargo
            stable.rustfmt
            stable.clippy
            stable.rust-analyzer
            stable.rust-src
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        src = craneLib.cleanCargoSource ./.;

        # Build-time tools
        nativeBuildInputs =
          with pkgs;
          [ pkg-config ] ++ lib.optional stdenv.isLinux makeWrapper;

        # Runtime dynamic libraries required by eframe (wgpu + winit) on Linux.
        runtimeLibs =
          with pkgs;
          lib.optionals stdenv.isLinux [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            vulkan-loader
          ];

        # Link-time + SDK dependencies (runtime libs + macOS frameworks).
        # darwin.apple_sdk / darwin.apple_sdk_11_0 was removed from nixpkgs;
        # frameworks are now available directly as top-level pkgs attributes.
        buildInputs =
          runtimeLibs
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (
            with pkgs;
            [
              AppKit
              CoreGraphics
              QuartzCore
              Metal
            ]
          );

        commonArgs = {
          inherit src nativeBuildInputs buildInputs;
          strictDeps = true;
          # Silence crane's "placeholder value" warnings: the root Cargo.toml is
          # a workspace manifest with no [package] section, so crane can't infer
          # name/version from it.  Supply them explicitly here instead.
          pname = "spiral-down";
          version = "0.1.0";
        };

        # Pre-build dependencies once so incremental rebuilds are fast.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        spiral-down-pkg = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            # Wrap the installed binary so eframe can locate GPU/display
            # libraries at runtime on NixOS without a global /usr/lib.
            postInstall = pkgs.lib.optionalString pkgs.stdenv.isLinux ''
              wrapProgram $out/bin/spiral-down \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath runtimeLibs}
            '';
          }
        );
      in
      {
        packages = {
          default = spiral-down-pkg;
          spiral-down = spiral-down-pkg;
        };

        devShells.default = pkgs.mkShell {
          # The toolchain brings cargo, rustc, rust-analyzer, rustfmt, clippy,
          # and rust-src.  pkg-config + buildInputs cover native system libs so
          # that `cargo build` succeeds and eframe can run inside the shell.
          packages = [ toolchain pkgs.pkg-config ] ++ buildInputs;

          # Let eframe/wgpu find GPU and display libraries at runtime.
          LD_LIBRARY_PATH = pkgs.lib.optionalString pkgs.stdenv.isLinux (
            pkgs.lib.makeLibraryPath runtimeLibs
          );
        };
      }
    );
}
