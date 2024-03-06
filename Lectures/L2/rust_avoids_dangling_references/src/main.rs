fn print(s:&String) {
	println!("{}, World1", s);
}

fn main() {
	// let mut s1 = String::from("Hello");

	// // both the statements are for concatenation of strings;
	// // s1 += "what"; // this is working well 
	// // s1.push_str("WHAT");

	// // s1 = "Why".to_string(); nothing... just if you want to reassign the variable with a different string than you need to use this function
	// let r1 = &mut s1;
	// print(r1);
	// println!("{}, World2", r1);
	// println!("{}, World3", s1);

	// SOMETHING DIFFERENT HERE:
	let mut s1 = String::from("Hello");
	let r1 = &s1;
	let r2 = &mut s1; // here it is wrong, now you have already defined &s1 before which is immutable borrowing that you have already done, and now here you are borrowing as mutable which is incorrect for some reason

	println!("{}, World", r1);
	println!("{}, World", r2);
}