use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

fn main() {
	let path = "hey.md";
	let file = match File::open(path) {
		Ok(f) => f,
		Err(e) => File::create(path).expect(&*format!("Problem creating the file \"{}\"", path)),
	};
	dbg!(&file);
}
