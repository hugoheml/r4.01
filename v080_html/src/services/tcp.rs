use async_trait::async_trait;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener};

use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, service::Service, storage::Storage, use_cases::VotingController};

pub struct TcpService<Store> {
	lexicon: Lexicon,
	controller: VotingController<Store>,
	port: u16,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone + 'static> Service<Store> for TcpService<Store> {
	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			lexicon,
			controller,
			port
		}
	}

	async fn serve(&mut self) -> Result<(), anyhow::Error> {
		let tcp_address = format!("127.0.0.1:{}", self.port);
		println!("Listening on TCP socket at {}", tcp_address);
		let listener = TcpListener::bind(tcp_address.clone()).await?;


		loop {
			let (stream, _) = listener.accept().await?;
			let mut controller_clone = self.controller.clone();
			let lexicon_clone = self.lexicon.clone();
			
			tokio::spawn(async move {
				let (reader, mut writer) = stream.into_split();
				let mut lines = BufReader::new(reader).lines();

				while let Ok(Some(line)) = lines.next_line().await {
					let result = handle_line(&mut controller_clone, &lexicon_clone, &line).await.unwrap();
				
					if writer.write_all((result + "\n").as_bytes()).await.is_err() {
						break;
					}
					if writer.flush().await.is_err() {
						break;
					}
				}
			});
        }
    }
}