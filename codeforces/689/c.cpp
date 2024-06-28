#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

ll M;

ll P(ll n) {
    ll cnt = 0;
    for (ll k = 2; 1 * k * k * k <= n; k++) {
        cnt += n / (k * k * k);
    }
    return cnt;
}

int main() {
    cin >> M;

    // 0 0 0 0 1 1 1
    // first 1

    ll lb = 0, ub = 1e18;

    while (ub - lb > 1) {
        ll mid = (lb + ub) / 2;
        if (P(mid) >= M) ub = mid;
        else lb = mid;
    }

    if (P(ub) == M) cout << ub << endl;
    else {
        cout << "-1\n";
    }

    return 0;
}
