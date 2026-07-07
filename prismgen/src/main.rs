mod args;
mod util;

use tokio::try_join;
use util::{
	Result,
	Client,
	PrismResponse,
	Nix,
	nix,
};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
	let args = args::Args::parse();

	let nix = Nix::new(std::path::Path::new(&args.output));

	match args.command {
		args::Command::Components => {
			let client = Client::new()?;

			let (mc, inter, fabric, quilt, neoforge, forge, liteloader) = try_join!(
				PrismResponse::new(&client, "net.minecraft"             ),
				PrismResponse::new(&client, "net.fabricmc.intermediary" ),
				PrismResponse::new(&client, "net.fabricmc.fabric-loader"),
				PrismResponse::new(&client, "org.quiltmc.quilt-loader"  ),
				PrismResponse::new(&client, "net.neoforged"             ),
				PrismResponse::new(&client, "net.minecraftforge"        ),
				PrismResponse::new(&client, "com.mumfrey.liteloader"    ),
			)?;
			let fabricv = fabric.get_latest_version().expect("A fabric version should always be available");
			let quiltv  = quilt .get_latest_version().expect("A quilt version should always be available");

			for vers in mc.get_all_versions() {
				let interv      = inter     .get_version_for_require("net.minecraft", vers);
				let forgev      = forge     .get_version_for_require("net.minecraft", vers);
				let neoforgev   = neoforge  .get_version_for_require("net.minecraft", vers);
				let liteloaderv = liteloader.get_version_for_require("net.minecraft", vers);

				nix.write_components(vers, nix::Components {
					fabric    : interv.map(|_| fabricv),
					quilt     : interv.map(|_| quiltv),
					forge     : forgev,
					neoforge  : neoforgev,
					liteloader: liteloaderv,
				})?;
			}
			nix.write_version_imports(mc.get_all_versions())?;
		}
	}

	Ok(())
}
