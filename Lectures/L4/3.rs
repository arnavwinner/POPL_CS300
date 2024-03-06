fn main() {
	let x = ret_five() + 3;
	println!("{}", x);
}
fn ret_five() -> i32 { // here if you don't write i32, it gives an error; REASON: the return type is otherwise "()" and not integer.
	let y = 5;
	y
}