// A struct with two fields
#[derive(Debug)]
struct Point {
	x: f32,
	y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
	// A rectangle can be specified by where the top left and bottom right
	// corners are in space.
	top_left: Point,
	bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f64 {
	let Rectangle {
		top_left: Point {
			x: left_x,
			y: top_y,
		},
		bottom_right: Point {
			x: right_x,
			y: bottom_y,
		},
	} = r;
	let (d_x, d_y) = (right_x - left_x, top_y - bottom_y);
	return (d_x * d_y).abs().into();
}

fn square(p: Point, a: f32) -> Rectangle {
	return Rectangle {
		top_left: Point { x: p.x, y: p.y + a },
		bottom_right: Point { x: p.x + a, y: p.y },
	};
}

fn main() {
	let rectangle = Rectangle {
		bottom_right: Point { x: 1.2, y: -3.4 },
		top_left: Point { x: 5.6, y: 7.8 },
	};
	let a_reactangle = rect_area(dbg!(rectangle));
	println!("area: {} AU\n", a_reactangle);

	let rectangle_sq = square(Point { x: 5.6, y: 7.8 }, 9.1);
	println!("square rectangle: {:?}", rectangle_sq);
	let a_reactangle_sq = rect_area(rectangle_sq);
	println!("area: {} AU", a_reactangle_sq);
}
