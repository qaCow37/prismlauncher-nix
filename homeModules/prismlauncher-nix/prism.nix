args1: {config, lib, pkgs, ...}@args2:
let
	launcher = import ./launcher.nix args1 args2;
	instance = import ./instance.nix args1 args2;
	
	cfg = config.programs.prismlauncher-nix;
	dir = "${config.xdg.dataHome}/PrismLauncher";
in
{
	config = lib.mkIf cfg.enable {
		home = lib.mergeAttrsList [
			(launcher.install cfg)
			(lib.concatMapAttrs (key: val:
				instance.build
					key                        # instance name
					"${dir}/instances/${key}"  # instance path
					val                        # instance config
			) cfg.instances)
		];
	};
}
