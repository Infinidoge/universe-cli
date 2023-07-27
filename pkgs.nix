{ inputs, ... }:
let
  mkPackage = pkgs:
    let
      rustToolchain = inputs.rust-overlay.packages.${pkgs.system}.default.override {
        extensions = [ "rust-src" ];
      };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      commonArgs = {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in
    craneLib.buildPackage (commonArgs // {
      inherit cargoArtifacts;
      meta.mainProgram = "universe-cli";
    });
in
{
  flake.overlays.default = final: prev: {
    universe-cli = mkPackage final;
  };

  perSystem = { config, pkgs, system, ... }: {
    packages.default = mkPackage pkgs;
  };
}
