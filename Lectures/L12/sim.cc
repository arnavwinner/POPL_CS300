#include <bits/stdc++.h>
using namespace std;

void solve() {
	int n;
	cin >> n;
	vector<int> v(n);
	for (int i = 0; i < n; i++) cin >> v[i];
	for (int i = 0; i < n; i++) cout << v[i] << " ";
	cout << '\n';
}

int main() {
	int T = 1;
	cin >> T;
	while (T--) {
		solve();
	}
	return 0;
}