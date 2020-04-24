use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
	let unix_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => n.as_secs(),
		Err(_) => panic!("SystemTime before UNIX EPOCH"),
	};
	dbg!(unix_time);

	// quintuple counter until it is greater or equal to unix_time at start of program
	let mut counter = 1;
	let result = loop {
		if dbg!(counter) >= unix_time {
			break counter;
		};
		counter *= 5;
		sleep(0.1);
	};
	dbg!(unix_time);
	dbg!(result);
}

fn sleep(time: f64) {
	thread::sleep(Duration::from_secs_f64(time));
}
