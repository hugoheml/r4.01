use std::net::UdpSocket;

use clap::Parser;

const LOCALHOST: &str ="127.0.0.1";
const PONG: &str = "PONG\n";

#[derive(Debug, Parser)]
struct Parameters {
	port: u16
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let parameters = Parameters::parse();
	let stream = UdpSocket::bind(format!("{}:{}", LOCALHOST, parameters.port))?;
	loop {
		let mut buffer = [0; 1024];
		let (_, src) = stream.recv_from(&mut buffer)?;	
		stream.send_to(PONG.as_bytes(), src)?;
	}
}