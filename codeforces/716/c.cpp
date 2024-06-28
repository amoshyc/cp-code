#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N;
    scanf("%d", &N);

    vector<ll> ans(N + 1, 0);
    ans[1] = 14;
    ans[2] = 16;

    for (ll k = 3; k <= N; k++) {
        ans[k] = k * (k + 1) * (k + 1) - (k - 1);
    }

    for (int i = 1; i <= N; i++)
        printf("%lld\n", ans[i]);

    return 0;
}
