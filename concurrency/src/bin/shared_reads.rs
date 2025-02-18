use std::sync::Arc;

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

	let hello_text = Arc::new(String::from("Bonjour"));

	let mut tasks: Vec<tokio::task::JoinHandle<()>>= vec![];

	for i in 1..parameters.n {
		let hello_text_clone = Arc::clone(&hello_text);
			
		let task = tokio::spawn(async move {
			println!("{} n° {}", hello_text_clone, i);
		});
		tasks.push(task);

		let task = tokio::spawn(async move {
			println!("Aurevoir n°{}", i);
		});
		tasks.push(task);
	}
	
	for task in tasks {
		task.await.unwrap();
	}
}