#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll n;
    cin >> n;

    if (n == 0) {
        cout << "0 0 0\n";
        return 0;
    }
    if (n == 1) {
        cout << "0 0 1\n";
        return 0;
    }
    if (n == 2) {
        cout << "0 1 1\n";
        return 0;
    }

    vector<ll> f;
    ll f1 = 0, f2 = 1;
    f.push_back(f1);
    f.push_back(f2);
    while (f1 + f2 < ll(1e9)) {
        ll res = f1 + f2;
        f.push_back(res);
        f1 = f2;
        f2 = res;
    }

    int idx = lower_bound(f.begin(), f.end(), n) - f.begin();
    cout << f[idx - 4] << " " << f[idx - 3] << " " << f[idx - 1] << endl;

    return 0;
}
