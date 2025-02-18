use clap::Parser;
use tokio::net::UdpSocket;

#[derive(Debug, Parser)]
struct Parameters {
	ip: String,
	port: u16
}

#[tokio::main]
async fn main() {
	let parameters = Parameters::parse();

	let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
	let server_ip = format!("{}:{}", parameters.ip, parameters.port);
	println!("Connecting to server: {}", server_ip);
	socket.connect(server_ip).await.unwrap();

	loop {
		let mut buffer = [0; 1024];
		let received = socket.recv(&mut buffer).await.unwrap();

		let message = std::str::from_utf8(&buffer[..received]).unwrap();
		println!("Message: {message}");
	}
}