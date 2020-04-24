fn main() {
	let x = 5;

	let y = {
		let x = x + 3;
		x + 1
	};

	println!("The value of y² is: {}", square(y));
}

fn square(x: i32) -> i32 {
	x * x
}
