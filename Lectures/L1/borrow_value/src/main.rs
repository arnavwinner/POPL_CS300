// WAY 1: We are passing the s1 through return type of print function

// fn print(s:String)->String {
// 	println!("{}, world!", s);
// 	s // this is the return
// }

// fn main() {
//     let mut s1 = String::from("Hello");
//     s1 = print(s1); // here we are passing the returned value from the function
//     println!("{}, world!", s1);
// }
// now here you need to use a mutable variable and return the value using a function.

//----------------------------------------------------------------

// WAY 2: We are directly passing the reference of s1 in the function of print

// fn print(s:&String){ // now we are using the reference by pointer // Here & is important, you cannot use it without it
// 	println!("{}, world!", s);
// 	// s // this is the return
// }

// fn main() {
//     let s1 = String::from("Hello");
//     print(&s1); // here we are passing the returned value from the function
//     println!("{}, world!", s1);
// }

//----------------------------------------------------------------

// WAY 3: We are using a new non-mutable variable r1 which is given the reference of s1 for passing it to print function

fn print(s:&String){ // now we are using the reference by pointer // Here & is important, you cannot use it without it
	println!("{}, world!", s);
	// s // this is the return
}

fn main() {
    let s1 = String::from("Hello");
    let r1 = &s1; // here we are passing the returned value from the function
    print(r1);
    println!("{}, world!", s1);
}