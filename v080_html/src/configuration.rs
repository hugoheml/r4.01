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

	#[arg(long, required = true, num_args = 1)]
	pub service: ServiceType,

	#[arg(short, long, required = false, num_args = 1, default_value = "3333")]
	pub port: u16
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

#[derive(Clone, Copy, ValueEnum)]
pub enum ServiceType {
	Stdio,
	Udp,
	Tcp,
	Web
}