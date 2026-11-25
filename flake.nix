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
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "ush";
      version = packageVersion;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      cargoBuildFlags = [ "-p" "ush" "-p" "ush_lsp" ];
      nativeCheckInputs = [ pkgs.python3 pkgs.git pkgs.bash pkgs.zsh ];
      preCheck = "export HOME=$TMPDIR/ush-test-home; mkdir -p $HOME";
      cargoTestFlags = [ "--workspace" ];
      checkType = "debug";
    };
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [ cargo rustc rustfmt clippy pkg-config nodejs shellcheck python3 jq bash zsh gh tynix.packages.${system}.default ];
    };
  });
}