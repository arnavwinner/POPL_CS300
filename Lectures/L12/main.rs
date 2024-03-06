// use std::io;

// fn read_i32() -> i32 {
//     let line = io::stdin().lines().next().unwrap().unwrap();
//     line.parse().unwrap()
// }
// fn main() {
// 	// let arr:[i32;3] = [1, 2, 3];
// 	// let j = 2;
// 	// let n = arr[j];
// 	// println!("{:?}", n);

// 	let mut t = read_i32();
// 	for i in 0..t {
// 		println!("{:?}", i);
// 	}
// }
use std::io::{self, BufRead};

fn read_int() -> usize {
	let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).expect("Failed to read line");
    let n: usize = n_input.trim().parse().expect("Invalid input");
    n
}

fn read_vector() -> Vec<i32> {
	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let v: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();
    v
}


fn solve() {
    // Read input for the size of the vector

    let mut s = String::new(); // string s
    io::stdin().read_line(&mut s); // cin >> s

    let n = s.len();
    let mut cntA = 0;
    let mut cntB = 0;
    for i:i32 in 0..n {
    	if s[i] == 'A' {
    		cntA += 1;
    	}
    	else {
    		cntB += 1;
    	}
    }
    if cntA > cntB {
    	println!("A");
    }
    else {
    	println!("B");
    }
}

fn main() {
    // Read the number of test cases
    let t = read_int();

    // Execute solve function T times
    for _ in 0..t {
        solve();
    }
}
