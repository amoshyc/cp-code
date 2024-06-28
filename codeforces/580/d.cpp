#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MAX_N = 18;
const int MAX_M = 18;

int N, M, K;
ll a[MAX_N];
ll extra[MAX_N][MAX_N];
ll dp[(1 << MAX_M)][MAX_N];

int main() {
    scanf("%d %d %d", &N, &M, &K);
    for (int i = 0; i < N; i++) {
        scanf("%lld", &a[i]);
    }
    for (int i = 0; i < K; i++) {
        int u, v, c; scanf("%d %d %d", &u, &v, &c); u--; v--;
        extra[u][v] = c;
    }

    // init
    dp[0][0] = 0;

    ll ans = -1;
    for (int S = 1; S < (1 << N); S++) {
        for (int v = 0; v < N; v++) {
            if (!(S & (1 << v))) { // v not in S, skipped
                continue;
            }

            int cnt = 0;
            for (int i = 0; i < N; i++)
                if (S & (1 << i))
                    cnt++;

            if (cnt > M)
                continue;

            for (int u = 0; u < N; u++) {
                if (!(S & (1 << u))) { // u not in S, skipped
                    continue;
                }

                dp[S][v] = max(dp[S][v],
                    dp[S ^ (1 << v)][u] + a[v] + extra[u][v]
                );
            }

            if (cnt == M) {
                ans = max(ans, dp[S][v]);
            }
        }
    }

    printf("%lld\n", ans);

    return 0;
}
