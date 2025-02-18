use clap::Parser;
use port_scanner::is_open;
use port_scanner::Parameters;

#[tokio::main]
async fn main() {
	let parameters = Parameters::parse();

	for i in parameters.port_min..parameters.port_max {
		if is_open(&parameters.host, i, parameters.timeout).await {
			println!("Port {} is open", i);
		}
	}
}