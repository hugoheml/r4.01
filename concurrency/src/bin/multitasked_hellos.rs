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

	for i in 1..parameters.n {
		let task = tokio::spawn(async move {
			println!("Hello n°{}", i);
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