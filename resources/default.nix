args:
let
	options = import ./options.nix args;
in
{
	options = options.resource;
}
