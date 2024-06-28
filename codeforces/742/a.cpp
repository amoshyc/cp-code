#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;

ll mod_pow(ll a, ll b) {
    ll ans = 1;
    ll base = a % 10;
    while (b) {
        if (b & 1) ans = (ans * base) % 10;
        base = (base * base) % 10;
        b >>= 1;
    }
    return ans;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int n;
    cin >> n;
    cout << mod_pow(1378, n) << endl;

    return 0;
}
