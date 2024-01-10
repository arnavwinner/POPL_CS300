fn print(s:&String) {
	println!("{}, World1", s);
}

fn main() {
	let mut s1 = String::from("Hello");
	s1 = "Why".to_string();
	let r1 = &s1;
	print(r1);
	println!("{}, World2", r1);
	println!("{}, World3", s1);
}