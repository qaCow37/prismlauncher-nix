mod cli;
mod nix;
mod prism;
mod error;
use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
	use clap::Parser;
	let cli = cli::Cli::parse();
	cli.run().await
}
