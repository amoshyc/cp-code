#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<ll, ll> pll;

ll x[2000];
ll y[2000];

int main() {
    int N; scanf("%d", &N);

    map<pll, int> cnt;
    for (int i = 0; i < N; i++) {
        scanf("%lld %lld", &x[i], &y[i]);
    }

    for (int i = 0; i < N - 1; i++) {
        for (int j = i + 1; j < N; j++) {
            cnt[pll(x[i] + x[j], y[i] + y[j])]++;
        }
    }

    ll ans = 0;
    for (auto it = cnt.begin(); it != cnt.end(); ++it) {
        int n = it->second;
        ans += n * (n - 1) / 2;
    }

    printf("%lld\n", ans);

    return 0;
}
