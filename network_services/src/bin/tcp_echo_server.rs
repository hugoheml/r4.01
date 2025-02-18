use clap::Parser;
	use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener};

const LOCALHOST: &str ="127.0.0.1";

#[derive(Parser, Debug)]
struct Parameters {
	port: u16
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let parameters = Parameters::parse();
	let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, parameters.port)).await?;
	loop {
		let (stream, _) = listener.accept().await?;
		
		tokio::spawn(async move{
			let (reader, mut writer) = stream.into_split();
			let mut lines = BufReader::new(reader).lines();
			
			loop {
				if let Ok(Some(line)) = lines.next_line().await {
					println!("Received message: {}", line);
					writer.write_all(line.as_bytes()).await.unwrap();
					writer.flush().await.unwrap();
				}
			}
		});
	}
}

