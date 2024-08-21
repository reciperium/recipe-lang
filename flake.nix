{
  description = "Recipe management cli";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    flake-parts = {
      inputs = {
        nixpkgs-lib.follows = "nixpkgs";
      };
    };
  };

  outputs = inputs@{ flake-parts, nci, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        # To import a flake module
        # 1. Add foo to inputs
        # 2. Add foo as a parameter to the outputs function
        # 3. Add here: foo.flakeModule
        nci.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        nci.projects."recipe-lang" = {
          path = ./.;
          # export all crates (packages and devshell) in flake outputs
          # alternatively you can access the outputs and export them yourself
          export = true;
        };

        # configure crates
        nci.crates = {
          "recp" = { };
          "recipe-parser" = { };
        };

        # export the project devshell as the default devshell
        # devShells.default = config.nci.outputs."recipe-lang".devShell;
        devShells.default = pkgs.mkShell {
          inputsFrom = [ config.nci.outputs."recipe-lang".devShell ];
          packages = [ pkgs.cargo-dist ];
        };
        # export the release package of the crate as default package
        packages.default = config.nci.outputs."recp".packages.release;
        packages.recp = config.nci.outputs."recp".packages.release;
      };
    };
}
