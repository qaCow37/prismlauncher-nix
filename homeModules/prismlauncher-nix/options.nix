{prism-lib, inputs, ...}:{lib, pkgs, ...}:
{
	options.programs.prismlauncher-nix = {
		enable = lib.mkEnableOption "Enable prismlauncher-nix";

		package = lib.mkOption {
			type = lib.types.package;
			default = inputs.prismlauncher.packages.${pkgs.stdenv.hostPlatform.system}.default;
			description = "The Prism Launcher package to use";
		};

		instances = lib.mkOption {
			type = lib.types.attrsOf prism-lib.types.instance;
			default = {};
			description = "All minecraft instances for prism launcher to be configured";
		};
	};
}
