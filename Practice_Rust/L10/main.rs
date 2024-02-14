use std::mem;

fn main() {
	#[derive(Debug)]
	struct Point3d {
		x:i32,
		y:i32,
		z:i32
	}
	let mut pt = Point3d{x: 1, y: 2, z: 3};
	pt.x = 5;
	println!("element = {:?}", pt);
}