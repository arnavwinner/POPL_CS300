fn main() {
	let x = {
		let y = 4;
		y + 1 // remember that you don't write semi colon while returning a value from a function.
	} + 3; // this works
	println!("{}", x);
}