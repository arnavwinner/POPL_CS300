fn main() {
	// practice switch case:


	// let num = 31;
	// match num {
	// 	1 | 3 | 5 | 7 | 9 => println!("Odd!"),
	// 	2 | 4 | 6 | 8 | 0 => println!("Even!"),
	// 	_ => println!("Only 1 digit allowed!"),
	// }

	for i in (1..12).step_by(2) {
		println!("{}", i);
	}


	let num = 12;
	match num {
		13..=19 => println!("Teenager!"),
		_ => println!("Non Teenager!")
	}
}