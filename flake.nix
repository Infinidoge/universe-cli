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
