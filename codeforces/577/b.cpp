#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 1e6;
const int MAX_M = 1e3;

int N, M;
int A[MAX_N];
bool dp[MAX_M][MAX_M];

inline int mod(int a, int b) {
    return (a + b) % b;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++)
        scanf("%d", &A[i]);

    if (N > M) {
        puts("YES");
        return 0;
    }

    // dp[i][j] = 前 i + 1 個數可否組出 mod m = j 的數
    // dp[i][j] = true if dp[i - 1][(j - (a[i] mod m)) mod m] or
    //                    dp[i - 1][j] or
    //                    j = a[i] % m

    dp[0][A[0] % M] = true;
    for (int i = 1; i < N; i++) {
        dp[i][A[i] % M] = true;
        for (int j = 0; j < M; j++) {
            if (dp[i - 1][j] || dp[i - 1][mod(j - A[i] % M, M)]) {
                dp[i][j] = true;
            }
        }
    }

    // for (int i = 0; i < N; i++) {
    //     printf("dp[%d] = ", i);
    //     for (int j = 0; j < M; j++) {
    //         printf("%d ", dp[i][j]);
    //     }
    //     puts("");
    // }

    puts(((dp[N - 1][0]) ? "YES" : "NO"));

    return 0;
}
