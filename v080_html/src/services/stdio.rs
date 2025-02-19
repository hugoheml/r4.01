use async_trait::async_trait;

use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, service::Service, storage::Storage, use_cases::VotingController};

pub struct StdioService<Store> {
	lexicon: Lexicon,
	controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync> Service<Store> for StdioService<Store> {
	fn new(_port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			lexicon,
			controller,
		}
	}

	async fn serve(&mut self) -> Result<(), anyhow::Error> {
		println!("{}", self.lexicon.choose_something_to_do);
		loop {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input)?;
			
			let result = handle_line(&mut self.controller, &self.lexicon, &input).await?;
			println!("{}", result);
		}
	}
}