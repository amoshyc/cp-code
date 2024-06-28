#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll n, a, b, c, d;
    cin >> n >> a >> b >> c >> d;

    ll cnt = 0;
    for (ll x = 1; x <= n; x++) {
        ll y = x + (b - c);
        ll w = x + (d - a);
        ll u = y + (d - a);

        if (1 <= y && y <= n &&
            1 <= w && w <= n &&
            1 <= u && u <= n)
            cnt++;
    }

    cout << cnt * n << "\n";

    return 0;
}
