{...}: rec {
	mkPackage = {stdenv, name, deps, src?null, dst?""}: stdenv.mkDerivation
	{
		name = name;
		dontConfigure = true;
		dontBuild = true;
		dontUnpack = true;
		buildInputs = deps;
		installPhase = let o = "$out/minecraft/"; in
		''
			mkdir -p "${o}"
			for dep in $buildInputs; do
				cp -rsn $dep/minecraft/* "${o}"
			done

			${if src != null then
			''
				mkdir -p "${o}/${dst}/"
				cp $src "${o}/${dst}/"
			''
			else ""}
		'';
	};
	mkMod          = args: mkPackage (args // {dst="mods";         });
	mkResourcePack = args: mkPackage (args // {dst="resourcepacks";});
	mkShaderPack   = args: mkPackage (args // {dst="shaderpacks";  });

	mkModrinthPkg = {
		stdenv,
		fetchurl,
		name,
		deps,
		projectid,
		versionid,
		filename,
		hash,
		dst,
	}: mkPackage {
		inherit stdenv name deps dst;
		src = fetchurl {
			url = "https://cdn.modrinth.com/data/${projectid}/versions/${versionid}/${filename}";
			hash = hash;
		};
	};
	mkModrinthMod          = args: mkModrinthPkg (args // {dst="mods";         });
	mkModrinthResourcePack = args: mkModrinthPkg (args // {dst="resourcepacks";});
	mkModrinthShaderPack   = args: mkModrinthPkg (args // {dst="shaderpacks";  });
}
