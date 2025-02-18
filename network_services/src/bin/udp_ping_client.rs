use std::net::UdpSocket;

const PING: &str = "PING\n";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let stream = UdpSocket::bind("127.0.0.1:0")?;
	stream.send_to(PING.as_bytes(), "127.0.0.1:8888")?;
	Ok(())
}