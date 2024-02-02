
/*
Types were being taught in the class


Strongly Typed, Statically Typed, Dynamically Typed
*/

// author: arnavwinner

#include <bits/stdc++.h>
using namespace std;

void solve() {
	float *f;
	float ff = 3.14;
	f = &ff;
	cout << *f << '\n';
	// cout << *(int *)&f << '\n';
	cout << (int)*f << '\n';

	// float f = 3.14;
	// cout << *((int *)&f) << '\n';
	// cout << (int) *&f << '\n';
}

int main() {

	#ifndef ONLINE_JUDGE
	freopen("input.txt", "r", stdin);
	freopen("output.txt", "w", stdout);
	#endif

	ios::sync_with_stdio(false);
	cin.tie(nullptr);
	
	int T = 1;
	// cin >> T;
	while (T--) {
		solve();
	}
	return 0;
}