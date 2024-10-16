{
  description = "The CLI for managing my NixOS configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # Inputs
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";

    ### Cleanup ###
    # Follow nixpkgs
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    devshell.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
    ];

    imports = [
      inputs.devshell.flakeModule
      ./pkgs.nix
    ];

    perSystem = { config, ... }: {
      apps.default.program = config.packages.default;

      devshells.default.devshell = {
        name = config.packages.default.meta.mainProgram;
        motd = "";
        packagesFrom = [ config.packages.default ];
      };
    };
  };
}
