# Class 10 POPL

## Playing with tuples in rust


### Snip 1.
```rust
use std::mem;

fn main() {
	let b:bool = false;
	let j = ("abc", 32, 123);
	println!("element = {}", j.2);
	println!("{}", mem::size_of_val(&j));
	println!("{}", mem::align_of_val(&j));
}
```
### Out 1.
```
element = 123
24
8
```

### Snip 2.
```rust
use std::mem;

fn main() {
	let b:bool = false;
	let j = ("abc", 32, 123);
	let (x, ..) = j; // this just ignores whatever is there to the right side. Just takes the first element
	println!("element = {}", x);
}
```
### Out 2.
```
element = abc
```

### Snip 3.
```rust
use std::mem;

fn main() {
	let b:bool = false;
	let j = ("abc", 32, 123);
	let (_, x, _) = j; // this just ignores whatever is there to the right side. Just takes the first element
	println!("element = {}", x);
}
```
### Out 3.
```
element = 32
```

### Snip 4. ERROR
```rust
use std::mem;

fn main() {
	let b:bool = false;
	let j = ("abc", 32, 123);
	let (_, _, _) = j; // We cannot use all _ in the parameters
	println!("element = {}", _);
}
```
### Out 4.
```
Error!
```

### Snip 5. Using Struct and its parameters
```rust
use std::mem;

fn main() {
	#[derive(Debug)]
	struct Point3d {
		x:i32, // just like we write first, second in c++ for pairs, we can create such a structure
		y:i32,
		z:i32
	}
	let mut pt = Point3d{x: 1, y: 2, z: 3};
	pt.x = 5; // same, something like first that we use in c++
	println!("element = {:?}", pt);
}
```
### Out 5.
```
element = Point3d { x: 5, y: 2, z: 3 }
```