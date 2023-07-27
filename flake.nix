{
  description = "The CLI for managing my NixOS configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    devshell.url = "github:numtide/devshell";
    devshell.inputs.nixpkgs.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, crane, flake-parts, rust-overlay, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
    ];

    imports = [
      inputs.devshell.flakeModule
    ];

    perSystem = { config, pkgs, system, ... }:
      let
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        universe-cli = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          meta.mainProgram = "universe-cli";
        });
      in
      {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        packages.default = universe-cli;
        apps.default.program = config.packages.default;

        devshells.default.devshell = {
          # name = "${universe-cli.meta.mainProgram}-devshell";

          packagesFrom = [ universe-cli ];

          packages = with pkgs; [
            rustToolchain
          ];
        };
      };
  };
}
