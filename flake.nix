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
          cargo = pkgs.rust-bin.nightly.latest.default;
          rustc = pkgs.rust-bin.nightly.latest.default;
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
          useFetchCargoVendor = true;
          cargoHash = "sha256-RCqjrrgHUMEf0rK1XH352plPoS8jKoo6lp7J9RakX/o=";

          doCheck = false;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          nativeBuildInputs = [
            cargo-afl
            llvmPackages.libllvm
            rust-bin.nightly.latest.default
          ];
        };
      }
    );
}
