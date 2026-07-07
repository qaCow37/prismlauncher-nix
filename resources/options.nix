{lib, pkgs, ...}:
let
	grep = "${lib.getExe pkgs.gnugrep}";
	sed  = "${lib.getExe pkgs.gnused }";
in
{
	resource = opts: dst: let file = "${dst}/options.txt"; in
	{
		name = "options";
		entry = lib.hm.dag.entryAfter ["linkGeneration"]
		''
			set() {
				if ${grep} -q "^$1:" "${file}"; then
					${sed} -i "s@^$1:.*@$1:$2@" "${file}"
				else
					echo "$1:$2" >> "${file}"
				fi
			}
			${lib.concatMapAttrsStringSep "\n" (key: val:
				''
					set "${key}" "${lib.toJSON val}"
				''
			) opts}
		'';
	};
}
