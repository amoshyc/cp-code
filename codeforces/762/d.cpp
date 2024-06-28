#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const ll INF = 1e16;
const int MAX_N = 1e5;

int N;
ll A[3][MAX_N];
ll dp[3][MAX_N];

ll sum(int col, int r1, int r2) {
    ll res = 0;
    for (int r = min(r1, r2); r <= max(r1, r2); r++) {
        res += A[r][col];
    }
    return res;
}

int main() {
    scanf("%d", &N);
    for (int r = 0; r < 3; r++) {
        for (int c = 0; c < N; c++) {
            scanf("%lld", &A[r][c]);
        }
    }

    for (int r = 0; r < 3; r++) {
        fill(dp[r], dp[r] + N, -INF);
    }
    dp[0][0] = A[0][0];
    dp[1][0] = A[0][0] + A[1][0];
    dp[2][0] = A[0][0] + A[1][0] + A[2][0];

    for (int c = 1; c < N; c++) {
        for (int p = 0; p < 3; p++) {
            for (int r = 0; r < 3; r++) {
                dp[r][c] = max(dp[r][c], dp[p][c - 1] + sum(c, p, r));
            }
        }

        if (c == 1) {
            dp[2][1] = max(dp[2][1], sum(0, 0, 2) + sum(1, 0, 2));
        }
        if (c >= 2) {
            ll val = sum(c - 1, 0, 2) + sum(c, 0, 2);
            dp[0][c] = max(dp[0][c], dp[2][c - 2] + val);
            dp[2][c] = max(dp[2][c], dp[0][c - 2] + val);
        }
    }

    printf("%lld\n", dp[2][N - 1]);

    return 0;
}
