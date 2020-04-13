use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret_number = dbg!(rand::thread_rng().gen_range(1, 101));

	println!("Please input your guess.");

	// # faq
	// difference std::string::String and &str (https://stackoverflow.com/a/24159933)
	// -> String is growable and utf-8 encoded, and &str only a pointer
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	let guess: u32 = guess.trim().parse().expect("Please type a number!");

	match dbg!(guess).cmp(dbg!(&secret_number)) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => println!("You win!"),
	}
}
