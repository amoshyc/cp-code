#include <algorithm>
#include <functional>
#include <iostream>
#include <map>
#include <vector>
using namespace std;
using ll = long long;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    ll N, M;
    cin >> N >> M;
    vector<ll> A(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    auto S = map<ll, ll>();
    auto G = map<ll, vector<ll>>();
    ll ttl = 0;
    for (auto a : A) {
        S[a] += a;
        G[a].push_back(a);
        ttl += a;
    }

    auto dp = map<ll, ll>();
    function<ll(ll)> f = [&](ll x) {
        if (dp.find(x) != dp.end()) {
            return dp[x];
        }
        ll p = (x - 1 + M) % M;
        dp[x] = 0;
        for (ll y : G[p]) {
            dp[x] = max(dp[x], f(y));
        }
        dp[x] += S[x];
        return dp[x];
    };

    ll ans = 1e18;
    for (ll a : A) {
        ans = min(ans, ttl - f(a));
    }
    cout << ans << endl;

    return 0;
}