{...}: rec {
	mkPack = {stdenv, name, packages}: stdenv.mkDerivation
	{
		name = name;
		dontConfigure = true;
		dontBuild = true;
		dontUnpack = true;
		buildInputs = packages;
		installPhase = ''
			mkdir -p "$out/minecraft/"
			for dep in $buildInputs; do
				cp -rsn $dep/minecraft/* "$out/minecraft/"
			done
		'';
	};

	mkFileResource = {stdenv, name, src, dst?""}: stdenv.mkDerivation
	{
		name = name;
		dontConfigure = true;
		dontBuild = true;
		dontUnpack = true;
		src = src;
		installPhase = let o = "$out/minecraft/${dst}/"; in
		''
			mkdir -p "${o}"
			cp "$src" "${o}"
		'';
	};
	mkMod          = args: mkFileResource (args // {dst="mods";         });
	mkResourcePack = args: mkFileResource (args // {dst="resourcepacks";});
	mkShaderPack   = args: mkFileResource (args // {dst="shaderpacks";  });

	mkModrinthPkg = {
		stdenv,
		fetchurl,
		name,
		projectid,
		versionid,
		filename,
		hash,
		dst,
	}: mkFileResource {
		inherit stdenv name dst;
		src = fetchurl {
			url = "https://cdn.modrinth.com/data/${projectid}/versions/${versionid}/${filename}";
			hash = hash;
		};
	};
	mkModrinthMod          = args: mkModrinthPkg (args // {dst="mods";         });
	mkModrinthResourcePack = args: mkModrinthPkg (args // {dst="resourcepacks";});
	mkModrinthShaderPack   = args: mkModrinthPkg (args // {dst="shaderpacks";  });
}
