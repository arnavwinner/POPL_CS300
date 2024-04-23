class Basetest:
	testvar = "ok"

class Test(BaseTest):
	__i = "world" # if you use __ (using two underscores in concatenation), this makes i as private
	def __init__(self):
		self.__i = "initialized"
	def printTest(self, j):
		print(id(j))
		print("Hello" + self.__i)

t = Test()
u = Test()
# print(t.__i)
# t.x = 1234
# print(t.x)
# t.i = "print"
t.printTest("hi")

