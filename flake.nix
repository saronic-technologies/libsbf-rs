{
  description = "A devShell example";

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
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };

        cargo-afl = rustPlatform.buildRustPackage rec {
          pname = "cargo-afl";
          version = "0.15.17";
          src = pkgs.fetchFromGitHub {
            owner = "rust-fuzz";
            repo = "afl.rs";
            rev = "v${version}";
            hash = "sha256-JMQYa8UL+QAo8D8T13BEvrrhy4c/fiSozFDTPdGS5ME=";
          };
          cargoLock.lockFile = "${src}/Cargo.lock";

          doCheck = false;
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        summary = rustPlatform.buildRustPackage {
          pname = "summary";
          inherit (cargoToml.package) version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "--example" "summary" ];
          doCheck = false;
          # cargo puts the example under target/**/release/examples/, which the
          # default install hook skips because it is not a declared bin.
          installPhase = ''
            runHook preInstall
            bin=$(find target -type f -name summary -path '*/release/examples/*' | head -n1)
            install -Dm755 "$bin" "$out/bin/summary"
            runHook postInstall
          '';
        };
      in
      with pkgs;
      {
        packages.summary = summary;

        apps.summary = {
          type = "app";
          program = "${summary}/bin/summary";
        };

        devShells.default = mkShell {
          nativeBuildInputs = [
            cargo-afl
            llvmPackages.libllvm
            rust-bin.stable.latest.default
            valgrind
          ];
          env = {
            RUST_LOG = "info";
          };
        };
      }
    );
}
