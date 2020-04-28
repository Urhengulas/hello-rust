#[derive(Debug)]
struct Cuboid {
	height: u32,
	width: u32,
	depth: u32,
}
impl Cuboid {
	// methods
	fn volume(&self) -> u32 {
		self.height * self.width * self.depth
	}
	fn can_hold(&self, other: &Cuboid) -> bool {
		self.height >= other.height && self.width >= other.width && self.depth >= other.depth
	}
	// associated functions
	fn create_dice(size: u32) -> Cuboid {
		Cuboid {
			height: size,
			width: size,
			depth: size,
		}
	}
}

fn main() {
	let c1 = Cuboid {
		height: 30,
		width: 50,
		depth: 20,
	};
	dbg!(c1.volume());
	dbg!(c1.can_hold(&Cuboid {
		height: 30,
		width: 45,
		depth: 20
	}));
	dbg!(c1.can_hold(&Cuboid {
		height: 35,
		width: 50,
		depth: 20
	}));
	let dice = Cuboid::create_dice(20);
	dbg!(dice.volume());
	dbg!(c1.can_hold(&dice));
}
