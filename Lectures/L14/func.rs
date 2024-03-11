fn main() {
	let i = sum(5);
	println!("value retured: {i}");
}

fn sum(x:i32) -> i32 {
	println!("inside function {x}");
	x
}