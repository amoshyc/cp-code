#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 200000;
int N, K;
ll A[MAX_N];
ll R[MAX_N];
int dp[MAX_N];
// dp[i] = x such that R[x] is max among R[0, i - K]
// dp[i] = dp[i ]

int main() {
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++) {
        scanf("%lld", &A[i]);
    }

    R[K - 1] = accumulate(A, A + K, 0ll);
    for (int i = K; i < N; i++) {
        R[i] = R[i - 1] - A[i - K] + A[i];
    }

    // for (int i = K - 1; i < N; i++) {
    //     printf("%lld ", R[i]);
    // }
    // puts("");

    dp[K] = K - 1;
    for (int i = K + 1; i < N; i++) {
        if (R[i - K] > R[dp[i - 1]]) {
            dp[i] = i -  K;
        }
        else {
            dp[i] = dp[i - 1];
        }
    }

    // for (int i = K; i < N; i++) {
    //     printf("%lld ", R[dp[i]]);
    // }
    // puts("");

    ll ans = -1; int idx1, idx2;
    for (int i = 2 * K - 1; i < N; i++) {
        // printf("%d : %lld\n", i, R[dp[i]] + R[i]);
        if (R[dp[i]] + R[i] > ans) {
            ans = R[dp[i]] + R[i];
            idx1 = dp[i];
            idx2 = i;
        }
    }

    // printf("%d %d\n", idx1, idx2);
    printf("%d %d\n", idx1 - K + 2, idx2 - K + 2);

    return 0;
}
