fn main() {
    let s1 = String::from("Arnav");
    let s2 = &s1;
    // println!("{s2}"); // 0 1
    // println!("Hello World!")
    println!("{}",&s1); // using this would be wrong.
}
