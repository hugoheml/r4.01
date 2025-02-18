use std::net::UdpSocket;

use clap::Parser;

const LOCALHOST: &str ="127.0.0.1";

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
		let (number_of_bytes, src) = stream.recv_from(&mut buffer)?;	

		let message = std::str::from_utf8(&buffer[..number_of_bytes])?;
		println!("Received message: {}", message);

		stream.send_to(&message.as_bytes(), src)?;
	}
}