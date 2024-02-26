fn main() {

	// just a normal for loop:

	// let mut i = 0;
	// loop {
	// 	if i == 10 {
	// 		break;
	// 	}
	// 	println!("i = {i}, Again!");
	// 	i += 1;
	// }


	// using "return" as we use in c++, to break out of all the loops
	let mut i = 1;
	'counting: loop {
		println!("Again!");
		loop {
			if i == 2 {
				// break;
				break 'counting;
			}
			i += 1;
		}
		if i == 10 {
			break;
		}
	};
	println!("i = {i}");
}