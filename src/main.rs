struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data `{}`!", self.data);
	}
}

fn main() {
	let _a = CustomSmartPointer {
		data: String::from("A"),
	};
	println!("1st");
	{
		let _b = CustomSmartPointer {
			data: String::from("B"),
		};
		let _c = CustomSmartPointer {
			data: String::from("C"),
		};
		println!("2nd");
	}
	let _d = CustomSmartPointer {
		data: String::from("D"),
	};
	println!("3rd");
}
