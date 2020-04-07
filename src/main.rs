use std::fmt; // Import `fmt`

struct Complex {
	real: f64,
	imag: f64,
}
impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Customize so only `x` and `y` are denoted.
		write!(f, "{} + {}i", self.real, self.imag)
	}
}
impl fmt::Debug for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Customize so only `x` and `y` are denoted.
		write!(f, "{{ real: {}, imag: {} }}", self.real, self.imag)
	}
}

fn main() {
	let complex = Complex {
		real: 3.3,
		imag: 7.2,
	};

	println!("Display: {}", complex);
	println!("Debug: {:#?}", complex);
	// println!("What does Point2D look like in binary: {:b}?", point);
}
