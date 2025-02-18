use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
pub struct Configuration {
	#[arg(short, long, required = true, num_args = 1..)]
	pub candidates: Vec<String>,

	#[arg(short, long, required = true, num_args = 1)]
	pub storage: StorageType,

	#[arg(short, long, required = true, num_args = 1)]
	pub language: LexiconType,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum StorageType {
	File,
	Memory
}

#[derive(Clone, Copy, ValueEnum)]
pub enum LexiconType {
	Fr,
	En
}