#include <vector>
#include <map>
#include <iostream>
#include <utility>
using namespace std;
using ll = long long;

ll powmod(ll a, ll b, ll m) {
    ll base = a;
    ll res = 1;
    while (b) {
        if (b & 1) {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    return res;
}

ll N, M, K;
ll mod = 998244353;
ll M_inv = 0;
map<pair<ll, ll>, ll> dp;


ll p(ll k, ll x) {
    if (dp.find({k, x}) != dp.end()) {
        return dp[{k, x}];
    }

    if (x == N) {
        return 1;
    }

    if (k == 0) {
        return 0;
    }

    ll val = 0;
    for (ll i = 1; i <= M; i++) {
        ll nx = 0;
        if (x + i >= N) {
            nx = N - (x + i - N);
        } else {
            nx = x + i;
        }
        val = (val + p(k - 1, nx) * M_inv % mod) % mod;
    }
    dp[{k, x}] = val;
    return val;
}


int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> N >> M >> K;
    M_inv = powmod(M, mod - 2, mod);

    cout << p(K, 0) << endl;
    // ll ans = 0;
    // for (int k = 0; k <= K; k++) {
    //     cout << p(k, 0) << endl;
    //     ans = (ans + p(k, 0)) % mod;
    // }
    // cout << ans << endl;
    
    return 0;
}