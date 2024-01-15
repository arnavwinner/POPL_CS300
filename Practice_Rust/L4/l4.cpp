#include <bits/stdc++.h>
using namespace std;
int main() {
	int a = 7; // a = 0, even in this case Value 3 gets printed
	if (a = 3) { // it just assigns 3 as soon as you write a = 3 in the parentheses
		cout << "Value 3" << '\n';
	}
	else {
		cout << "Not 3" << '\n';
	}
	return 0;
}