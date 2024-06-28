#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const ll INF = 1e18;
#define sz(x) (int(x.size()))

ll max_subarray(const vector<ll>& v) {
    vector<ll> dp(sz(v), -INF);
    dp[0] = v[0];
    for (int i = 1; i < sz(v); i++) {
        dp[i] = max(dp[i - 1] + v[i], v[i]);
    }
    return *max_element(dp.begin(), dp.end());
}

int main() {
    int N;
    scanf("%d", &N);

    vector<ll> d;
    ll p; scanf("%lld", &p);
    for (int i = 1; i < N; i++) {
        ll c; scanf("%lld", &c);
        d.push_back(abs(c - p));
        p = c;
    }

    vector<ll> d1 = d, d2 = d;
    for (int i = 0; i < sz(d); i++) {
        if (i % 2 == 0) {
            d1[i] *= (-1);
        }
        else {
            d2[i] *= (-1);
        }
    }

    printf("%lld\n", max(max_subarray(d1), max_subarray(d2)));

    return 0;
}
