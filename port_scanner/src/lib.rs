use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
pub struct Parameters {
	pub host: String,
	pub port_min: u16,
	pub port_max: u16,
	pub timeout: u64
}

pub async fn is_open(host: &str, port: u16, timeout: u64) -> bool {
	matches! (tokio::time::timeout(Duration::from_secs(timeout), tokio::net::TcpStream::connect(format!("{}:{}", host, port))).await, Ok(Ok(_)))
}