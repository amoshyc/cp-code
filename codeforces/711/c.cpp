#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 100;
const int MAX_M = 100;
const int MAX_K = 100;
const ll INF = 1e18;

// all 1-based index
int N, M, K;
int c[MAX_N + 1];
ll p[MAX_N + 1][MAX_M + 1];
ll dp[MAX_N + 1][MAX_K + 1][MAX_M + 1];

void upd_min(ll& a, ll b) {
    a = min(a, b);
}

int main() {
    scanf("%d %d %d", &N, &M, &K);
    for (int i = 1; i <= N; i++)
        scanf("%d", &c[i]);
    for (int i = 1; i <= N; i++)
        for (int j = 1; j <= M; j++)
            scanf("%lld", &p[i][j]);

    for (int i = 0; i <= N; i++) {
        for (int j = 0; j <= K; j++) {
            for (int k = 0; k <= M; k++) {
                dp[i][j][k] = INF;
            }
        }
    }

    // base case
    if (c[1] != 0) dp[1][1][c[1]] = 0;
    else {
        for (int k = 1; k <= M; k++) {
            dp[1][1][k] = p[1][k];
        }
    }

    for (int i = 2; i <= N; i++) {
        for (int j = 1; j <= i; j++) {
            if (c[i] != 0) { // c[i] is fixed
                for (int z = 1; z <= M; z++) { // color at (i - 1)-th
                    if (z == c[i])
                        upd_min(dp[i][j][c[i]], dp[i - 1][j][z]);
                    else
                        upd_min(dp[i][j][c[i]], dp[i - 1][j - 1][z]);
                }
            }
            else { // c[i] is not fixed
                for (int k = 1; k <= M; k++) { // color at i-th
                    for (int z = 1; z <= M; z++) { // color at (i - 1)-th
                        if (z == k)
                            upd_min(dp[i][j][k], dp[i - 1][j][z] + p[i][k]);
                        else
                            upd_min(dp[i][j][k], dp[i - 1][j - 1][z] + p[i][k]);
                    }
                }
            }
        }
    }

    // for (int i = 0; i <= N; i++) {
    //     for (int j = 0; j <= i; j++) {
    //         for (int k = 1; k <= M; k++) {
    //             printf("dp[%d][%d][%d] = %lld\n", i, j, k, dp[i][j][k]);
    //         }
    //     }
    //     puts("");
    // }

    ll ans = *min_element(dp[N][K] + 1, dp[N][K] + M + 1);
    if (ans >= INF) puts("-1");
    else {
        printf("%lld\n", ans);
    }

    return 0;
}
