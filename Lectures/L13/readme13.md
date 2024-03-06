# Lecture 13

## About Functions and default parameters


### Input

```python

def sum(i, j = 5):
	# j = i + j.... doesn't matter still IMPORTANT: "j is immutable here"
	return i + j

print(sum(3, 2))
print(sum(4))
print(sum(5))


def fun(i, l = []):
	# print(id(l))
	l.append(i)
	return l

print(fun(1))
print(fun(2))
print(fun(3))
print(fun(4, [1, 2]))
print(fun(5))

```

### Output

```
5
9
10
[1]
[1, 2]
[1, 2, 3]
[1, 2, 4]
[1, 2, 3, 5]
```


* **Inference**: When you call a function using **default parameter**: you can say that, that object is **globally declared**