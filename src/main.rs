fn main() {
	check_divisiblity(22);
}

fn check_divisiblity(number: i32) {
	if number % 5 == 0 {
		println!("number is divisible by 5");
	} else if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is not divisible by 2, 3, 4 or 5")
	}
}
