{
  description = "Protontweaks";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }:
    let
      inherit (nixpkgs) lib legacyPackages;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      # Devshell for bootstrapping; acessible via 'nix develop' or 'nix-shell' (legacy)
      devShells = forAllSystems (system:
        let pkgs = legacyPackages.${system};
        in import ./shell.nix { inherit pkgs; }
      );
    };
}
