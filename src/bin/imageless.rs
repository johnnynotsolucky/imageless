use clap::Parser;
use imageless::{process_file, Error, ImageOutputFormat, Operation};
use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::BufWriter, path::PathBuf};

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

	process_and_save(cli.file, cli.out, config.out_format, config.operations)?;

	Ok(())
}

fn process_and_save(
	in_path: PathBuf,
	out_path: PathBuf,
	out_format: ImageOutputFormat,
	operations: Vec<Operation>,
) -> Result<(), Error> {
	let image = process_file(in_path, operations)?;

	let out_file = File::create(out_path)?;
	let mut out_buf = BufWriter::new(out_file);
	image.write_to(&mut out_buf, out_format)?;

	Ok(())
}
