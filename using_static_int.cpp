#include <bits/stdc++.h>
using namespace std;

using i64 = long long;

void perform() {
    static int x = 0;
    x += 1;
    cout << x << '\n';
}

void solve() {
    perform();
    perform();
    perform();
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int tt = 1;
    // cin >> tt;
    while (tt--) {
        solve();
    }

    return 0;
}