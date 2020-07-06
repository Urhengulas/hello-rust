use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn hello(name: &str) {
	println!("Hello, {}!", name);
}

fn main() {
	hello(&MyBox::new(String::from("Deref")));
}
