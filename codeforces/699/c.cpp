#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 100;
int N;
int A[MAX_N];
const int gym = (1 << 1);
const int con = (1 << 0);
int dp[MAX_N][3];

int mymin(int a, int b, int c) {
    return min(a, min(b, c));
}

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%d", &A[i]);
    }

    dp[0][0] = ((A[0] & gym) ? 0 : 1);
    dp[0][1] = ((A[0] & con) ? 0 : 1);
    dp[0][2] = 1;

    for (int i = 1; i < N; i++) {
        // to gym
        if (A[i] & gym) {
            dp[i][0] = min(dp[i-1][1], dp[i-1][2]);
        }
        else {
            dp[i][0] = mymin(dp[i-1][0], dp[i-1][1], dp[i-1][2]) + 1;
        }

        // to con
        if (A[i] & con) {
            dp[i][1] = min(dp[i-1][0], dp[i-1][2]);
        }
        else {
            dp[i][1] = mymin(dp[i-1][0], dp[i-1][1], dp[i-1][2]) + 1;
        }

        // to rest
        dp[i][2] = mymin(dp[i-1][0], dp[i-1][1], dp[i-1][2]) + 1;
    }

    // for (int i = 0; i < N; i++) {
    //     printf("%d %d %d\n", dp[i][0], dp[i][1], dp[i][2]);
    // }

    printf("%d\n", mymin(dp[N-1][0], dp[N-1][1], dp[N-1][2]));

    return 0;
}
