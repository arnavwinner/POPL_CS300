# Using a function size_of_val(...args)

* use **`std::mem;`**  in the starting, before the **fn main()** function.
* the function inside mem, i.e. **`size_of_val(&j)`**, where j is the variable which you want to see that how much size is occupied in the memory by this variable.

## Unsigned int

```rust
use std::mem;

fn main() {
	let b:bool = false;
	let j:u128 = 32; // for this variable, OUTPUT: 16
	println!("{}", mem::size_of_val(&b)); // OUTPUT: 1
}
```

## Character

```rust
use std::mem;

fn main() {
	let j:char = 'a';
	println!("{}", mem::size_of_val(&j)); // OUTPUT: 4
}
```

## String

```rust
use std::mem;

fn main() {
	let j:&str = "abcdefgh"; // string can be accessed using the pointer only.
	println!("{}", mem::size_of_val(&j)); // OUTPUT: 16
}
```

## Float

```rust
use std::mem;

fn main() {
	let j:f32 = 32.0; // string can be accessed using the pointer only.
	println!("{}", mem::size_of_val(&j)); // OUTPUT: 16
}
```


### 2 Wrong Ways:

```rust
use std::mem;

fn main() {
	let j:f32 = 32.0 + 2; // ERROR
	println!("{}", mem::size_of_val(&j)); // OUTPUT: 16
}

// ----------------------------------------------------------------------------

use std::mem;

fn main() {
	let j:f32 = 2 + 32.0; // ERROR
	println!("{}", mem::size_of_val(&j)); // OUTPUT: 16
}
```

### Correct Way (Use Positional Arguements as well):

```rust
use std::mem;

fn main() {
	let j:f32 = 32.0 + 2.0; // Correct
	println!("{0} {1}", mem::size_of_val(&j), j); // OUTPUT: 4 34
}
```

## Composite Types like Tuple

```rust
use std::mem;

fn main() {
	let j = ("abc", 32);
	println!("{}", mem::size_of_val(&j));
	println!("---------------------------------");
	println!("{}", mem::size_of_val(j.0));
	println!("{}", mem::align_of_val(j.0)); // this is interesting, even if we don't use &j.0 here, it still works since we are using string here, we can directly use j.0 here
}
```
* Also some kind of padding is required to fill up the space in order to use the memory appropriately.

<!-- ```rust,ignore
# This example won't be tested. wow!! we can use these as the comments in the md file
panic!("oops!");
``` -->