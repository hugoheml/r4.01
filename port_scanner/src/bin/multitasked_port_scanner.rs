use clap::Parser;
use port_scanner::is_open;
use port_scanner::Parameters;

#[tokio::main]
async fn main() {
	let parameters = Parameters::parse();

	let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];

	for i in parameters.port_min..parameters.port_max {
		let host = parameters.host.clone();
		let task = tokio::spawn(async move {
			if is_open(&host, i, parameters.timeout).await {
				println!("Port {} is open", i);
			}
		});
		tasks.push(task);
	}

	for task in tasks {
		task.await.unwrap();
	}
}