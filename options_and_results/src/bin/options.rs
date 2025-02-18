#![allow(non_snake_case)]
#![allow(dead_code)]

fn print_first_word1(sentence: &str) {
	match sentence.split_whitespace().next() {
		None => println!("Chaine vide"),
		Some(word) => println!("{}", word),
	}
}

fn print_first_word2(sentence: &str) -> &str {
	let word: Option<&str> = sentence.split_whitespace().next();
	let wordStr = word.expect("La chaîne doit être non vide.");

	return wordStr;
}

fn iterate_over_words(sentence: &str) {
	for word in sentence.split_whitespace() {
		println!("Mot: {}", word);
	}
}

fn main() {
	let sentence1 = "Bonjour Limoges";
	let sentence2 = "";

	// print_first_word1(sentence1);
	// print_first_word1(sentence2);

	print_first_word2(sentence1);
	// print_first_word2(sentence2);

	iterate_over_words(sentence1);
	iterate_over_words(sentence2);
}