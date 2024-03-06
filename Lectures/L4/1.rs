fn main() {
	let mut a = 7;
	if a == 3 { // in rust you cannot write a = 3 here, so I will write a c++ code to explain
		println!("Value 3");
	}
	else {
		println!("Not 3");
	}
	// here we just get Value 3 as the output. NOTE: Even a = 0 works and will still output Value 3.
}