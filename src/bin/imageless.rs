use clap::Parser;
use imageless::{process_file, ImageOutputFormat, Operation};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// File to process
	#[arg(short, long)]
	file: PathBuf,
	/// Output file
	#[arg(short, long)]
	out: PathBuf,
	/// Path to an Imageless config file
	#[arg(short, long)]
	config: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
	out_format: ImageOutputFormat,
	operations: Vec<Operation>,
}

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();
	let config_file = cli.config.canonicalize()?;
	let config: Config = toml::from_str(&fs::read_to_string(config_file)?)?;

	process_file(cli.file, cli.out, config.out_format, config.operations)?;

	Ok(())
}
