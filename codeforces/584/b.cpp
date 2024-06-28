#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
const ll M = 1000000000 + 7;

ll fast_pow_mod(ll x, ll n) {
    ll ans = 1;
    while (n != 0) {
        if (n & 1)
            ans = (ans * x) % M;
        x = (x * x) % M;
        n >>= 1;
    }
    return ans;
}

int main() {
    ll N; cin >> N;

    // (3^(3N) - 7^N) % M
    ll a = fast_pow_mod(3, 3 * N);
    ll b = fast_pow_mod(7, N);
    ll ans = a - b;
    if (ans < 0)
        ans += M;
    cout << ans % M << endl;

    return 0;
}
