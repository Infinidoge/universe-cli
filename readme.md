# `universe-cli`: An Opinioned Management CLI for Flake-based NixOS

Originally based on my scripts for `bud`, `universe-cli` replaces all that I used `bud` for, and more.
This tool is specifically designed for my needs, and to complement [my configuration, my universe](https://github.com/Infinidoge/universe).
If you find it useful too, great!
However please keep in mind that I make no guarantees on stability, and things can change with my whims.

It is designed to be aliased or put in a script so that the flake root is provided.
If one isn't provided, it will check for a `flake.nix` in `/etc/nixos`, and if present, it will assume it is the flake root.
(As a note, it _will_ check for a `flake.nix` in a provided root, just for sanity checking.)

# TODO

- [ ] Improve readme
- [ ] Add more metadata to [Cargo.toml](./Cargo.toml)
- [ ] Add help text to subcommands
- [ ] Create something to make use of `universe-cli cd`
- [ ] Add NixOS/Home Manager module for automatic installation (including `universe-cli cd` setup)
