{
  description = "Advent of code 2020";

  inputs.nixpkgs.url = "nixpkgs/nixos-20.09";
  inputs.import-cargo.url = "github:edolstra/import-cargo";

  outputs = { self, nixpkgs, import-cargo }:
    let
      systems = [ "x86_64-linux" ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

      # Memoize nixpkgs for different platforms for efficiency.
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });

      buildPackage = { isShell, system }: with nixpkgsFor.${system}; stdenv.mkDerivation {
        name = "aoc${lib.substring 0 8 self.lastModifiedDate}-${self.shortRev or "dirty"}";

        buildInputs =
          [
            rustc
            cargo
          ] ++ (if isShell then [
            clippy
            rustfmt
          ] else [
            (import-cargo.builders.importCargo {
              lockFile = ./Cargo.lock;
              inherit pkgs;
            }).cargoHome
          ]);

        src = if isShell then null else self;

        buildPhase = "cargo build --release --frozen --offline";

        doCheck = true;

        checkPhase = "cargo test --release --frozen --offline";

        installPhase =
          ''
            mkdir -p $out
            cargo install --frozen --offline --path . --root $out
            rm $out/.crates.toml
          '';
      };

    in
    {

      defaultPackage = forAllSystems (system: buildPackage { inherit system; isShell = false; });

      devShell = forAllSystems (system: buildPackage { inherit system; isShell = true; });

      checks = forAllSystems (system:
        let

          pkgs = import nixpkgs { inherit system; };

        in
        {
          build = self.defaultPackage.${system};
        }
      );
    };
}
