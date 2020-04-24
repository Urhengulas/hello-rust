use std::time::SystemTime;

fn main() {
	let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => n.as_secs(),
		Err(_) => panic!("SystemTime before UNIX EPOCH"),
	};
	check_divisiblity(time);
}

fn check_divisiblity(number: u64) {
	if number % 5 == 0 {
		println!("{} is divisible by 5", number);
	} else if number % 4 == 0 {
		println!("{} is divisible by 4", number);
	} else if number % 3 == 0 {
		println!("{} is divisible by 3", number);
	} else if number % 2 == 0 {
		println!("{} is divisible by 2", number);
	} else {
		println!("{} is not divisible by 2, 3, 4 or 5", number)
	}
}
