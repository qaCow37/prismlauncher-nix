use std::{
	io::{Write, BufWriter},
	path::Path,
	fs::File,
	fmt::Display,
};
use super::Result;

pub struct Nix<'a> {
	root: &'a Path,
}

pub struct Components<'a> {
	pub fabric    : Option<&'a str>,
	pub quilt     : Option<&'a str>,
	pub forge     : Option<&'a str>,
	pub neoforge  : Option<&'a str>,
	pub liteloader: Option<&'a str>,
}

impl<'a> Display for Components<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use std::write;

		if let Some(v) = self.fabric {
			write!(f, r#"fabric={{uid="net.fabricmc.fabric-loader";version="{v}";}};"#)?;
		}
		if let Some(v) = self.quilt {
			write!(f, r#"quilt={{uid="org.quiltmc.quilt-loader";version="{v}";}};"#)?;
		}
		if let Some(v) = self.forge {
			write!(f, r#"forge={{uid="net.minecraftforge";version="{v}";}};"#)?;
		}
		if let Some(v) = self.neoforge {
			write!(f, r#"neoforge={{uid="net.neoforged";version="{v}";}};"#)?;
		}
		if let Some(v) = self.liteloader {
			write!(f, r#"liteloader={{uid="com.mumfrey.liteloader";version="{v}";}};"#)?;
		}
		Ok(())
	}
}

impl<'a> Nix<'a> {
	pub fn new(root: &'a Path) -> Self {
		Self {
			root
		}
	}

	pub fn write_components<'b>(
		&self,
		vers: &str,
		comps: Components<'b>,
	) -> Result<()> {
		std::fs::create_dir_all(self.root)?;
		let mut file = {
			let mut path = self.root.to_path_buf();
			path.push(vers);
			path.add_extension("nix");
			File::create(path)
		}?;
		writeln!(
			file,
			r#"{{game={{important=true;uid="net.minecraft";version="{vers}";}};{comps}}}"#
		)?;

		Ok(())
	}
	pub fn write_version_imports<'b, It>(&self, versions: It) -> Result<()>
	where
		It: Iterator<Item = &'b str>
	{
		std::fs::create_dir_all(self.root)?;
		let file = {
			let mut path = self.root.to_path_buf();
			path.push("default.nix");
			File::create(path)
		}?;
		let mut buf = BufWriter::new(file);

		write!(buf, "{{")?;

		for v in versions {
			write!(buf, r#""{v}"=import ./{v}.nix;"#)?;
		}

		write!(buf, "}}")?;
		buf.flush()?;

		Ok(())
	}
}
