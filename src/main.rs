pub trait Draw {
	fn draw(&self);
}

#[derive(Default)]
pub struct Screen {
	pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

#[derive(Debug)]
pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}
impl Draw for Button {
	fn draw(&self) {
		println!("Draw {:?} on screen", self);
	}
}

fn main() {
	let mut s = Screen::default();
	s.components.push(Box::new(Button {
		width: 4,
		height: 5,
		label: "sign-up".to_string(),
	}));
	s.components.push(Box::new(Button {
		width: 10,
		height: 3,
		label: "sign-in".to_string(),
	}));

	s.run();
}
