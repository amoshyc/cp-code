#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

ll a[3];

ll solve(ll s, ll t) {
    ll need[3] = {0};
    for (int i = s; i != t; i = (i + 1) % 3) {
        need[i]++;
    }

    // 0 0 0 1 1 1
    ll lb = -1, ub = 8e18;
    while (ub - lb > 1) {
        ll mid = (lb + ub) / 2;

        ll x = need[0] + mid;
        ll y = need[1] + mid;
        ll z = need[2] + mid;

        if (x >= a[0] && y >= a[1] && z >= a[2]) ub = mid;
        else lb = mid;
    }

    need[0] += ub;
    need[1] += ub;
    need[2] += ub;

    return (need[0] + need[1] + need[2]) - (a[0] + a[1] + a[2]);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> a[0] >> a[1] >> a[2];

    ll ans = 8e18;
    for (int s = 0; s < 3; s++) {
        for (int t = 0; t < 3; t++) {
            ans = min(ans, solve(s, t));
        }
    }

    cout << ans << endl;

    return 0;
}
