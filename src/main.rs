use std::io;

fn main() {
	println!("Guess the number!");

	println!("Please input your guess.");

	// # faq
	// difference std::string::String and &str (https://stackoverflow.com/a/24159933)
	// -> String is growable and utf-8 encoded, and &str only a pointer
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {}", guess);
}
