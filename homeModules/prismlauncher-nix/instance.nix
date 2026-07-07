{prism-lib, ...}: {lib, pkgs, ...}:
{
	build = name: dst: cfg:
	let
		resource = let rsc = cfg.resources; types = prism-lib.types.resource; in
		{
			activations = builtins.filter (v: v != null) (map (item:
				if types.any-activation.check item
					then types.call-activation
						item
						"${dst}/minecraft"
					else null
			) rsc);

			packages = builtins.filter (item:
				types.package.check
					item
			) rsc;
		};
	in
	{
		activation =
		let
			# transform list of activation scripts to list of activation scripts
			# with the name of the activation script prefixed with instance name
			resources = lib.mergeAttrsList (map (item:
				{
					"prismlauncher-nix.${name}.${item.name}" = item.entry;
				}
			) resource.activations);
		in
		{
			"prismlauncher-nix.${name}" =
			let
				mmc-pack = pkgs.writeText "prismlauncher-nix.${name}.mmc-pack" (
					lib.toJSON {
						components = cfg.components;
						formatVersion = 1;
					}
				);
				crudini = "${lib.getExe pkgs.crudini}";
			in lib.hm.dag.entryAfter ["linkGeneration"]
			''
				mkdir -p "${dst}"
				cp -f "${mmc-pack}" "${dst}/mmc-pack.json"
				touch "${dst}/instance.cfg"

				${crudini} --set "${dst}/instance.cfg" "General" "InstanceType" "OneSix"
				${crudini} --set "${dst}/instance.cfg" "General" "ConfigVersion" "1.3"
				${crudini} --set "${dst}/instance.cfg" "General" "name" "${cfg.config.name}"
			'';
		}
		// resources;

		file."${dst}/minecraft/" =
		let
			pkg = pkgs.callPackage prism-lib.pkgs.mkPack {
				name = "${name}-mcpack";
				packages = resource.packages;
			};
		in
		lib.mkIf ((builtins.length resource.packages) > 0)
		{
			source = "${pkg}/minecraft/";
			recursive = true;
		};
	};
}
