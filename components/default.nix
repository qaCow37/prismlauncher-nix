args:
let
	versions = import ./versions;
	lib = import ./lib.nix args;
in
{
	inherit lib;
	inherit (lib)
		fabric-with
		quilt-with
		forge-with
		neoforge-with
		liteloader-with;
} // versions
