# Class - 8, POPL

## Types

1. Type `Equivalence`
2. Type `Compatibility`
3. Type `Inference`

Example: Assigning `long long int` to `int` would **overflow** the variable, hence using correct types is important.

Example:<br>

1. **"Hi" * 5**
2. **"Hi" + 5**

These two will return different type.

# Very Very Important Below

```cpp
float f = 3.14;
cout << *((int *)&f) << '\n'; // this is correct (this will give the int, but not necessarily the round of value) .. output = 96880596 (this number is actually a random number for some reason)
```

## Endians

1. Big Endian: Bits will reverse, but the individual block of bits wouldn't. We assume that the rightmost consumes the least memory.
2. Little Endian: Here MSB is stored the place which consumes the max memory.


```cpp
union u
{
	int i;
	char c[u];
}; // this is kind of struct that we made to show how the bits are arranged
```
The above `int` is stored something like this:

|b1	|b2	|b3	|b4	|
|---|---|---|---|
|c[0]|c[1]|c[2]|c[3]|


