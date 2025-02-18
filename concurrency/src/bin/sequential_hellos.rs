use clap::Parser;

#[derive(Parser)]
struct Parameters {
	sequential_hellos: u32,
}

fn main() {
	let parameters = Parameters::parse();
	for i in 1..=parameters.sequential_hellos {
		println!("Hello n°{}", i);
	}
}