{lib, ...}:
let
	resource = rec {
		activation = lib.types.submodule {
			options = {
				name = lib.mkOption {
					type = lib.types.str;
					description = "The name of the activation";
				};
				entry = lib.mkOption {
					type = lib.mkOptionType {
						name = "home-manager dag-entry";
						check = v: lib.hm.dag.isEntry v;
					};
					description = "The entry of the activation";
				};
			};
		};
		activation-fn = lib.types.functionTo activation;
		package = lib.types.package;

		any-activation = lib.types.oneOf [
			activation-fn
			activation
		];
		call-activation = act: dst: (
			if activation-fn.check act
				then act dst
				else act
		);

		any = lib.types.oneOf [
			any-activation
			package
		];
	};

	instance = lib.types.submodule {
		options = {
			config = {
				name = lib.mkOption {
					type = lib.types.str;
				};
			};
			components = lib.mkOption {
				type = lib.types.listOf (lib.types.mkOptionType {
					name = "any-json-value-except-null";
					description = "any JSON value except null";
					check = v: v != null && (lib.types.json.check v);
					merge = lib.types.anything.merge;
				});
				default = [];
				description = "All the components to install into this instance";
			};
			resources = lib.mkOption {
				type = lib.types.listOf resource.any;
				default = [];
				description = "All the resources to install into the minecraft folder";
			};
		};
	};
in
{
	inherit resource;
	inherit instance;
}
