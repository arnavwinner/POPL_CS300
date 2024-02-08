use std::mem;

fn main() {
	let j = ("abc", 32);
	println!("{}", mem::size_of_val(&j));
	println!("---------------------------------");
	println!("{}", mem::size_of_val(j.0));
	println!("{}", mem::align_of_val(j.0)); // this is interesting, even if we don't use &j.0 here, it still works since we are using string here, we can directly use j.0 here
}