use async_trait::async_trait;
use tokio::net::UdpSocket;

use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, service::Service, storage::Storage, use_cases::VotingController};

pub struct UdpService<Store> {
	lexicon: Lexicon,
	controller: VotingController<Store>,
	port: u16,
}

#[async_trait]
impl<Store: Storage + Send + Sync> Service<Store> for UdpService<Store> {
	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			lexicon,
			controller,
			port
		}
	}

	async fn serve(&mut self) -> Result<(), anyhow::Error> {
		let udp_address = format!("127.0.0.1:{}", self.port);
		println!("Reading from UDP socket at {}", udp_address);
		let stream = UdpSocket::bind(udp_address.clone()).await?;

		loop {
			let mut buffer = [0; 1024];
			let (received, client_address) = stream.recv_from(&mut buffer).await?;

			let message = std::str::from_utf8(&buffer[..received])?;

			let mut result = handle_line(&mut self.controller, &self.lexicon, message).await?;
			result.insert(result.len(), '\n');
			let result_bin = result.as_bytes();

			stream.send_to(result_bin, client_address).await?;
		}
	}
}