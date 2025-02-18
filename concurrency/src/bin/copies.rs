use std::sync::{Arc, RwLock};

use clap::Parser;

#[derive(Parser)]
struct Parameters {
	n: u32
}

#[tokio::main]
async fn main() {
	let parameters = Parameters::parse();
	// for i in 1..parameters.n {
	// 	println!("Hello n°{}", i);
	// 	println!("Aurevoir n°{}", i);
	// }

	let mut tasks: Vec<tokio::task::JoinHandle<()>>= vec![];

	let mut entier = 0;

	for _ in 1..parameters.n {
		let task = tokio::spawn(async move {

			entier += 1;

			println!("Bonjour n°{}", entier);
			println!("Aurevoir n°{}", entier);
		});
		tasks.push(task);
	}
	
	for task in tasks {
		task.await.unwrap();
	}
}