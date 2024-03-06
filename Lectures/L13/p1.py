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