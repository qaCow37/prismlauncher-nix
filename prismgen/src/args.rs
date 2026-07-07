use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,

	pub output: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	Components,
}
