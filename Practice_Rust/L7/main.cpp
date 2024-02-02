
/*
Types were being taught in the class


Strongly Typed, Statically Typed, Dynamically Typed
*/

// author: arnavwinner

#include <bits/stdc++.h>
using namespace std;

void solve() {
	// cout << "Hello World" << '\n';
	cout << gcd(108, 12) << '\n';
	cout << (108 * 12) / gcd(108, 12) << '\n';
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