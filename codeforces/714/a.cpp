#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

bool inside(ll a, ll b, ll c) {
    return (b <= a && a <= c);
}

int main() {
    ll l1, r1, l2, r2, k;
    cin >> l1 >> r1 >> l2 >> r2 >> k;

    // [l1, r1], [l2, r2]

    ll l = max(l1, l2);
    ll r = min(r1, r2);

    ll overlap = r - l + 1;
    if (l <= k && k <= r)
        overlap--;

    cout << max(overlap, 0ll) << endl;

    return 0;
}
