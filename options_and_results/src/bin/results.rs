fn convert_to_int1(sentence: &str) {
	match sentence.parse::<i32>() {
		Err(_error) => println!("{} n'est pas un entier", sentence),
		Ok(number) => println!("Le carré de {} est {}", number, number * number),
	}
}

fn convert_to_int2(sentence: &str) {
	let result = sentence.parse::<i32>();

	let number = result.expect("La chaîne ne contient pas d'entier");

	println!("Le carré de {} est {}", number, number * number);
}

fn convert_to_int3(sentence: &str) -> anyhow::Result<()> {
	let number = sentence.parse::<i32>()?;

	println!("Le carré de {} est {}", number, number * number);

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let string1 = "5";
	let string2 = "Bonjour";

	convert_to_int3(string1)?;
	convert_to_int3(string2)?;

	Ok(())
}