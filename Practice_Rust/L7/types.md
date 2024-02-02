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

**Let's look here**

```cpp
float *f;
float ff = 3.14;
f = &ff;
cout << *f << '\n'; // this gives 3.14
cout << *((int *)&f) << '\n'; // this is incorrect (but sir told this one) .. output = 96880596 (this number is actually a random number for some reason)
cout << (int)*f << '\n'; // this is correct... output = 3
```

The above one is for 
```cpp
float *f
```

But trying for below:
```cpp
float f
```

```cpp
float f = 3.14;
cout << (int)*&f << '\n'; // this looks fine and outputs 3 here
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


