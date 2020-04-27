fn main() {
	// structs
	{
		#[derive(Debug)]
		struct User {
			username: String,
			email: String,
			sign_in_count: u64,
			active: bool,
		}

		let user1 = User {
			email: String::from("someone@example.com"),
			username: String::from("someusername123"),
			active: true,
			sign_in_count: 1,
		};
		let user2 = User {
			email: String::from("mail@lukasmüller.com"),
			username: String::from("mToTheLukas"),
			..user1
		};
		dbg!((user1, user2));
	}

	// tuple structs
	{
		#[derive(Debug)]
		struct Color(i32, i32, i32);
		#[derive(Debug)]
		struct Point(i32, i32, i32);

		let black = Color(0, 0, 0);
		let origin = Point(0, 0, 0);
		dbg!((black, origin));
	}
}
