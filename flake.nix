{
  description = "ush - ubugeeei shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    tynix.url = "github:ubugeeei-prod/tynix/v2.0.0";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, tynix }: flake-utils.lib.eachDefaultSystem (system: let
    packageVersion = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;
    pkgs = import nixpkgs {
      inherit system;
    };
    kacheVersion = "2.3.1";
    kacheArtifacts = {
      aarch64-darwin = {
        triple = "aarch64-apple-darwin";
        hash = "sha256-viLTAsQCOLOnX5aVrRgCLlo64ygwDCdKE4bEa6DmuNE=";
      };
      x86_64-darwin = {
        triple = "x86_64-apple-darwin";
        hash = "sha256-b7pg5tQ8zfRKcwRQQolUiBb3IByekT/tBoeREd01zL8=";
      };
      aarch64-linux = {
        triple = "aarch64-unknown-linux-musl";
        hash = "sha256-/dOmWIRM5Xw5HSMHPSUOVKH8Fk8pFzbI0Yd1ZEQ4R18=";
      };
      x86_64-linux = {
        triple = "x86_64-unknown-linux-musl";
        hash = "sha256-bx/z+fr1qbsOOFAcW/6atbSpgINPHINdCa1bD9+bark=";
      };
    };
    kache = pkgs.stdenvNoCC.mkDerivation {
      pname = "kache";
      version = kacheVersion;
      src = pkgs.fetchurl {
        url = "https://github.com/kunobi-ninja/kache/releases/download/v${kacheVersion}/kache-${kacheArtifacts.${system}.triple}.tar.gz";
        hash = kacheArtifacts.${system}.hash;
      };
      sourceRoot = ".";
      installPhase = ''
            runHook preInstall
            install -Dm755 kache $out/bin/kache
            install -Dm755 kache-engine $out/bin/kache-engine
            runHook postInstall
          '';
      meta = {
        description = "Compiler cache for Rust, C/C++ and CUDA";
        homepage = "https://github.com/kunobi-ninja/kache";
        license = pkgs.lib.licenses.asl20;
        mainProgram = "kache";
        platforms = builtins.attrNames kacheArtifacts;
      };
    };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "ush";
      version = packageVersion;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      cargoBuildFlags = [ "-p" "ush" "-p" "ush_lsp" ];
      nativeCheckInputs = [ pkgs.python3 pkgs.git pkgs.bash pkgs.zsh pkgs.curl pkgs.file pkgs.jq pkgs.perl pkgs.unixtools.ps ];
      preCheck = "export HOME=$TMPDIR/ush-test-home; mkdir -p $HOME";
      cargoTestFlags = [ "--workspace" ];
      checkType = "debug";
    };
    packages.kache = kache;
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [ cargo rustc rustfmt clippy kache pkg-config nodejs shellcheck python3 jq curl file git perl unixtools.ps bash zsh gh tynix.packages.${system}.default ];
      shellHook = ''
            ush_checkout=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
            if [ -x "$ush_checkout/scripts/launchers/rustc.sh" ]; then
              case "''${RUSTC_WRAPPER:-}" in
                "" | */scripts/launchers/rustc.sh) export RUSTC_WRAPPER="$ush_checkout/scripts/launchers/rustc.sh" ;;
              esac
            fi
            unset ush_checkout
            export USH_KACHE="''${USH_KACHE-${kache}/bin/kache}"
            export USH_RUSTC_FRONTEND_JOBS="''${USH_RUSTC_FRONTEND_JOBS:-8}"
          '';
    };
  });
}