use std::sync::RwLock;
use std::sync::Arc;

use clap::Parser;
use port_scanner::is_open;
use port_scanner::Parameters;

#[tokio::main]
async fn main() {
	let parameters = Parameters::parse();

	let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];
	let opened_ports = Arc::new(RwLock::new(vec![]));

	for i in parameters.port_min..parameters.port_max {
		let host = parameters.host.clone();
		let opened_ports_clone = Arc::clone(&opened_ports);
		let task = tokio::spawn(async move {
			if is_open(&host, i, parameters.timeout).await {
				let mut opened_ports = opened_ports_clone.write().unwrap();
				opened_ports.push(i);
			}
		});
		tasks.push(task);
	}

	for task in tasks {
		task.await.unwrap();
	}

	let opened_ports = opened_ports.read().unwrap();
	for port in opened_ports.iter() {
		println!("Port {} is open", port);
	}
}