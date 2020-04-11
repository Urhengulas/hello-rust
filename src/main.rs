// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
enum Abrafaxe {
	Abrax,
	Bebrax,
	Califax,
}

// enum with explicit discriminator
enum Color {
	Red = 0xff0000,
	Green = 0x00ff00,
	Blue = 0x0000ff,
}

fn main() {
	// `enums` can be cast as integers.
	println!("Abrax is {}", Abrafaxe::Abrax as i32);
	println!("Califax is {}", Abrafaxe::Califax as i32);

	println!("\nRoses are #{:06x}", Color::Red as i32);
	println!("Violets are #{:06x}", Color::Blue as i32);
	println!("Sugar is sweet,\nAnd so are you.")
}
