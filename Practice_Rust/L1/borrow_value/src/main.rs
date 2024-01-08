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

// fn print(s:&String){ // now we are using the reference by pointer
// 	println!("{}, world!", s);
// 	// s // this is the return
// }

// fn main() {
//     let s1 = String::from("Hello");
//     print(&s1); // here we are passing the returned value from the function
//     println!("{}, world!", s1);
// }

//----------------------------------------------------------------

fn print(s:&String){ // now we are using the reference by pointer
	println!("{}, world!", s);
	// s // this is the return
}

fn main() {
    let s1 = String::from("Hello");
    let r1 = &s1; // here we are passing the returned value from the function
    print(r1);
    println!("{}, world!", s1);
}