{
  outputs = { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      rec {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = toml.package.name;
          version = toml.package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = with pkgs; [
            raylib
          ];
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ packages.default ];
          packages = with pkgs; [
            clippy
            rust-analyzer
            rustfmt
          ];
        };
      });
}
