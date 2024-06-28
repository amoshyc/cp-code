#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N;
    scanf("%d", &N);

    vector<ll> v(N, 0);
    for (ll& i : v)
        scanf("%lld", &i);

    sort(v.begin(), v.end());

    ll ans = 0;
    for (int i = 0; i < N; i++) {
        ans += abs((i + 1) - v[i]);
    }

    printf("%lld\n", ans);

    return 0;
}
