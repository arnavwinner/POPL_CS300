use std::io;

fn main() {
	println!("Guess a number!");
	println!("Input a number: ");
	let mut guess = String::new();
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");
	println!("You Guessed: {guess}");
}
