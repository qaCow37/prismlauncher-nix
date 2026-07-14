use clap::{
	Parser,
	Args,
};
use crate::error::Result;

#[derive(Parser, Debug)]
pub enum Cli {
	Components(CmdComponents),
}

#[derive(Args, Debug)]
pub struct CmdComponents {
	#[arg(long, default_value="github:qacow37/prismlauncher-nix/prismgen")]
	client_agent: String,

	output: String,
}

impl CmdComponents {
	pub async fn run(self) -> Result<()> {
		// An absolute mess, I dont wanna fix this

		use reqwest::Client;
		use std::{
			path::Path,
			fs::File,
			io::{Write, BufWriter},
			sync::Arc,
			write,
		};
		use crate::prism::PrismIndex;
		use crate::nix;
		use tokio::{
			task::JoinSet,
			sync::Semaphore,
		};

		#[derive(serde::Serialize)]
		struct Component {
			important: bool,
			uid: &'static str,
			version: String,
		}
		#[derive(serde::Serialize)]
		struct Version {
			game    : Component,
			fabric  : Option<Component>,
			quilt   : Option<Component>,
			forge   : Option<Component>,
			neoforge: Option<Component>,
		}

		let opath = Path::new(&self.output);
		std::fs::create_dir_all(opath)?;
		let default_file = {
			let mut path = opath.to_path_buf();
			path.push("default");
			path.set_extension("nix");
			File::create(path)
		}?;
		let mut default_filebuf = BufWriter::new(default_file);
		default_filebuf.write_all(b"{\n")?;

		let client = Client::builder()
			.user_agent(self.client_agent)
			.https_only(true) // prismlauncher only has https API
			.build()?;
		let index = PrismIndex::new(&client).await?;

		let mcpkgs = index
			.get_pkgs("net.minecraft")
			.expect("net.minecraft should always be available");

		let mut tasks = JoinSet::new();
		let tasks_limit = Arc::new(Semaphore::new(64));

		for pkg in mcpkgs {
			let vfabric     = index.get_latest_pkg_for_mc("net.fabricmc.fabric-loader", pkg.get_version());
			let vquilt      = index.get_latest_pkg_for_mc("org.quiltmc.quilt-loader"  , pkg.get_version());
			let vforge      = index.get_latest_pkg_for_mc("net.minecraftforge"        , pkg.get_version());
			let vneoforge   = index.get_latest_pkg_for_mc("net.neoforged"             , pkg.get_version());

			let version = Version {
				game: Component {
					important: true,
					uid: "net.minecraft",
					version: pkg.get_version().to_string(),
				},
				fabric: vfabric.map(|p| Component {
					important: false,
					uid: "net.fabricmc.fabric-loader",
					version: p.get_version().to_string(),
				}),
				quilt: vquilt.map(|p| Component {
					important: false,
					uid: "org.quilmtc.quilt-loader",
					version: p.get_version().to_string(),
				}),
				forge: vforge.map(|p| Component {
					important: false,
					uid: "net.minecraftforge",
					version: p.get_version().to_string(),
				}),
				neoforge: vneoforge.map(|p| Component {
					important: false,
					uid: "net.neoforged",
					version: p.get_version().to_string(),
				}),
			};
			let path = {
				let mut path = opath.to_path_buf();
				path.push(pkg.get_version());
				path.add_extension("nix");
				path
			};

			let sem = tasks_limit.clone();
			tasks.spawn(async move {
				let _permit = sem.acquire().await.unwrap();

				let file = File::create(path)?;
				let buf = BufWriter::new(file);
				nix::to_writer(buf, &version)?;
				Result::<_>::Ok(())
			});
			write!(default_filebuf, "  \"{}\" = import ./{}.nix;\n",
				pkg.get_version(),
				pkg.get_version(),
			)?;
		}
		default_filebuf.write_all(b"}")?;

		// wait till all tasks are finished
		while let Some(r) = tasks.join_next().await {r??;}

		Ok(())
	}
}

impl Cli {
	pub async fn run(self) -> Result<()> {
		match self {
			Self::Components(args) => args.run().await,
		}
	}
}
