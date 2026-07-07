{
	inputs = {
		prismlauncher = {
			url = "github:PrismLauncher/PrismLauncher";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		home-manager =  {
			url = "github:nix-community/home-manager";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	};
	outputs = {self, nixpkgs, prismlauncher, home-manager, ...}@inputs:
	let
		lib = nixpkgs.lib // home-manager.lib;
			
		supportedSystems = builtins.attrNames prismlauncher.packages;
		forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
	in
	{
		homeModules = {
			prismlauncher-nix = import ./homeModules/prismlauncher-nix
				{
					inputs = inputs;
					prism-lib = self.lib;
				};
		};
		resources = forAllSystems (sys: let pkgs = nixpkgs.legacyPackages.${sys}; in
			import ./resources {
				pkgs = pkgs;
				lib = lib;
				stdenv = pkgs.stdenv;
				system = sys;
			}
		);
		components = import ./components {inherit lib;};
		lib        = import ./lib        {inherit lib;};
	};
}
