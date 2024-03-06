fn main() {
	let b:bool = true;
	let mut i:u8 = 32;
	println!("{b}");
	i -= 31; // this will work, but if you do i -= 33, it will become negative, which is wrong
	println!("{i}");
}