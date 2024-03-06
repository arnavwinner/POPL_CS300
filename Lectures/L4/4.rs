fn main() {
	let x = ret_five(5) + 3;
	println!("{}", x);
}
fn ret_five(x: i32) -> i32 { // ofc you need to write the return type and the parameter datatype as well, just like you do in c++
	x
}